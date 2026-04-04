/// Lexer errors — sopfr=5 categories
///
/// Each error variant carries a Span so diagnostics can point to the exact
/// location in source code.

use super::span::Span;

#[derive(Clone, Debug)]
pub enum LexError {
    /// Character not recognized in any token pattern
    InvalidChar(Span, char),
    /// String literal missing closing quote
    UnterminatedString(Span),
    /// Malformed numeric literal (e.g. "12.34.56")
    InvalidNumber(Span, String),
    /// Block comment `/* ... */` missing closing `*/`
    UnterminatedComment(Span),
    /// Catch-all for unexpected situations
    Unknown(Span, String),
}

impl LexError {
    pub fn span(&self) -> Span {
        match self {
            LexError::InvalidChar(s, _) => *s,
            LexError::UnterminatedString(s) => *s,
            LexError::InvalidNumber(s, _) => *s,
            LexError::UnterminatedComment(s) => *s,
            LexError::Unknown(s, _) => *s,
        }
    }
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexError::InvalidChar(span, ch) => {
                write!(f, "[{}] invalid character '{}'", span, ch)
            }
            LexError::UnterminatedString(span) => {
                write!(f, "[{}] unterminated string literal", span)
            }
            LexError::InvalidNumber(span, s) => {
                write!(f, "[{}] invalid number '{}'", span, s)
            }
            LexError::UnterminatedComment(span) => {
                write!(f, "[{}] unterminated block comment", span)
            }
            LexError::Unknown(span, msg) => {
                write!(f, "[{}] {}", span, msg)
            }
        }
    }
}
