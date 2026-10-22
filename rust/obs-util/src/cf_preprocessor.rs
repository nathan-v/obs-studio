//! Safe token-stream core of the C-family preprocessor.
use crate::cf_tokenizer::CfTokenType;

/// A token and its adapter-owned provenance handle.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: CfTokenType,
    pub text: Vec<u8>,
    /// Bytes from the token to the source NUL, for the C include diagnostic.
    pub tail: Vec<u8>,
    pub origin: usize,
    pub filler: bool,
}
/// A macro definition, including the C terminator and trailing filler.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Definition {
    pub name: Token,
    pub params: Vec<Token>,
    pub tokens: Vec<Token>,
    pub macro_call: bool,
}
/// A diagnostic attached to an original token.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub token: Token,
    pub message: Vec<u8>,
    pub level: i32,
}
/// Host scanf grammar choices, supplied through the platform boundary.
#[derive(Clone, Copy, Debug)]
pub struct IntegerSyntax {
    pub incomplete_hex_is_zero: bool,
    pub binary_prefix: bool,
}
impl Default for IntegerSyntax {
    fn default() -> Self {
        let (incomplete_hex_is_zero, binary_prefix) =
            crate::platform::crt::preprocessor_integer_syntax();
        Self {
            incomplete_hex_is_zero,
            binary_prefix,
        }
    }
}
/// State persists across preprocessing calls, just as in C.
#[derive(Default)]
pub struct Preprocessor {
    pub integer_syntax: IntegerSyntax,
    pub definitions: Vec<Definition>,
    pub tokens: Vec<Token>,
    pub diagnostics: Vec<Diagnostic>,
    pub ignore: bool,
}
use CfTokenType::{Name, Newline, None as End, Num, Other, SpaceTab, String as Literal};

fn current(tokens: &[Token], i: usize) -> &Token {
    &tokens[i.min(tokens.len() - 1)]
}
fn next(tokens: &[Token], i: &mut usize) -> bool {
    if current(tokens, *i).kind != End {
        *i += 1;
    }
    while current(tokens, *i).kind == SpaceTab {
        *i += 1;
    }
    current(tokens, *i).kind != End
}
fn newline(tokens: &[Token], i: &mut usize) -> bool {
    while !matches!(current(tokens, *i).kind, Newline | End) {
        *i += 1;
    }
    current(tokens, *i).kind != End
}
fn filler(base: &Token) -> Token {
    Token {
        kind: SpaceTab,
        text: b" ".to_vec(),
        tail: Vec::new(),
        origin: base.origin,
        filler: true,
    }
}
fn root_filler(base: &Token) -> Token {
    Token {
        origin: usize::MAX,
        ..filler(base)
    }
}
fn end(base: &Token) -> Token {
    Token {
        kind: End,
        text: Vec::new(),
        tail: Vec::new(),
        origin: base.origin,
        filler: true,
    }
}

/// File loading belongs to the host adapter; completion preserves C's
/// depth-first dependency order without storing host pointers in the core.
pub trait Includes {
    fn load(&mut self, token: &Token) -> Option<Vec<Token>>;
    fn complete(&mut self, _token: &Token) {}
}
impl<F: FnMut(&Token) -> Option<Vec<Token>>> Includes for F {
    fn load(&mut self, token: &Token) -> Option<Vec<Token>> {
        self(token)
    }
}

/// Resolve all includes relative to the root file, as the C preprocessor does.
pub fn include_path(root: &[u8], literal: &[u8]) -> Vec<u8> {
    let mut path = root
        .iter()
        .rposition(|&b| b == b'/')
        .map_or_else(Vec::new, |i| root[..=i].to_vec());
    path.extend_from_slice(&literal[1..literal.len() - 1]);
    path
}

