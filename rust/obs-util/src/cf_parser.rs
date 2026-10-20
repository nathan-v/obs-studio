//! Safe core of the two exported cf-parser functions.

/// Token access shared by the C adapter and safe test cursors.
pub trait Cursor {
    /// C token kind and first byte (zero for the terminator).
    fn token(&self) -> (i32, u8);
    /// Move to the next token in the terminated stream.
    fn advance(&mut self);
}

/// Skip a delimiter pair, preserving the C cursor postcondition.
pub fn pass_pair(cursor: &mut impl Cursor, open: u8, close: u8) -> bool {
    let _ = (cursor, open, close);
    todo!()
}

/// Format a parser diagnostic using the original sequential substitutions.
pub fn diagnostic(
    text: &[u8],
    position: usize,
    message: &[u8],
    values: [Option<&[u8]>; 3],
) -> (u32, u32, Vec<u8>) {
    let _ = (text, position, message, values);
    todo!()
}
