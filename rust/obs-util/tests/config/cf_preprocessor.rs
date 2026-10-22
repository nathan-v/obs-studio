//! Tier 1 mirrors of test_cf_preprocessor.c (C quirks intentionally preserved).
use obs_util::cf_preprocessor::{Preprocessor, Token};
use obs_util::cf_tokenizer::{CfTokenType, cf_lex};

fn lex(input: &[u8], arena: &mut Vec<(Vec<u8>, Vec<u8>, usize)>, file: &[u8]) -> Vec<Token> {
    let l = cf_lex(input);
    l.tokens
        .iter()
        .map(|t| {
            let origin = arena.len();
            arena.push((input.to_vec(), file.to_vec(), t.unmerged_start));
            Token {
                kind: t.kind,
                text: l.reformatted[t.str_start..t.str_start + t.str_len].to_vec(),
                origin,
                filler: false,
            }
        })
        .collect()
}
fn check(
    src: &str,
    file: &str,
    output: &str,
    count: usize,
    definitions: usize,
    deps: usize,
    errors: usize,
    message: Option<&str>,
) {
    let mut arena = Vec::new();
    let input = lex(src.as_bytes(), &mut arena, file.as_bytes());
    let mut loaded: std::collections::BTreeMap<Vec<u8>, Vec<Token>> =
        std::collections::BTreeMap::new();
    let mut p = Preprocessor::default();
    p.process(&input, &mut |t| {
        let path = &t.text[1..t.text.len() - 1];
        if let Some(tokens) = loaded.get(path) {
            return Some(tokens.clone());
        }
        let data: &[u8] = match path {
            b"a.inc" => b"#define A_VAL 1\nfrom_a\n",
            b"nest.inc" => b"#include \"b.inc\"\n",
            b"b.inc" => b"#define B_VAL 2\nfrom_b\n",
            _ => return None,
        };
        let tokens = lex(data, &mut arena, path);
        loaded.insert(path.to_vec(), tokens.clone());
        Some(tokens)
    });
    let visible: Vec<_> = p
        .tokens
        .iter()
        .take_while(|t| t.kind != CfTokenType::None)
        .collect();
    assert_eq!(
        visible
            .iter()
            .flat_map(|t| t.text.iter().copied())
            .collect::<Vec<_>>(),
        output.as_bytes()
    );
    assert_eq!(visible.len(), count);
    assert_eq!(p.definitions.len(), definitions);
    assert_eq!(loaded.len(), deps);
    assert_eq!(p.diagnostics.len(), errors);
    let mut rendered = String::new();
    for d in &p.diagnostics {
        let (text, file, pos) = &arena[d.token.origin];
        let (row, col) = obs_util::lexer::str_offset_in(text, *pos);
        rendered.push_str(&format!(
            "{} ({row}, {col}): {}\n",
            String::from_utf8_lossy(file),
            String::from_utf8_lossy(&d.message)
        ));
    }
    assert_eq!(rendered, message.unwrap_or(""));
}

