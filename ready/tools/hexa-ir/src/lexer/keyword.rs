/// Keyword lookup — sigma=12 keywords + sigma-tau=8 type keywords
///
/// Maps identifier strings to their TokenKind.
/// Total entries = sigma + (sigma-tau) = 20 = σ + σ-τ.

use super::token::TokenKind;

/// Look up an identifier; if it matches a keyword, return the keyword TokenKind.
/// Otherwise return None (caller should produce Ident).
pub fn lookup_keyword(ident: &str) -> Option<TokenKind> {
    match ident {
        // Keywords (sigma=12)
        "fn"     => Some(TokenKind::Fn),
        "let"    => Some(TokenKind::Let),
        "mut"    => Some(TokenKind::Mut),
        "if"     => Some(TokenKind::If),
        "else"   => Some(TokenKind::Else),
        "while"  => Some(TokenKind::While),
        "return" => Some(TokenKind::Return),
        "struct" => Some(TokenKind::Struct),
        "enum"   => Some(TokenKind::Enum),
        "true"   => Some(TokenKind::True),
        "false"  => Some(TokenKind::False),
        "type"   => Some(TokenKind::Type),
        "match"  => Some(TokenKind::Match),
        "mod"    => Some(TokenKind::Mod),
        "use"    => Some(TokenKind::Use),
        "pub"    => Some(TokenKind::Pub),
        "trait"  => Some(TokenKind::Trait),
        "impl"   => Some(TokenKind::Impl),
        "for"    => Some(TokenKind::For),
        "in"     => Some(TokenKind::In),

        // Type keywords (sigma-tau=8)
        "i64"  => Some(TokenKind::KwI64),
        "f64"  => Some(TokenKind::KwF64),
        "bool" => Some(TokenKind::KwBool),
        "char" => Some(TokenKind::KwChar),
        "str"  => Some(TokenKind::KwStr),
        "byte" => Some(TokenKind::KwByte),
        "void" => Some(TokenKind::KwVoid),
        "any"  => Some(TokenKind::KwAny),

        _ => None,
    }
}
