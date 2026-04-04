/// Character cursor — scanning engine with phi=2 lookahead
///
/// Provides character-level iteration with position tracking.
/// The lexer builds tokens by calling peek/advance in tight loops.

use super::span::Span;

pub struct Cursor {
    chars: Vec<char>,
    pos: usize,
    line: u32,
    col: u32,
}

impl Cursor {
    pub fn new(source: &str) -> Self {
        Cursor {
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    /// Look at current character without consuming
    pub fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    /// phi=2 lookahead: look at the character after current
    pub fn peek_next(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    /// Consume current character and advance position
    pub fn advance(&mut self) -> Option<char> {
        let ch = self.chars.get(self.pos).copied()?;
        self.pos += 1;
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(ch)
    }

    /// Check if we've reached end of source
    pub fn is_eof(&self) -> bool {
        self.pos >= self.chars.len()
    }

    /// Current byte offset
    pub fn offset(&self) -> usize {
        self.pos
    }

    /// Current line number (1-based)
    pub fn line(&self) -> u32 {
        self.line
    }

    /// Current column number (1-based)
    pub fn col(&self) -> u32 {
        self.col
    }

    /// Create a span from a saved start position to current position
    pub fn span_from(&self, start: usize, start_line: u32, start_col: u32) -> Span {
        Span::new(start, self.pos, start_line, start_col)
    }

    /// Skip whitespace (spaces, tabs, newlines)
    pub fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_ascii_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Consume characters while predicate holds, returning the collected string
    pub fn eat_while<F: Fn(char) -> bool>(&mut self, pred: F) -> String {
        let mut s = String::new();
        while let Some(ch) = self.peek() {
            if pred(ch) {
                s.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        s
    }
}
