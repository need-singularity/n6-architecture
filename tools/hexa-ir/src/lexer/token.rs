/// Tokens — the atomic units produced by the lexer
///
/// TokenKind groups follow n=6 arithmetic:
///   - Literals:     tau=4 kinds
///   - Keywords:     sigma=12 (Mk.I)
///   - TypeKeywords: sigma-tau=8
///   - Operators:    4 groups of n=6 = J2=24
///   - Delimiters:   sigma=12

use super::span::Span;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    // ── Literals (tau=4) ──
    IntLit(i64),
    FloatLit(f64),
    StrLit(String),
    BoolLit(bool),

    // ── Keywords (sigma+tau=16 for Mk.II: +match+mod+use+pub) ──
    Fn,
    Let,
    Mut,
    If,
    Else,
    While,
    Return,
    Struct,
    Enum,
    True,
    False,
    Type,
    Match,
    Mod,
    Use,
    Pub,
    Trait,
    Impl,
    For,
    In,

    // ── Type keywords (sigma-tau=8) ──
    KwI64,
    KwF64,
    KwBool,
    KwChar,
    KwStr,
    KwByte,
    KwVoid,
    KwAny,

    // ── Arithmetic operators (n=6) ──
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Percent,    // %
    Caret,      // ^

    // ── Comparison operators (n=6) ──
    Eq,         // ==
    Neq,        // !=
    Lt,         // <
    Gt,         // >
    Le,         // <=
    Ge,         // >=

    // ── Logic operators (n=6) ──
    And,        // &&
    Or,         // ||
    Not,        // !
    BitAnd,     // &
    BitOr,      // |
    BitXor,     // ~

    // ── Structural operators (n=6) ──
    Assign,     // =
    Arrow,      // ->
    FatArrow,   // =>
    Dot,        // .
    DotDot,     // ..
    DotDotEq,   // ..=
    Colon,      // :

    // ── Delimiters (sigma=12) ──
    LParen,     // (
    RParen,     // )
    LBrace,     // {
    RBrace,     // }
    LBracket,   // [
    RBracket,   // ]
    Comma,      // ,
    Semicolon,  // ;
    ColonColon, // ::
    Hash,       // #
    At,         // @
    Question,   // ?

    // ── Other ──
    Ident(String),
    Eof,
}

impl TokenKind {
    /// Returns true for tokens that can start an expression
    pub fn is_expr_start(&self) -> bool {
        matches!(
            self,
            TokenKind::IntLit(_) | TokenKind::FloatLit(_) |
            TokenKind::StrLit(_) | TokenKind::BoolLit(_) |
            TokenKind::True | TokenKind::False |
            TokenKind::Ident(_) | TokenKind::LParen | TokenKind::LBracket |
            TokenKind::Minus | TokenKind::Not |
            TokenKind::If | TokenKind::Match
        )
    }

    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            TokenKind::IntLit(_) | TokenKind::FloatLit(_) |
            TokenKind::StrLit(_) | TokenKind::BoolLit(_)
        )
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Token { kind, span }
    }

    pub fn eof() -> Self {
        Token { kind: TokenKind::Eof, span: Span::dummy() }
    }
}
