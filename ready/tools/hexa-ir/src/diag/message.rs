/// Diagnostic messages — structured error/warning/note representation
///
/// Each diagnostic carries a severity level, source span, message text,
/// and optional notes/labels for contextual information.

use crate::lexer::Span;

/// Diagnostic severity level
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    /// Fatal error — compilation cannot continue
    Error,
    /// Warning — potential issue, compilation continues
    Warning,
    /// Note — additional context for a previous diagnostic
    Note,
    /// Help — suggestion for fixing an error
    Help,
}

impl Level {
    /// ANSI color code for terminal rendering
    pub fn color_code(&self) -> &'static str {
        match self {
            Level::Error => "\x1b[1;31m",   // bold red
            Level::Warning => "\x1b[1;33m", // bold yellow
            Level::Note => "\x1b[1;36m",    // bold cyan
            Level::Help => "\x1b[1;32m",    // bold green
        }
    }

    /// Label text for the severity
    pub fn label(&self) -> &'static str {
        match self {
            Level::Error => "error",
            Level::Warning => "warning",
            Level::Note => "note",
            Level::Help => "help",
        }
    }
}

/// A secondary label attached to a diagnostic
#[derive(Clone, Debug)]
pub struct Label {
    pub span: Span,
    pub message: String,
}

/// A complete diagnostic message
#[derive(Clone, Debug)]
pub struct Diagnostic {
    /// Severity level
    pub level: Level,
    /// Primary source location
    pub span: Span,
    /// Main error/warning message
    pub message: String,
    /// Optional secondary labels (pointing to related code)
    pub labels: Vec<Label>,
    /// Optional notes (free-text context)
    pub notes: Vec<String>,
    /// Optional suggestions for fixing
    pub suggestions: Vec<String>,
}

impl Diagnostic {
    /// Create a new error diagnostic
    pub fn error(span: Span, message: impl Into<String>) -> Self {
        Diagnostic {
            level: Level::Error,
            span,
            message: message.into(),
            labels: Vec::new(),
            notes: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Create a new warning diagnostic
    pub fn warning(span: Span, message: impl Into<String>) -> Self {
        Diagnostic {
            level: Level::Warning,
            span,
            message: message.into(),
            labels: Vec::new(),
            notes: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Create a new note diagnostic
    pub fn note(span: Span, message: impl Into<String>) -> Self {
        Diagnostic {
            level: Level::Note,
            span,
            message: message.into(),
            labels: Vec::new(),
            notes: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Add a secondary label
    pub fn with_label(mut self, span: Span, message: impl Into<String>) -> Self {
        self.labels.push(Label { span, message: message.into() });
        self
    }

    /// Add a note
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Add a suggestion
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }
}

/// Diagnostic collector for a compilation session
pub struct DiagnosticBag {
    diagnostics: Vec<Diagnostic>,
    error_count: usize,
    warning_count: usize,
}

impl DiagnosticBag {
    pub fn new() -> Self {
        DiagnosticBag {
            diagnostics: Vec::new(),
            error_count: 0,
            warning_count: 0,
        }
    }

    /// Add a diagnostic
    pub fn emit(&mut self, diag: Diagnostic) {
        match diag.level {
            Level::Error => self.error_count += 1,
            Level::Warning => self.warning_count += 1,
            _ => {}
        }
        self.diagnostics.push(diag);
    }

    /// Check if any errors were emitted
    pub fn has_errors(&self) -> bool {
        self.error_count > 0
    }

    /// Get error count
    pub fn error_count(&self) -> usize {
        self.error_count
    }

    /// Get warning count
    pub fn warning_count(&self) -> usize {
        self.warning_count
    }

    /// Get all diagnostics
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Consume and return all diagnostics
    pub fn into_diagnostics(self) -> Vec<Diagnostic> {
        self.diagnostics
    }
}

impl Default for DiagnosticBag {
    fn default() -> Self {
        Self::new()
    }
}
