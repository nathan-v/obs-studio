//! Safe token-stream core of the C-family preprocessor.
use crate::cf_tokenizer::CfTokenType;

/// A token and its adapter-owned provenance handle.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: CfTokenType,
    pub text: Vec<u8>,
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
/// State persists across preprocessing calls, just as in C.
#[derive(Default)]
pub struct Preprocessor {
    pub definitions: Vec<Definition>,
    pub tokens: Vec<Token>,
    pub diagnostics: Vec<Diagnostic>,
    pub ignore: bool,
}
impl Preprocessor {
    /// Process one terminated token stream, loading local includes through the adapter.
    pub fn process(
        &mut self,
        _tokens: &[Token],
        _load: &mut impl FnMut(&Token) -> Option<Vec<Token>>,
    ) {
        todo!("cf preprocessor port")
    }
}
