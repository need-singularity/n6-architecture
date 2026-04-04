/// HEXA-LANG Lexer — n=6 files, sigma=12 keywords, J2=24 operators
///
/// Transforms source text into a token stream.
/// File count = n = 6: mod.rs, token.rs, span.rs, keyword.rs, cursor.rs, error.rs

pub mod token;
pub mod span;
pub mod keyword;
pub mod cursor;
pub mod error;

pub use token::{Token, TokenKind};
pub use span::Span;
pub use error::LexError;

use cursor::Cursor;
use keyword::lookup_keyword;

/// Tokenize source code into a vector of Tokens.
/// Returns Ok(tokens) on success, Err(errors) if any lexical errors found.
/// Non-fatal: collects all errors and continues scanning.
pub fn lex(source: &str) -> Result<Vec<Token>, Vec<LexError>> {
    let mut cursor = Cursor::new(source);
    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    loop {
        cursor.skip_whitespace();

        if cursor.is_eof() {
            tokens.push(Token::new(
                TokenKind::Eof,
                cursor.span_from(cursor.offset(), cursor.line(), cursor.col()),
            ));
            break;
        }

        let start = cursor.offset();
        let start_line = cursor.line();
        let start_col = cursor.col();

        let ch = match cursor.peek() {
            Some(c) => c,
            None => break,
        };

        // ── Line comments ──
        if ch == '/' && cursor.peek_next() == Some('/') {
            cursor.advance(); // /
            cursor.advance(); // /
            cursor.eat_while(|c| c != '\n');
            continue;
        }

        // ── Block comments ──
        if ch == '/' && cursor.peek_next() == Some('*') {
            cursor.advance(); // /
            cursor.advance(); // *
            let mut depth = 1u32;
            while depth > 0 {
                match cursor.advance() {
                    Some('/') if cursor.peek() == Some('*') => {
                        cursor.advance();
                        depth += 1;
                    }
                    Some('*') if cursor.peek() == Some('/') => {
                        cursor.advance();
                        depth -= 1;
                    }
                    Some(_) => {}
                    None => {
                        errors.push(LexError::UnterminatedComment(
                            cursor.span_from(start, start_line, start_col),
                        ));
                        break;
                    }
                }
            }
            continue;
        }

        // ── Identifiers and keywords ──
        if ch.is_ascii_alphabetic() || ch == '_' {
            let word = {
                let mut s = String::new();
                s.push(ch);
                cursor.advance();
                while let Some(c) = cursor.peek() {
                    if c.is_ascii_alphanumeric() || c == '_' {
                        s.push(c);
                        cursor.advance();
                    } else {
                        break;
                    }
                }
                s
            };
            let span = cursor.span_from(start, start_line, start_col);
            let kind = lookup_keyword(&word).unwrap_or(TokenKind::Ident(word));
            tokens.push(Token::new(kind, span));
            continue;
        }

        // ── Numeric literals (int and float) ──
        if ch.is_ascii_digit() {
            let mut num_str = String::new();
            num_str.push(ch);
            cursor.advance();
            let mut is_float = false;

            while let Some(c) = cursor.peek() {
                if c.is_ascii_digit() {
                    num_str.push(c);
                    cursor.advance();
                } else if c == '.' && !is_float {
                    // Check it's not a method call like `123.foo`
                    if let Some(next) = cursor.peek_next() {
                        if next.is_ascii_digit() {
                            is_float = true;
                            num_str.push(c);
                            cursor.advance();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                } else if c == '_' {
                    // Allow numeric separators like 1_000_000
                    cursor.advance();
                } else {
                    break;
                }
            }

            let span = cursor.span_from(start, start_line, start_col);
            if is_float {
                match num_str.parse::<f64>() {
                    Ok(v) => tokens.push(Token::new(TokenKind::FloatLit(v), span)),
                    Err(_) => errors.push(LexError::InvalidNumber(span, num_str)),
                }
            } else {
                match num_str.parse::<i64>() {
                    Ok(v) => tokens.push(Token::new(TokenKind::IntLit(v), span)),
                    Err(_) => errors.push(LexError::InvalidNumber(span, num_str)),
                }
            }
            continue;
        }

        // ── String literals ──
        if ch == '"' {
            cursor.advance(); // opening quote
            let mut s = String::new();
            let mut terminated = false;
            while let Some(c) = cursor.advance() {
                if c == '"' {
                    terminated = true;
                    break;
                }
                if c == '\\' {
                    // Escape sequences
                    match cursor.advance() {
                        Some('n') => s.push('\n'),
                        Some('t') => s.push('\t'),
                        Some('\\') => s.push('\\'),
                        Some('"') => s.push('"'),
                        Some('0') => s.push('\0'),
                        Some(other) => { s.push('\\'); s.push(other); }
                        None => break,
                    }
                } else {
                    s.push(c);
                }
            }
            let span = cursor.span_from(start, start_line, start_col);
            if terminated {
                tokens.push(Token::new(TokenKind::StrLit(s), span));
            } else {
                errors.push(LexError::UnterminatedString(span));
            }
            continue;
        }

        // ── Operators and delimiters ──
        cursor.advance(); // consume the character
        let next = cursor.peek();
        let span2 = |cur: &Cursor| cur.span_from(start, start_line, start_col);

        let kind = match ch {
            '+' => TokenKind::Plus,
            '*' => TokenKind::Star,
            '%' => TokenKind::Percent,
            '^' => TokenKind::Caret,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semicolon,
            '#' => TokenKind::Hash,
            '@' => TokenKind::At,
            '?' => TokenKind::Question,
            '~' => TokenKind::BitXor,
            '-' => {
                if next == Some('>') { cursor.advance(); TokenKind::Arrow }
                else { TokenKind::Minus }
            }
            '=' => {
                if next == Some('=') { cursor.advance(); TokenKind::Eq }
                else if next == Some('>') { cursor.advance(); TokenKind::FatArrow }
                else { TokenKind::Assign }
            }
            '!' => {
                if next == Some('=') { cursor.advance(); TokenKind::Neq }
                else { TokenKind::Not }
            }
            '<' => {
                if next == Some('=') { cursor.advance(); TokenKind::Le }
                else { TokenKind::Lt }
            }
            '>' => {
                if next == Some('=') { cursor.advance(); TokenKind::Ge }
                else { TokenKind::Gt }
            }
            '&' => {
                if next == Some('&') { cursor.advance(); TokenKind::And }
                else { TokenKind::BitAnd }
            }
            '|' => {
                if next == Some('|') { cursor.advance(); TokenKind::Or }
                else { TokenKind::BitOr }
            }
            '/' => TokenKind::Slash,
            '.' => {
                if next == Some('.') {
                    cursor.advance();
                    if cursor.peek() == Some('=') { cursor.advance(); TokenKind::DotDotEq }
                    else { TokenKind::DotDot }
                }
                else { TokenKind::Dot }
            }
            ':' => {
                if next == Some(':') { cursor.advance(); TokenKind::ColonColon }
                else { TokenKind::Colon }
            }
            _ => {
                let span = cursor.span_from(start, start_line, start_col);
                errors.push(LexError::InvalidChar(span, ch));
                continue;
            }
        };

        tokens.push(Token::new(kind, span2(&cursor)));
    }

    if errors.is_empty() {
        Ok(tokens)
    } else {
        Err(errors)
    }
}
