//! Original-C parity for defined, terminated streams. Recursive macro cycles,
//! empty include files, unterminated macro invocations, and diagnostics in included files are excluded: the C
//! implementation recurses without a bound, dereferences a NULL token stream,
//! overruns the token terminator, or retains a stack/moved dependency lexer.
//! Rust handles those cases in dedicated non-oracle tests (see #1499).
use core::ffi::{CStr, c_char};
use core::mem::{align_of, offset_of, size_of};
use obs_util::ffi::cf_preprocessor::{self as rs, cf_def, cf_preprocessor};
use obs_util::ffi::cf_tokenizer::{cf_lexer, cf_token};
use obs_util::ffi::darray::{bfree, darray, darray_free};
use obs_util::ffi::lexer::{error_data, error_item};
use proptest::prelude::*;
use std::ffi::CString;
unsafe extern "C" {
    fn oracle_cf_preprocessor_init(pp: *mut cf_preprocessor);
    fn oracle_cf_preprocessor_free(pp: *mut cf_preprocessor);
    fn oracle_cf_preprocess(
        pp: *mut cf_preprocessor,
        lex: *mut cf_lexer,
        ed: *mut error_data,
    ) -> bool;
    fn oracle_cf_preprocessor_add_def(pp: *mut cf_preprocessor, def: *mut cf_def);
    fn oracle_cf_preprocessor_remove_def(pp: *mut cf_preprocessor, name: *const c_char);
    fn oracle_cf_lexer_init(lex: *mut cf_lexer);
    fn oracle_cf_lexer_lex(lex: *mut cf_lexer, text: *const c_char, file: *const c_char) -> bool;
    fn oracle_cf_lexer_free(lex: *mut cf_lexer);
    fn oracle_cf_def_layout(i: usize) -> usize;
}
#[test]
fn cf_def_layout() {
    for (i, n) in [
        size_of::<cf_def>(),
        align_of::<cf_def>(),
        offset_of!(cf_def, name),
        offset_of!(cf_def, params),
        offset_of!(cf_def, tokens),
        offset_of!(cf_def, r#macro),
    ]
    .into_iter()
    .enumerate()
    {
        // SAFETY: i indexes the fixed C layout table measured from cf-lexer.h.
        assert_eq!(n, unsafe { oracle_cf_def_layout(i) });
    }
}
struct Run {
    pp: Box<cf_preprocessor>,
    lex: Box<cf_lexer>,
    ed: Box<error_data>,
    c: bool,
    ok: bool,
}
impl Run {
    fn new(bytes: &[u8], c: bool, file: &CStr) -> Self {
        // SAFETY: zero initializes the C layouts; every owned allocation is
        // freed in Drop, and boxed lexers retain stable addresses.
        unsafe {
            let mut r = Self {
                pp: Box::new(core::mem::zeroed()),
                lex: Box::new(core::mem::zeroed()),
                ed: Box::new(core::mem::zeroed()),
                c,
                ok: false,
            };
            let text = CString::new(bytes).unwrap();
            if c {
                oracle_cf_preprocessor_init(&mut *r.pp);
                oracle_cf_lexer_init(&mut *r.lex);
                oracle_cf_lexer_lex(&mut *r.lex, text.as_ptr(), file.as_ptr());
                r.ok = oracle_cf_preprocess(&mut *r.pp, &mut *r.lex, &mut *r.ed);
            } else {
                rs::cf_preprocessor_init(&mut *r.pp);
                obs_util::ffi::cf_tokenizer::cf_lexer_init(&mut *r.lex);
                obs_util::ffi::cf_tokenizer::cf_lexer_lex(
                    &mut *r.lex,
                    text.as_ptr(),
                    file.as_ptr(),
                );
                r.ok = rs::cf_preprocess(&mut *r.pp, &mut *r.lex, &mut *r.ed);
            }
            r
        }
    }
    fn snapshot(&self) -> Snapshot {
        // SAFETY: arrays contain initialized values from the preprocess call.
        unsafe {
            let defs = (0..self.pp.defines.num)
                .map(|i| {
                    let d = &*self.pp.defines.array.cast::<cf_def>().add(i);
                    (
                        token(&d.name),
                        tokens(&d.params),
                        tokens(&d.tokens),
                        d.r#macro,
                    )
                })
                .collect();
            let errors = (0..self.ed.errors.num)
                .map(|i| {
                    let e = &*self.ed.errors.array.cast::<error_item>().add(i);
                    (
                        CStr::from_ptr(e.error).to_bytes().to_vec(),
                        if e.file.is_null() {
                            None
                        } else {
                            Some(CStr::from_ptr(e.file).to_bytes().to_vec())
                        },
                        e.row,
                        e.column,
                        e.level,
                    )
                })
                .collect();
            Snapshot {
                ok: self.ok,
                defs,
                tokens: tokens(&self.pp.tokens),
                errors,
                deps: (0..self.pp.dependencies.num)
                    .map(|i| {
                        CStr::from_ptr((*self.pp.dependencies.array.cast::<cf_lexer>().add(i)).file)
                            .to_bytes()
                            .to_vec()
                    })
                    .collect(),
                ignore: self.pp.ignore_state,
            }
        }
    }
}
impl Drop for Run {
    fn drop(&mut self) {
        // SAFETY: matching destructors release all owned buffers exactly once.
        unsafe {
            if self.c {
                oracle_cf_preprocessor_free(&mut *self.pp);
                oracle_cf_lexer_free(&mut *self.lex);
            } else {
                rs::cf_preprocessor_free(&mut *self.pp);
                obs_util::ffi::cf_tokenizer::cf_lexer_free(&mut *self.lex);
            }
            for i in 0..self.ed.errors.num {
                bfree(
                    (*self.ed.errors.array.cast::<error_item>().add(i))
                        .error
                        .cast(),
                );
            }
            darray_free(&mut self.ed.errors);
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
struct Tok {
    kind: i32,
    text: Vec<u8>,
    unmerged: Vec<u8>,
    null_lex: bool,
}
unsafe fn token(t: &cf_token) -> Tok {
    // SAFETY: valid token strrefs; null is accepted only for zero lengths.
    unsafe {
        let bytes = |p: *const c_char, n| {
            if n == 0 {
                Vec::new()
            } else {
                core::slice::from_raw_parts(p.cast::<u8>(), n).to_vec()
            }
        };
        Tok {
            kind: t.r#type,
            text: bytes(t.str.array, t.str.len),
            unmerged: bytes(t.unmerged_str.array, t.unmerged_str.len),
            null_lex: t.lex.is_null(),
        }
    }
}
unsafe fn tokens(a: &darray) -> Vec<Tok> {
    // SAFETY: a owns num initialized tokens, including entries after NONE.
    (0..a.num)
        .map(|i| unsafe { token(&*a.array.cast::<cf_token>().add(i)) })
        .collect()
}
type Def = (Tok, Vec<Tok>, Vec<Tok>, bool);
type Error = (Vec<u8>, Option<Vec<u8>>, u32, u32, i32);
#[derive(Debug, PartialEq, Eq)]
struct Snapshot {
    ok: bool,
    defs: Vec<Def>,
    tokens: Vec<Tok>,
    errors: Vec<Error>,
    deps: Vec<Vec<u8>>,
    ignore: bool,
}
fn compare(bytes: &[u8]) {
    let a = Run::new(bytes, false, c"t.c");
    let b = Run::new(bytes, true, c"t.c");
    assert_eq!(
        a.snapshot(),
        b.snapshot(),
        "source {:?}",
        String::from_utf8_lossy(bytes)
    );
}
#[test]
fn cf_preprocessor_characterization_parity() {
    for src in [
        "#define FOO 3\nint a = FOO;",
        "#define ADD(a,b) a + b\nADD(1,2)",
        "#define A B\n#define B 4\nA",
        "#ifdef A\n#ifdef B\nx\n#endif\n#else\ny\n#endif",
        "#if 1\nx\n#elif 0\ny\n#endif",
        "#define\nx",
        "#include \"missing1499.inc\"\nx",
        "#define ZERO() 2\nZERO()",
        "#define FOO 2\n#undef FOO\nFOO",
        "#define A 0x10\n#if A == 16\ny\n#endif",
    ] {
        compare(src.as_bytes());
    }
}
proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]
    #[test]
    fn cf_preprocessor_arbitrary_text(bytes in prop::collection::vec(1u8..=255,1..128)) {
        // The tokenizer's existing exclusions: trailing line splice in a comment
        // or literal can read beyond NUL in C. Exclude exactly such suffixes.
        prop_assume!(!bytes.ends_with(b"\\\n") && !bytes.ends_with(b"\\\r") && !bytes.ends_with(b"\\\r\n") && !bytes.ends_with(b"\\\n\r"));
        // Arbitrary non-directive tokens cannot create recursive definitions.
        let mut input=b"ordinary ".to_vec();input.extend(bytes);
        compare(&input);
    }
    #[test]
    fn cf_preprocessor_generated_directives(a in 0i32..10000,b in 0i32..10000, cmp in prop::sample::select(vec!["==","!=",">=","<=","<",">"]), keep in any::<bool>()) {
        compare(format!("#define A {a}\n#define B {b}\n#if A {cmp} B\nA\n#else\nB\n#endif\n#define M(a,b) (a+b)\nM(A,B)\n{}\nA",if keep{"#undef B"}else{"#undef A"}).as_bytes());
    }
}

#[test]
fn cf_preprocessor_public_definition_lifecycle_parity() {
    let mut a = Run::new(b"x BAR y", false, c"t.c");
    let mut b = Run::new(b"x BAR y", true, c"t.c");
    for (r, c) in [(&mut a, false), (&mut b, true)] {
        // SAFETY: hand-built definitions own bmem arrays; the add API consumes
        // them. All token strings are static, valid NUL-terminated storage.
        unsafe {
            let mut d: cf_def = core::mem::zeroed();
            d.name.str = obs_util::ffi::lexer::strref {
                array: c"BAR".as_ptr(),
                len: 3,
            };
            d.name.r#type = 1;
            let mut t: cf_token = core::mem::zeroed();
            t.str = obs_util::ffi::lexer::strref {
                array: c"7".as_ptr(),
                len: 1,
            };
            t.r#type = 2;
            obs_util::ffi::darray::darray_push_back_array(
                size_of::<cf_token>(),
                &mut d.tokens,
                core::ptr::from_ref(&t).cast(),
                1,
            );
            t = core::mem::zeroed();
            obs_util::ffi::darray::darray_push_back_array(
                size_of::<cf_token>(),
                &mut d.tokens,
                core::ptr::from_ref(&t).cast(),
                1,
            );
            if c {
                oracle_cf_preprocessor_add_def(&mut *r.pp, &mut d);
                oracle_cf_preprocess(&mut *r.pp, &mut *r.lex, &mut *r.ed);
            } else {
                rs::cf_preprocessor_add_def(&mut *r.pp, &mut d);
                rs::cf_preprocess(&mut *r.pp, &mut *r.lex, &mut *r.ed);
            }
        }
    }
    assert_eq!(a.snapshot(), b.snapshot());
    for name in [c"NOPE", c"BAR"] {
        // SAFETY: initialized preprocessors and static C strings.
        unsafe {
            rs::cf_preprocessor_remove_def(&mut *a.pp, name.as_ptr());
            oracle_cf_preprocessor_remove_def(&mut *b.pp, name.as_ptr());
        }
        assert_eq!(a.snapshot(), b.snapshot());
    }
}
#[test]
fn cf_preprocessor_file_includes_parity() {
    let dir = crate::common::ScratchDir::new("cf-preprocessor-parity");
    std::fs::write(dir.join("a.inc"), b"#define A_VAL 1\nfrom_a\n").unwrap();
    std::fs::write(dir.join("nest.inc"), b"#include \"b.inc\"\n").unwrap();
    std::fs::write(dir.join("b.inc"), b"#define B_VAL 2\nfrom_b\n").unwrap();
    let file = CString::new(dir.join("main.c").to_str().unwrap()).unwrap();
    for text in [
        b"#include \"a.inc\"\nA_VAL".as_slice(),
        b"#include \"nest.inc\"\nB_VAL",
        b"#include \"a.inc\"\n#include \"a.inc\"\nA_VAL",
    ] {
        let a = Run::new(text, false, &file);
        let b = Run::new(text, true, &file);
        assert_eq!(a.snapshot(), b.snapshot());
        // Every Rust dependency token must point to its current live lexer,
        // including after the owning dependency darray grows.
        unsafe {
            for i in 0..a.pp.dependencies.num {
                let l = a.pp.dependencies.array.cast::<cf_lexer>().add(i);
                for j in 0..(*l).tokens.num {
                    assert_eq!((*(*l).tokens.array.cast::<cf_token>().add(j)).lex, l);
                }
            }
        }
    }
}

#[test]
fn cf_preprocessor_unterminated_and_numeric_directives() {
    for literal in [
        "0",
        "00",
        "08",
        "0x",
        "0X",
        "0xz",
        "123suffix",
        "1.5",
        "0x10",
        "012",
        "0b1",
        "0009",
    ] {
        compare(format!("#if {literal}\ny\n#else\nn\n#endif").as_bytes());
        compare(format!("#define V {literal}\n#if V\ny\n#endif").as_bytes());
    }
}

#[test]
fn cf_preprocessor_defined_unterminated_directives() {
    for text in [
        "#",
        "#define",
        "#define X",
        "#define M(",
        "#define M(a",
        "#if",
        "#if 1",
        "#if 1 ==",
        "#ifdef",
        "#undef",
        "#include",
        "#pragma",
        "#else",
        "#elif",
        "#define M(a) a\nM;",
        "#define M(a) a\nM(,)",
        "#define M(a) a\nM()",
    ] {
        compare(text.as_bytes());
    }
}
#[test]
fn cf_preprocessor_empty_include_and_unterminated_macro_are_bounded() {
    let dir = crate::common::ScratchDir::new("cf-preprocessor-empty");
    std::fs::write(dir.join("empty.inc"), b"").unwrap();
    let file = CString::new(dir.join("main.c").to_str().unwrap()).unwrap();
    let r = Run::new(b"#include \"empty.inc\"\nx", false, &file);
    assert!(r.ok);
    assert_eq!(r.pp.dependencies.num, 1);
    assert_eq!(r.ed.errors.num, 0);
    std::fs::write(dir.join("bad.inc"), b"#endif\n").unwrap();
    let r = Run::new(b"#include \"bad.inc\"\nx", false, &file);
    let snapshot = r.snapshot();
    assert_eq!(snapshot.errors.len(), 1);
    assert_eq!(
        snapshot.errors[0].0,
        b"#endif outside of #if/#ifdef/#ifndef block"
    );
    assert_eq!(
        snapshot.errors[0].1.as_deref(),
        Some(dir.join("bad.inc").to_str().unwrap().as_bytes())
    );
    assert_eq!((snapshot.errors[0].2, snapshot.errors[0].3), (1, 2));
    for text in [b"#define M(a) a\nM(".as_slice(), b"#define M(a) a\nM(1"] {
        let r = Run::new(text, false, c"t.c");
        assert!(r.ok);
        assert_eq!(r.ed.errors.num, 2);
        unsafe {
            assert_eq!(
                (*r.pp
                    .tokens
                    .array
                    .cast::<cf_token>()
                    .add(r.pp.tokens.num - 1))
                .r#type,
                0
            );
        }
    }
}

#[test]
fn cf_preprocessor_empty_lexer_matches_c() {
    compare(b"");
}
