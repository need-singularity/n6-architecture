/// Source location span — tracks byte offsets + line/col for diagnostics
///
/// Every token and AST node carries a Span for error reporting.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: u32,
    pub col: u32,
}

impl Span {
    pub fn new(start: usize, end: usize, line: u32, col: u32) -> Self {
        Span { start, end, line, col }
    }

    /// Dummy span for synthetic tokens (EOF, generated nodes)
    pub fn dummy() -> Self {
        Span { start: 0, end: 0, line: 0, col: 0 }
    }

    /// Merge two spans into one covering both
    pub fn merge(self, other: Span) -> Span {
        let start = self.start.min(other.start);
        let end = self.end.max(other.end);
        // Keep the earlier span's line/col
        let (line, col) = if self.start <= other.start {
            (self.line, self.col)
        } else {
            (other.line, other.col)
        };
        Span { start, end, line, col }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}