impl Preprocessor {
    fn error(&mut self, token: &Token, message: impl Into<Vec<u8>>) {
        self.diagnostics.push(Diagnostic {
            token: token.clone(),
            message: message.into(),
            level: 0,
        });
    }
    fn expected(&mut self, token: &Token, what: &str) {
        self.error(token, format!("Expected {what}").into_bytes());
    }
    fn def_index(&self, name: &[u8]) -> Option<usize> {
        self.definitions.iter().position(|d| d.name.text == name)
    }
    /// Remove only the first matching definition, preserving duplicate #defines.
    pub fn remove(&mut self, name: &[u8]) {
        if let Some(i) = self.def_index(name) {
            self.definitions.remove(i);
        }
    }
    /// Public add_def replaces the first match, unlike a #define directive.
    pub fn add(&mut self, definition: Definition) {
        if let Some(i) = self.def_index(&definition.name.text) {
            for (token, prefix, suffix) in [
                (
                    &definition.name,
                    b"Token ".as_slice(),
                    b" already defined".as_slice(),
                ),
                (
                    &self.definitions[i].name,
                    b"Previous definition of ".as_slice(),
                    b" is here".as_slice(),
                ),
            ] {
                self.diagnostics.push(Diagnostic {
                    token: token.clone(),
                    message: [prefix, &definition.name.text, suffix].concat(),
                    level: 1,
                });
            }
            self.definitions[i] = definition;
        } else {
            self.definitions.push(definition);
        }
    }
    /// Parse the leading integer as sscanf("%i"), ignoring any trailing bytes.
    pub fn value(&self, token: &Token) -> Option<i32> {
        self.value_inner(token, 0)
    }
    fn value_inner(&self, token: &Token, depth: usize) -> Option<i32> {
        if depth > self.definitions.len() {
            return None;
        }
        if token.kind == Num {
            return integer(&token.text, self.integer_syntax);
        }
        if token.kind != Name {
            return None;
        }
        let d = &self.definitions[self.def_index(&token.text)?];
        if d.macro_call {
            return None;
        }
        let t = d.tokens.iter().find(|t| t.kind != SpaceTab)?;
        self.value_inner(t, depth + 1)
    }
    fn define(&mut self, tokens: &[Token], i: &mut usize) {
        if self.ignore {
            newline(tokens, i);
            return;
        }
        next(tokens, i);
        if current(tokens, *i).kind != Name {
            self.expected(current(tokens, *i), "identifier");
            newline(tokens, i);
            return;
        }
        let name = current(tokens, *i).clone();
        let mut d = Definition {
            name: name.clone(),
            params: Vec::new(),
            tokens: vec![root_filler(&tokens[0])],
            macro_call: false,
        };
        if next(tokens, i) {
            if current(tokens, *i).text == b"(" {
                d.macro_call = true;
                loop {
                    next(tokens, i);
                    if current(tokens, *i).kind != Name {
                        self.expected(current(tokens, *i), "identifier");
                        newline(tokens, i);
                        return;
                    }
                    d.params.push(current(tokens, *i).clone());
                    next(tokens, i);
                    let t = current(tokens, *i);
                    if t.kind != Other || (t.text != b"," && t.text != b")") {
                        self.expected(t, "',' or ')'");
                        newline(tokens, i);
                        return;
                    }
                    if t.text == b")" {
                        break;
                    }
                }
                next(tokens, i);
            }
            while !matches!(current(tokens, *i).kind, Newline | End) {
                d.tokens.push(current(tokens, *i).clone());
                *i += 1;
            }
        }
        d.tokens.push(end(&name));
        d.tokens.push(root_filler(&tokens[0]));
        self.definitions.push(d);
    }
    fn subblock(
        &mut self,
        tokens: &[Token],
        i: &mut usize,
        ignore: bool,
        load: &mut impl Includes,
    ) -> bool {
        if !next(tokens, i) {
            return false;
        }
        let old = self.ignore;
        self.ignore |= ignore;
        self.walk(tokens, i, true, load);
        self.ignore = old;
        if current(tokens, *i).kind == End {
            self.error(
                current(tokens, *i),
                b"Unexpected end of file before #endif".to_vec(),
            );
            false
        } else {
            true
        }
    }
    fn conditional(
        &mut self,
        tokens: &[Token],
        i: &mut usize,
        directive: &[u8],
        load: &mut impl Includes,
    ) {
        next(tokens, i);
        let is_true;
        if directive == b"ifdef" || directive == b"ifndef" {
            let t = current(tokens, *i);
            if t.kind != Name {
                self.expected(t, "identifier");
                newline(tokens, i);
                return;
            }
            is_true = self.def_index(&t.text).is_none() == (directive == b"ifndef");
        } else {
            let Some(a) = self.value(current(tokens, *i)) else {
                self.expected(current(tokens, *i), "identifier or number");
                newline(tokens, i);
                return;
            };
            next(tokens, i);
            if current(tokens, *i).kind == Newline {
                is_true = a != 0;
                *i = i.saturating_sub(1);
            } else {
                if current(tokens, *i).kind != Other {
                    self.expected(current(tokens, *i), "comparator");
                    newline(tokens, i);
                    return;
                }
                let mut comparator = Vec::new();
                while current(tokens, *i).kind == Other {
                    comparator.extend_from_slice(&current(tokens, *i).text);
                    *i += 1;
                }
                *i -= 1;
                next(tokens, i);
                let Some(b) = self.value(current(tokens, *i)) else {
                    self.expected(current(tokens, *i), "identifier or number");
                    newline(tokens, i);
                    return;
                };
                is_true = match comparator.as_slice() {
                    b"==" => a == b,
                    b">=" => a >= b,
                    b"<=" => a <= b,
                    b"!=" => a != b,
                    b"<" => a < b,
                    b">" => a > b,
                    _ => {
                        self.expected(current(tokens, *i), "comparator");
                        newline(tokens, i);
                        return;
                    }
                };
            }
        }
        if !self.subblock(tokens, i, !is_true, load) {
            return;
        }
        let t = current(tokens, *i);
        if t.text == b"else" {
            if !self.subblock(tokens, i, is_true, load) {
                return;
            }
        } else if t.text == b"elif" {
            if is_true {
                if !self.subblock(tokens, i, true, load) {
                    return;
                }
            } else {
                self.conditional(tokens, i, b"if", load);
                return;
            }
        }
        *i += 1;
    }
    fn directive(
        &mut self,
        tokens: &[Token],
        i: &mut usize,
        block: bool,
        load: &mut impl Includes,
    ) -> bool {
        let t = current(tokens, *i).clone();
        match t.text.as_slice() {
            b"define" => self.define(tokens, i),
            b"undef" => {
                if self.ignore {
                    newline(tokens, i);
                } else {
                    next(tokens, i);
                    if current(tokens, *i).kind != Name {
                        self.expected(current(tokens, *i), "identifier");
                        newline(tokens, i);
                    } else {
                        self.remove(&current(tokens, *i).text);
                        *i += 1;
                    }
                }
            }
            b"ifdef" | b"ifndef" | b"if" => self.conditional(tokens, i, &t.text, load),
            b"else" | b"elif" | b"endif" => {
                if block {
                    return false;
                }
                self.error(
                    &t,
                    [
                        b"#".as_slice(),
                        &t.text,
                        b" outside of #if/#ifdef/#ifndef block",
                    ]
                    .concat(),
                );
                *i += 1;
            }
            b"include" => {
                if self.ignore {
                    newline(tokens, i);
                } else {
                    next(tokens, i);
                    let file = current(tokens, *i).clone();
                    if file.kind != Literal {
                        self.expected(&file, "string");
                        newline(tokens, i);
                    } else if file.text.len() >= 2
                        && file.text.first() == Some(&b'<')
                        && file.text.last() == Some(&b'>')
                    {
                        *i += 1;
                    } else if file.text.len() >= 2
                        && file.text.first() == Some(&b'"')
                        && file.text.last() == Some(&b'"')
                    {
                        if let Some(included) = load.load(&file) {
                            if !included.is_empty() {
                                self.walk(&included, &mut 0, false, load);
                            }
                            load.complete(&file);
                        } else {
                            // The adapter supplies the full NUL-terminated tail for this diagnostic.
                            self.error(
                                &file,
                                [b"Could not open file '".as_slice(), &file.tail, b"'"].concat(),
                            );
                        }
                        *i += 1;
                    } else {
                        self.error(&file, b"Invalid or incomplete string".to_vec());
                        newline(tokens, i);
                    }
                }
            }
            _ => {
                if !matches!(t.kind, Newline | End) {
                    *i += 1;
                }
            }
        }
        true
    }
    fn expand(
        &mut self,
        tokens: &[Token],
        i: &mut usize,
        base: Option<&Token>,
        params: &[(Vec<u8>, Vec<Token>)],
        dst: &mut Vec<Token>,
    ) {
        let t = current(tokens, *i).clone();
        if self.ignore {
            *i += 1;
            return;
        }
        let base = base.unwrap_or(&t);
        if t.kind == Name {
            if let Some((_, values)) = params.iter().find(|(name, _)| *name == t.text) {
                let mut j = 0;
                while current(values, j).kind != End {
                    self.expand(values, &mut j, Some(base), &[], dst);
                }
                *i += 1;
                return;
            }
            if let Some(index) = self.def_index(&t.text) {
                let def = self.definitions[index].clone();
                let mut new_params = Vec::new();
                if def.macro_call {
                    // C next_token(false) advances once without skipping
                    // spaces or tabs. Preserve its nonstandard call syntax.
                    *i += 1;
                    if current(tokens, *i).kind != Other || current(tokens, *i).text != b"(" {
                        self.expected(current(tokens, *i), "'('");
                    } else {
                        let mut count = 0;
                        loop {
                            *i += 1;
                            count += 1;
                            let mut values = vec![filler(base)];
                            let mut braces = 0;
                            while current(tokens, *i).kind != End {
                                let t = current(tokens, *i);
                                if t.text == b"(" {
                                    braces += 1;
                                } else if t.text == b")" {
                                    if braces == 0 {
                                        break;
                                    }
                                    braces -= 1;
                                } else if t.text == b"," && braces == 0 {
                                    break;
                                }
                                self.expand(tokens, i, Some(base), params, &mut values);
                            }
                            if current(tokens, *i).kind == End {
                                self.error(current(tokens, *i), b"Unexpected end of file".to_vec());
                            }
                            values.push(filler(base));
                            values.push(end(base));
                            let t = current(tokens, *i);
                            if t.kind != Other || (t.text != b"," && t.text != b")") {
                                self.expected(t, "',' or ')'");
                                break;
                            }
                            if count == 1
                                && def.params.is_empty()
                                && t.text == b")"
                                && values
                                    .iter()
                                    .all(|v| matches!(v.kind, End | SpaceTab | Newline))
                            {
                                break;
                            }
                            if count <= def.params.len() {
                                new_params.push((def.params[count - 1].text.clone(), values));
                            }
                            if t.text == b")" {
                                break;
                            }
                        }
                        if count != def.params.len() {
                            self.error(
                                current(tokens, *i),
                                b"Mismatching number of macro parameters".to_vec(),
                            );
                        }
                    }
                }
                let mut j = 0;
                while current(&def.tokens, j).kind != End {
                    self.expand(&def.tokens, &mut j, Some(base), &new_params, dst);
                }
                *i += usize::from(current(tokens, *i).kind != End);
                return;
            }
        }
        dst.push(t);
        *i += 1;
    }
    fn walk(&mut self, tokens: &[Token], i: &mut usize, block: bool, load: &mut impl Includes) {
        let mut at_newline = true;
        let mut directive_line = block;
        while current(tokens, *i).kind != End {
            if !matches!(current(tokens, *i).kind, SpaceTab | Newline) {
                if directive_line {
                    self.error(
                        current(tokens, *i),
                        b"Unexpected token after preprocessor, expected newline".to_vec(),
                    );
                    if !newline(tokens, i) {
                        break;
                    }
                }
                if at_newline && current(tokens, *i).text == b"#" {
                    next(tokens, i);
                    directive_line = true;
                    if !self.directive(tokens, i, block, load) {
                        break;
                    }
                    continue;
                }
                at_newline = false;
            }
            if current(tokens, *i).kind == Newline {
                at_newline = true;
                directive_line = false;
            } else if current(tokens, *i).kind == End {
                break;
            }
            let mut out = Vec::new();
            self.expand(tokens, i, None, &[], &mut out);
            self.tokens.extend(out);
        }
    }
    /// Process one terminated stream; output is appended, including its terminator.
    pub fn process(&mut self, tokens: &[Token], load: &mut impl Includes) {
        if tokens.is_empty() {
            return;
        }
        let mut i = 0;
        self.walk(tokens, &mut i, false, load);
        self.tokens.push(current(tokens, i).clone());
    }
}
fn integer(bytes: &[u8], syntax: IntegerSyntax) -> Option<i32> {
    let bytes = bytes.trim_ascii_start();
    let (negative, bytes) = match bytes.first() {
        Some(b'-') => (true, &bytes[1..]),
        Some(b'+') => (false, &bytes[1..]),
        _ => (false, bytes),
    };
    let (radix, start) = if bytes.starts_with(b"0x") || bytes.starts_with(b"0X") {
        (16, 2)
    } else if syntax.binary_prefix && (bytes.starts_with(b"0b") || bytes.starts_with(b"0B")) {
        (2, 2)
    } else if bytes.first() == Some(&b'0') {
        (8, 0)
    } else {
        (10, 0)
    };
    let mut found = radix == 16 && syntax.incomplete_hex_is_zero;
    let mut value = 0i32;
    for &b in &bytes[start..] {
        let digit = match b {
            b'0'..=b'9' => u32::from(b - b'0'),
            b'a'..=b'f' => u32::from(b - b'a' + 10),
            b'A'..=b'F' => u32::from(b - b'A' + 10),
            _ => break,
        };
        if digit >= radix {
            break;
        }
        value = value.wrapping_mul(radix as i32).wrapping_add(digit as i32);
        found = true;
    }
    found.then_some(if negative {
        value.wrapping_neg()
    } else {
        value
    })
}
