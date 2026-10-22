//! C ABI adapter for the safe cf preprocessor. All C-visible storage uses bmem.
use super::cf_tokenizer::{cf_lexer, cf_lexer_free, cf_lexer_init, cf_lexer_lex, cf_token};
use super::darray::{bfree, bmalloc, darray, darray_free, darray_push_back_array};
use super::lexer::{error_data, error_data_add, strref};
use crate::cf_preprocessor::{self as safe, Definition, Preprocessor, Token};
use crate::cf_tokenizer::CfTokenType;
use core::ffi::{CStr, c_char, c_int};
use core::{ptr, slice};

/// `struct cf_def`, including the two owning token arrays.
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct cf_def {
    pub name: cf_token,
    pub params: darray,
    pub tokens: darray,
    pub r#macro: bool,
}
/// Public preprocessor layout from the unchanged cf-lexer.h.
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct cf_preprocessor {
    pub lex: *mut cf_lexer,
    pub ed: *mut error_data,
    pub defines: darray,
    pub sys_include_dirs: darray,
    pub dependencies: darray,
    pub tokens: darray,
    pub ignore_state: bool,
}
fn empty() -> darray {
    darray {
        array: ptr::null_mut(),
        num: 0,
        capacity: 0,
    }
}

// Every unsafe helper below is confined to initialized, caller-owned C layouts.
// Tokens retain pointers to original lexer buffers; provenance IDs index this
// temporary arena, never addresses manufactured by the safe core.
struct Adapter {
    root: *const cf_lexer,
    arena: Vec<cf_token>,
    // Lexer addresses must survive Vec growth while preprocessing nested includes.
    #[allow(clippy::vec_box)]
    included: Vec<Box<cf_lexer>>,
    completed: Vec<usize>,
}
impl Adapter {
    unsafe fn token(&mut self, t: cf_token) -> Token {
        let origin = self.arena.len();
        self.arena.push(t);
        let kind = match t.r#type {
            1 => CfTokenType::Name,
            2 => CfTokenType::Num,
            3 => CfTokenType::SpaceTab,
            4 => CfTokenType::Newline,
            5 => CfTokenType::String,
            6 => CfTokenType::Other,
            _ => CfTokenType::None,
        };
        // SAFETY: C strrefs are readable for len; a zero-length NULL is valid.
        let text = if t.str.len == 0 {
            Vec::new()
        } else {
            unsafe { slice::from_raw_parts(t.str.array.cast(), t.str.len) }.to_vec()
        };
        let tail = text.clone();
        Token {
            kind,
            text,
            tail,
            origin,
            filler: false,
        }
    }
    unsafe fn tokens(&mut self, a: &darray) -> Vec<Token> {
        (0..a.num)
            .map(|i| {
                // SAFETY: initialized array has num token entries.
                unsafe { self.token(*a.array.cast::<cf_token>().add(i)) }
            })
            .collect()
    }
    unsafe fn source_tokens(&mut self, a: &darray) -> Vec<Token> {
        // SAFETY: source tokens come from a real lexer and point into its
        // NUL-terminated reformatted allocation. Hand-built definition
        // strrefs use tokens(), which reads exactly their counted lengths.
        let mut tokens = unsafe { self.tokens(a) };
        for t in &mut tokens {
            if t.kind == CfTokenType::String {
                let raw = self.arena[t.origin];
                t.tail = unsafe { CStr::from_ptr(raw.str.array) }.to_bytes().to_vec();
            }
        }
        tokens
    }
    unsafe fn definitions(&mut self, a: &darray) -> Vec<Definition> {
        (0..a.num)
            .map(|i| {
                // SAFETY: initialized array has num definitions.
                let d = unsafe { &*a.array.cast::<cf_def>().add(i) };
                // SAFETY: every definition owns valid token arrays.
                unsafe {
                    Definition {
                        name: self.token(d.name),
                        params: self.tokens(&d.params),
                        tokens: self.tokens(&d.tokens),
                        macro_call: d.r#macro,
                    }
                }
            })
            .collect()
    }
    fn raw(&self, t: &Token) -> cf_token {
        if t.origin == usize::MAX {
            let space = strref {
                array: c" ".as_ptr(),
                len: 1,
            };
            return cf_token {
                lex: self.root,
                str: space,
                unmerged_str: space,
                r#type: 3,
            };
        }
        if !t.filler {
            return self.arena[t.origin];
        }
        if t.kind == CfTokenType::None {
            return cf_token {
                lex: ptr::null(),
                str: strref {
                    array: ptr::null(),
                    len: 0,
                },
                unmerged_str: strref {
                    array: ptr::null(),
                    len: 0,
                },
                r#type: 0,
            };
        }
        let base = self.arena[t.origin];
        cf_token {
            str: strref {
                array: c" ".as_ptr(),
                len: 1,
            },
            r#type: 3,
            ..base
        }
    }
    unsafe fn array(&self, tokens: &[Token]) -> darray {
        let mut dst = empty();
        for t in tokens {
            // SAFETY: dst is initialized; raw token is copied into bmem storage.
            unsafe {
                push(&mut dst, self.raw(t));
            }
        }
        dst
    }
    unsafe fn load(
        &mut self,
        t: &Token,
        root: *mut cf_lexer,
        existing: &darray,
    ) -> Option<Vec<Token>> {
        // C resolves all nested paths against pp.lex, the original root.
        // SAFETY: root is the live root lexer and its file is optional.
        let file = if root.is_null() || unsafe { (*root).file }.is_null() {
            &[][..]
        } else {
            unsafe { CStr::from_ptr((*root).file) }.to_bytes()
        };
        let mut path = safe::include_path(file, &t.text);
        for i in 0..existing.num {
            // SAFETY: existing holds initialized dependency lexers.
            let l = unsafe { &*existing.array.cast::<cf_lexer>().add(i) };
            if unsafe { CStr::from_ptr(l.file) }.to_bytes() == path {
                return Some(unsafe { self.source_tokens(&l.tokens) });
            }
        }
        if let Some(i) = self
            .included
            .iter()
            .position(|l| unsafe { CStr::from_ptr(l.file) }.to_bytes() == path)
        {
            let l = &*self.included[i] as *const cf_lexer;
            return Some(unsafe { self.source_tokens(&(*l).tokens) });
        }
        path.push(0);
        // SAFETY: path is NUL-terminated. File reading delegates to the
        // platform port (same UTF-8/BOM conversion as the C implementation).
        let path_c = unsafe { CStr::from_ptr(path.as_ptr().cast()) };
        let mut file = crate::platform_file::fopen(path_c, c"rb")?;
        let mut data = crate::platform_file::fread_utf8(&mut file).unwrap_or_default();
        data.push(0);
        let mut l = Box::new(unsafe { core::mem::zeroed::<cf_lexer>() });
        // SAFETY: the stable boxed lexer owns the copied input and path.
        unsafe {
            cf_lexer_init(&mut *l);
            cf_lexer_lex(&mut *l, data.as_ptr().cast(), path.as_ptr().cast());
            if l.file.is_null() {
                l.file = bmalloc(path.len()).cast();
                ptr::copy_nonoverlapping(path.as_ptr().cast(), l.file, path.len());
            }
        }
        let tokens = unsafe { self.source_tokens(&l.tokens) };
        self.included.push(l);
        Some(tokens)
    }
    unsafe fn diagnostics(&self, pp: *mut cf_preprocessor, diagnostics: &[safe::Diagnostic]) {
        for d in diagnostics {
            let t = self.arena[d.token.origin];
            // SAFETY: diagnostics have live source provenance under the C contract.
            unsafe {
                let root = (*pp).lex;
                let lex = if t.lex.is_null() {
                    root
                } else {
                    t.lex.cast_mut()
                };
                let text = CStr::from_ptr((*lex).base_lexer.text).to_bytes();
                let pos = t
                    .unmerged_str
                    .array
                    .addr()
                    .checked_sub((*lex).base_lexer.text.addr())
                    .unwrap_or(text.len())
                    .min(text.len());
                let (row, col) = crate::lexer::str_offset_in(text, pos);
                let mut msg = d.message.clone();
                msg.push(0);
                error_data_add(
                    (*pp).ed,
                    (*lex).file,
                    row,
                    col,
                    msg.as_ptr().cast(),
                    d.level,
                );
            }
        }
    }
    unsafe fn store(&self, pp: *mut cf_preprocessor, p: &Preprocessor) {
        // SAFETY: replace owning definition arrays only after snapshotting all
        // token metadata; token strings belong to lexers, not those arrays.
        unsafe {
            for i in 0..(*pp).defines.num {
                let d = &mut *(*pp).defines.array.cast::<cf_def>().add(i);
                darray_free(&mut d.params);
                darray_free(&mut d.tokens);
            }
            (*pp).defines.num = 0;
            for d in &p.definitions {
                push(
                    &mut (*pp).defines,
                    cf_def {
                        name: self.raw(&d.name),
                        params: self.array(&d.params),
                        tokens: self.array(&d.tokens),
                        r#macro: d.macro_call,
                    },
                );
            }
            for t in &p.tokens {
                push(&mut (*pp).tokens, self.raw(t));
            }
            (*pp).ignore_state = p.ignore;
        }
    }
}
struct HostIncludes<'a> {
    adapter: &'a mut Adapter,
    pp: *mut cf_preprocessor,
}
impl safe::Includes for HostIncludes<'_> {
    fn load(&mut self, token: &Token) -> Option<Vec<Token>> {
        // SAFETY: host context is alive for the synchronous core call.
        unsafe {
            self.adapter
                .load(token, (*self.pp).lex, &(*self.pp).dependencies)
        }
    }
    fn complete(&mut self, token: &Token) {
        // SAFETY: all dependency lexers are stable boxes until the call completes.
        unsafe {
            let adapter = &mut self.adapter;
            let root = (*self.pp).lex;
            let file = if (*root).file.is_null() {
                &[][..]
            } else {
                CStr::from_ptr((*root).file).to_bytes()
            };
            let path = safe::include_path(file, &token.text);
            if let Some(i) = adapter
                .included
                .iter()
                .position(|l| CStr::from_ptr(l.file).to_bytes() == path)
                && !adapter.completed.contains(&i)
            {
                adapter.completed.push(i);
            }
        }
    }
}
unsafe fn push<T>(a: &mut darray, value: T) {
    // SAFETY: array stores plain C values of T, with initialized ownership.
    unsafe {
        darray_push_back_array(size_of::<T>(), a, ptr::from_ref(&value).cast(), 1);
    }
}
unsafe fn free_definitions(a: &mut darray) {
    for i in 0..a.num {
        // SAFETY: every entry is an initialized definition owning these arrays.
        unsafe {
            let d = &mut *a.array.cast::<cf_def>().add(i);
            darray_free(&mut d.params);
            darray_free(&mut d.tokens);
        }
    }
    unsafe {
        darray_free(a);
    }
}
/// # Safety
/// pp must be writable for one preprocessor.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cf_preprocessor_init(pp: *mut cf_preprocessor) {
    // SAFETY: caller provides writable storage; every field is initialized.
    unsafe {
        pp.write(cf_preprocessor {
            lex: ptr::null_mut(),
            ed: ptr::null_mut(),
            defines: empty(),
            sys_include_dirs: empty(),
            dependencies: empty(),
            tokens: empty(),
            ignore_state: false,
        });
    }
}
/// # Safety
/// pp is initialized and owns bmem-allocated arrays and dependency lexers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cf_preprocessor_free(pp: *mut cf_preprocessor) {
    // SAFETY: the caller transfers initialized C-owned buffers for destruction.
    unsafe {
        free_definitions(&mut (*pp).defines);
        for i in 0..(*pp).sys_include_dirs.num {
            bfree(
                *(*pp)
                    .sys_include_dirs
                    .array
                    .cast::<*mut core::ffi::c_void>()
                    .add(i),
            );
        }
        for i in 0..(*pp).dependencies.num {
            cf_lexer_free((*pp).dependencies.array.cast::<cf_lexer>().add(i));
        }
        darray_free(&mut (*pp).sys_include_dirs);
        darray_free(&mut (*pp).dependencies);
        darray_free(&mut (*pp).tokens);
        (*pp).lex = ptr::null_mut();
        (*pp).ed = ptr::null_mut();
        (*pp).ignore_state = false;
    }
}
/// # Safety
/// pp, lex and ed are live initialized C objects; lex outlives pp's token use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cf_preprocess(
    pp: *mut cf_preprocessor,
    lex: *mut cf_lexer,
    ed: *mut error_data,
) -> bool {
    // SAFETY: caller supplies live initialized C objects and terminated strings.
    unsafe {
        if (*lex).tokens.array.is_null() {
            return false;
        }
        (*pp).lex = lex;
        (*pp).ed = ed;
        let mut adapter = Adapter {
            root: (*pp).lex,
            arena: Vec::new(),
            included: Vec::new(),
            completed: Vec::new(),
        };
        let mut p = Preprocessor {
            definitions: adapter.definitions(&(*pp).defines),
            ignore: (*pp).ignore_state,
            ..Preprocessor::default()
        };
        let tokens = adapter.source_tokens(&(*lex).tokens);
        p.process(
            &tokens,
            &mut HostIncludes {
                adapter: &mut adapter,
                pp,
            },
        );
        adapter.diagnostics(pp, &p.diagnostics);
        // Move dependency lexers to the public array, then rebind every raw
        // token's lex pointer after the array's final allocation.
        let old = (*pp).dependencies.array.cast::<cf_lexer>();
        let old_count = (*pp).dependencies.num;
        let addresses: Vec<_> = adapter
            .included
            .iter()
            .map(|l| ptr::from_ref(&**l))
            .collect();
        let mut included: Vec<_> = adapter.included.drain(..).map(Some).collect();
        let order = adapter.completed.clone();
        for &index in &order {
            push(
                &mut (*pp).dependencies,
                *included[index].take().expect("completed dependency"),
            );
        }
        let new = (*pp).dependencies.array.cast::<cf_lexer>();
        for t in &mut adapter.arena {
            if let Some(i) = addresses.iter().position(|&l| l == t.lex) {
                t.lex = new.add(
                    old_count
                        + order
                            .iter()
                            .position(|&j| j == i)
                            .expect("dependency completed"),
                );
            } else if let Some(i) =
                (0..old_count).find(|&i| old.wrapping_add(i) == t.lex.cast_mut())
            {
                t.lex = new.add(i);
            }
        }
        for i in 0..(*pp).dependencies.num {
            let l = new.add(i);
            for j in 0..(*l).tokens.num {
                (*(*l).tokens.array.cast::<cf_token>().add(j)).lex = l;
            }
        }
        // Existing output tokens may also reference relocated dependencies.
        for i in 0..(*pp).tokens.num {
            let t = &mut *(*pp).tokens.array.cast::<cf_token>().add(i);
            if let Some(j) = (0..old_count).find(|&j| old.wrapping_add(j) == t.lex.cast_mut()) {
                t.lex = new.add(j);
            }
        }
        adapter.store(pp, &p);
        !(*lex).unexpected_eof
    }
}
/// # Safety
/// pp and def are initialized; ownership of def's token arrays transfers to pp.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cf_preprocessor_add_def(pp: *mut cf_preprocessor, def: *mut cf_def) {
    // SAFETY: the caller supplies initialized arrays; snapshot before moving ownership.
    unsafe {
        let mut a = Adapter {
            root: (*pp).lex,
            arena: Vec::new(),
            included: Vec::new(),
            completed: Vec::new(),
        };
        let mut p = Preprocessor {
            definitions: a.definitions(&(*pp).defines),
            ignore: (*pp).ignore_state,
            ..Preprocessor::default()
        };
        let d = Definition {
            name: a.token((*def).name),
            params: a.tokens(&(*def).params),
            tokens: a.tokens(&(*def).tokens),
            macro_call: (*def).r#macro,
        };
        p.add(d);
        a.diagnostics(pp, &p.diagnostics);
        a.store(pp, &p);
        // C consumes the supplied arrays without clearing the caller's struct.
        bfree((*def).params.array);
        bfree((*def).tokens.array);
    }
}
/// # Safety
/// pp is initialized and name is a NUL-terminated string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cf_preprocessor_remove_def(pp: *mut cf_preprocessor, name: *const c_char) {
    // SAFETY: the caller owns initialized pp and a NUL-terminated name.
    unsafe {
        let mut a = Adapter {
            root: (*pp).lex,
            arena: Vec::new(),
            included: Vec::new(),
            completed: Vec::new(),
        };
        let mut p = Preprocessor {
            definitions: a.definitions(&(*pp).defines),
            ignore: (*pp).ignore_state,
            ..Preprocessor::default()
        };
        p.remove(CStr::from_ptr(name).to_bytes());
        a.store(pp, &p);
    }
}
/// Internal C helper used by shader parsing.
/// # Safety
/// pp is initialized; name is a valid strref and value is writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cf_preprocess_get_def_value(
    pp: *mut cf_preprocessor,
    name: *const strref,
    value: *mut c_int,
) -> bool {
    // SAFETY: caller supplies live initialized C objects and terminated strings.
    unsafe {
        let mut a = Adapter {
            root: (*pp).lex,
            arena: Vec::new(),
            included: Vec::new(),
            completed: Vec::new(),
        };
        let p = Preprocessor {
            definitions: a.definitions(&(*pp).defines),
            ..Preprocessor::default()
        };
        let mut token = Token {
            kind: CfTokenType::Name,
            text: slice::from_raw_parts((*name).array.cast(), (*name).len).to_vec(),
            tail: Vec::new(),
            origin: 0,
            filler: false,
        };
        token.kind = CfTokenType::Name;
        if let Some(v) = p.value(&token) {
            *value = v;
            true
        } else {
            false
        }
    }
}
