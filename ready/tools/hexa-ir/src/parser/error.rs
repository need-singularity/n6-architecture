/// Parser error + Parser struct (shared by expr/stmt/decl modules)
///
/// The Parser struct lives here because all parsing sub-modules need it,
/// and it owns the token stream + error accumulator.

use crate::lexer::{Token, TokenKind, Span};

// ── Parse Error ──

#[derive(Clone, Debug)]
pub struct ParseError {
    pub span: Span,
    pub message: String,
}

impl ParseError {
    pub fn new(span: Span, message: impl Into<String>) -> Self {
        ParseError { span, message: message.into() }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] parse error: {}", self.span, self.message)
    }
}

// ── Parser State ──

pub struct Parser {
    pub tokens: Vec<Token>,
    pub pos: usize,
    pub errors: Vec<ParseError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0, errors: Vec::new() }
    }

    /// Look at current token without consuming
    pub fn peek(&self) -> &TokenKind {
        self.tokens.get(self.pos)
            .map(|t| &t.kind)
            .unwrap_or(&TokenKind::Eof)
    }

    /// Look at current token's span
    pub fn peek_span(&self) -> Span {
        self.tokens.get(self.pos)
            .map(|t| t.span)
            .unwrap_or(Span::dummy())
    }

    /// Consume current token and return it
    pub fn advance(&mut self) -> &Token {
        let tok = &self.tokens[self.pos.min(self.tokens.len() - 1)];
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        tok
    }

    /// Check if current token matches the given kind
    pub fn at(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(self.peek()) == std::mem::discriminant(kind)
    }

    /// Consume if current token matches; return true if consumed
    pub fn eat(&mut self, kind: &TokenKind) -> bool {
        if self.at(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Expect current token to be `kind`; consume and return span, or push error
    pub fn expect(&mut self, kind: &TokenKind) -> Result<Span, ParseError> {
        if self.at(kind) {
            let span = self.peek_span();
            self.advance();
            Ok(span)
        } else {
            let span = self.peek_span();
            let err = ParseError::new(
                span,
                format!("expected {:?}, found {:?}", kind, self.peek()),
            );
            Err(err)
        }
    }

    /// Expect and consume an identifier, returning its name
    pub fn expect_ident(&mut self) -> Result<(String, Span), ParseError> {
        let span = self.peek_span();
        match self.peek().clone() {
            TokenKind::Ident(name) => {
                let name = name.clone();
                self.advance();
                Ok((name, span))
            }
            other => Err(ParseError::new(
                span,
                format!("expected identifier, found {:?}", other),
            )),
        }
    }

    /// Check if parser has reached EOF
    pub fn is_eof(&self) -> bool {
        matches!(self.peek(), TokenKind::Eof)
    }

    /// Record an error and attempt to synchronize to the next statement boundary
    pub fn error_and_sync(&mut self, err: ParseError) {
        self.errors.push(err);
        // Skip tokens until we find a likely statement start
        while !self.is_eof() {
            match self.peek() {
                TokenKind::Semicolon => { self.advance(); return; }
                TokenKind::RBrace => return,
                TokenKind::Fn | TokenKind::Let | TokenKind::Struct |
                TokenKind::Enum | TokenKind::Return | TokenKind::If |
                TokenKind::While | TokenKind::For | TokenKind::Type |
                TokenKind::Match | TokenKind::Mod | TokenKind::Use |
                TokenKind::Pub | TokenKind::Trait | TokenKind::Impl => return,
                _ => { self.advance(); }
            }
        }
    }
}