#[test]
fn c_characterization_case_00() {
    check(
        "#include \"a.inc\"\nA_VAL x",
        "cf_pp_test/main.c",
        "\nfrom_a\n\n 1 x",
        8,
        1,
        1,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_01() {
    check(
        "#include \"nest.inc\"\nB_VAL",
        "cf_pp_test/main.c",
        "\nfrom_b\n\n\n 2",
        7,
        1,
        2,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_02() {
    check(
        "#include \"a.inc\"\n#include \"a.inc\"\nA_VAL",
        "cf_pp_test/main.c",
        "\nfrom_a\n\n\nfrom_a\n\n 1",
        10,
        2,
        1,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_03() {
    check(
        "#include \"nope.inc\"\nx",
        "cf_pp_test/main.c",
        "\nx",
        2,
        0,
        0,
        1,
        Some("cf_pp_test/main.c (1, 10): Could not open file '\"nope.inc\"\nx'\n"),
    );
}

#[test]
fn c_characterization_case_04() {
    check(
        "#include x\ny",
        "cf_pp_test/main.c",
        "\ny",
        2,
        0,
        0,
        1,
        Some("cf_pp_test/main.c (1, 10): Expected string\n"),
    );
}

#[test]
fn c_characterization_case_05() {
    check(
        "#include <s.inc>\nS_VAL",
        "cf_pp_test/main.c",
        "\nS_VAL",
        2,
        0,
        0,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_06() {
    check(
        "#include <s.inc>\nS_VAL",
        "cf_pp_test/main.c",
        "\nS_VAL",
        2,
        0,
        0,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_07() {
    check(
        "#define ADD(a, b) a + b\nADD(1, 2);",
        "t.c",
        "\n  1  +   2 ;",
        13,
        1,
        0,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_08() {
    check(
        "#define ADD(a, b) (a+b)\nADD(ADD(1,2), 3)",
        "t.c",
        "\n (  ( 1 + 2 ) +  3 )",
        21,
        1,
        0,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_09() {
    check(
        "#define ADD(a, b) a + b\nADD(1)",
        "t.c",
        "\n  1  + b",
        9,
        1,
        0,
        1,
        Some("t.c (2, 6): Mismatching number of macro parameters\n"),
    );
}

#[test]
fn c_characterization_case_10() {
    check(
        "#define ADD(a, b) a + b\nADD(1, 2, 3)",
        "t.c",
        "\n  1  +   2 ",
        12,
        1,
        0,
        1,
        Some("t.c (2, 12): Mismatching number of macro parameters\n"),
    );
}

#[test]
fn c_characterization_case_11() {
    check(
        "#define ADD(a, b) a + b\nADD;",
        "t.c",
        "\n a + b",
        7,
        1,
        0,
        1,
        Some("t.c (2, 4): Expected '('\n"),
    );
}

#[test]
fn c_characterization_case_12() {
    check(
        "#define ADD(a, b a + b\nADD(1,2)",
        "t.c",
        "\nADD(1,2)",
        7,
        0,
        0,
        1,
        Some("t.c (1, 18): Expected ',' or ')'\n"),
    );
}

#[test]
fn c_characterization_case_13() {
    check(
        "#define FOO 1\n#undef FOO\nFOO",
        "t.c",
        "\n\nFOO",
        3,
        0,
        0,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_14() {
    check("#undef NOPE\nx", "t.c", "\nx", 2, 0, 0, 0, None);
}

#[test]
fn c_characterization_case_15() {
    check(
        "#define FOO 1\n#define FOO 2\nFOO",
        "t.c",
        "\n\n 1",
        4,
        2,
        0,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_16() {
    check(
        "#ifndef FOO\nyes\n#else\nno\n#endif",
        "t.c",
        "\nyes\n",
        3,
        0,
        0,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_17() {
    check(
        "#define FOO\n#ifndef FOO\nyes\n#else\nno\n#endif",
        "t.c",
        "\n\nno\n",
        4,
        1,
        0,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_18() {
    check(
        "#ifdef A\n#ifdef B\nab\n#else\nanb\n#endif\n#else\nna\n#endif",
        "t.c",
        "\nna\n",
        3,
        0,
        0,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_19() {
    check(
        "#ifdef A\n#define X 1\n#endif\nX",
        "t.c",
        "\nX",
        2,
        0,
        0,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_20() {
    check(
        "#define A B\n#define B 3\nA",
        "t.c",
        "\n\n  3",
        5,
        2,
        0,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_21() {
    check(
        "#define M 1 \\\n + 2\nM",
        "t.c",
        "\n 1  + 2",
        7,
        1,
        0,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_22() {
    check(
        "x # define Y 1\nY",
        "t.c",
        "x # define Y 1\nY",
        11,
        0,
        0,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_23() {
    check(
        "#pragma once\nx",
        "t.c",
        " \nx",
        3,
        0,
        0,
        1,
        Some("t.c (1, 9): Unexpected token after preprocessor, expected newline\n"),
    );
}

#[test]
fn c_characterization_case_24() {
    check(
        "#ifdef A\n#else\n#else\n#endif",
        "t.c",
        "\n\n",
        2,
        0,
        0,
        1,
        Some("t.c (4, 2): #endif outside of #if/#ifdef/#ifndef block\n"),
    );
}

#[test]
fn c_characterization_case_25() {
    check(
        "#define\nx",
        "t.c",
        "\nx",
        2,
        0,
        0,
        1,
        Some("t.c (1, 8): Expected identifier\n"),
    );
}

#[test]
fn c_characterization_case_26() {
    check(
        "#define 3 4\nx",
        "t.c",
        "\nx",
        2,
        0,
        0,
        1,
        Some("t.c (1, 9): Expected identifier\n"),
    );
}

#[test]
fn c_characterization_case_27() {
    check("#if 1\nx\n#endif", "t.c", "\nx\n", 3, 0, 0, 0, None);
}

#[test]
fn c_characterization_case_28() {
    check(
        "#if 0\nx\n#else\ny\n#endif",
        "t.c",
        "\ny\n",
        3,
        0,
        0,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_29() {
    check(
        "#define V 3\n#if V\nx\n#endif",
        "t.c",
        "\n\nx\n",
        4,
        1,
        0,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_30() {
    check(
        "#define A B\n#define B 5\n#if A == 5\nx\n#endif",
        "t.c",
        "\n\n\nx\n",
        5,
        2,
        0,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_31() {
    check(
        "#define V 3\n#if V == 3\nx\n#else\ny\n#endif",
        "t.c",
        "\n\nx\n",
        4,
        1,
        0,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_32() {
    check(
        "#define V 3\n#if V != 3\nx\n#else\ny\n#endif",
        "t.c",
        "\n\ny\n",
        4,
        1,
        0,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_33() {
    check(
        "#if 2 >= 3\na\n#endif\n#if 2 <= 3\nb\n#endif",
        "t.c",
        "\n\nb\n",
        4,
        0,
        0,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_34() {
    check(
        "#if 2 < 3\na\n#endif\n#if 2 > 3\nb\n#endif",
        "t.c",
        "\na\n\n",
        4,
        0,
        0,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_35() {
    check(
        "#if 0x10 == 16\nx\n#endif",
        "t.c",
        "\nx\n",
        3,
        0,
        0,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_36() {
    check(
        "#ifdef X\n#if 1\na\n#endif\n#endif\nz",
        "t.c",
        "\nz",
        2,
        0,
        0,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_37() {
    check(
        "#if NOPE\nx\n#endif",
        "t.c",
        "\nx\n",
        3,
        0,
        0,
        2,
        Some(
            "t.c (1, 5): Expected identifier or number\nt.c (3, 2): #endif outside of #if/#ifdef/#ifndef block\n",
        ),
    );
}

#[test]
fn c_characterization_case_38() {
    check(
        "#if -1\nx\n#endif",
        "t.c",
        "\nx\n",
        3,
        0,
        0,
        2,
        Some(
            "t.c (1, 5): Expected identifier or number\nt.c (3, 2): #endif outside of #if/#ifdef/#ifndef block\n",
        ),
    );
}

#[test]
fn c_characterization_case_39() {
    check(
        "#define V abc\n#if V\nx\n#endif",
        "t.c",
        "\n\nx\n",
        4,
        1,
        0,
        2,
        Some(
            "t.c (2, 5): Expected identifier or number\nt.c (4, 2): #endif outside of #if/#ifdef/#ifndef block\n",
        ),
    );
}

#[test]
fn c_characterization_case_40() {
    check(
        "#define M(a) a\n#if M\nx\n#endif",
        "t.c",
        "\n\nx\n",
        4,
        1,
        0,
        2,
        Some(
            "t.c (2, 5): Expected identifier or number\nt.c (4, 2): #endif outside of #if/#ifdef/#ifndef block\n",
        ),
    );
}

#[test]
fn c_characterization_case_41() {
    check(
        "#if 1 =! 2\nx\n#endif",
        "t.c",
        "\nx\n",
        3,
        0,
        0,
        2,
        Some(
            "t.c (1, 10): Expected comparator\nt.c (3, 2): #endif outside of #if/#ifdef/#ifndef block\n",
        ),
    );
}

#[test]
fn c_characterization_case_42() {
    check(
        "#if 1 2\nx\n#endif",
        "t.c",
        "\nx\n",
        3,
        0,
        0,
        2,
        Some(
            "t.c (1, 7): Expected comparator\nt.c (3, 2): #endif outside of #if/#ifdef/#ifndef block\n",
        ),
    );
}

#[test]
fn c_characterization_case_43() {
    check(
        "#if 1 ==\nx\n#endif",
        "t.c",
        "\nx\n",
        3,
        0,
        0,
        2,
        Some(
            "t.c (1, 9): Expected identifier or number\nt.c (3, 2): #endif outside of #if/#ifdef/#ifndef block\n",
        ),
    );
}

#[test]
fn c_characterization_case_44() {
    check(
        "#if 1\nx",
        "t.c",
        "\nx",
        2,
        0,
        0,
        1,
        Some("t.c (2, 2): Unexpected end of file before #endif\n"),
    );
}

#[test]
fn c_characterization_case_45() {
    check(
        "#if 0\na\n#elif 1\nb\n#else\nc\n#endif",
        "t.c",
        "\nb\n",
        3,
        0,
        0,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_46() {
    check(
        "#if 0\na\n#elif 0\nb\n#else\nc\n#endif",
        "t.c",
        "\nc\n",
        3,
        0,
        0,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_47() {
    check(
        "#ifdef X\na\n#elif 1\nb\n#endif",
        "t.c",
        "\nb\n",
        3,
        0,
        0,
        0,
        None,
    );
}

#[test]
fn c_characterization_case_48() {
    check(
        "#if 1\na\n#elif 1\nb\n#endif",
        "t.c",
        "\na\n",
        3,
        0,
        0,
        1,
        Some("t.c (3, 7): Unexpected token after preprocessor, expected newline\n"),
    );
}

#[test]
fn c_characterization_case_49() {
    check(
        "#elif 1\nx",
        "t.c",
        " \nx",
        3,
        0,
        0,
        2,
        Some(
            "t.c (1, 2): #elif outside of #if/#ifdef/#ifndef block\nt.c (1, 7): Unexpected token after preprocessor, expected newline\n",
        ),
    );
}

#[test]
fn c_characterization_case_50() {
    check(
        "#else\nx",
        "t.c",
        "\nx",
        2,
        0,
        0,
        1,
        Some("t.c (1, 2): #else outside of #if/#ifdef/#ifndef block\n"),
    );
}
