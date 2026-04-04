/// Semantic analysis errors — sopfr=5 categories
///
/// Each error carries a span and human-readable message for diagnostics.

use crate::lexer::Span;

#[derive(Clone, Debug)]
pub enum SemaError {
    /// Type mismatch: expected vs found
    TypeError {
        span: Span,
        expected: String,
        found: String,
    },
    /// Ownership violation: use-after-move, double-borrow, etc.
    OwnershipError {
        span: Span,
        message: String,
    },
    /// Reference to undefined name
    UndefinedName {
        span: Span,
        name: String,
    },
    /// Name already defined in current scope
    DuplicateDefinition {
        span: Span,
        name: String,
    },
    /// Feature not yet supported in Mk.I
    Unsupported {
        span: Span,
        feature: String,
    },
}

impl SemaError {
    pub fn span(&self) -> Span {
        match self {
            SemaError::TypeError { span, .. } => *span,
            SemaError::OwnershipError { span, .. } => *span,
            SemaError::UndefinedName { span, .. } => *span,
            SemaError::DuplicateDefinition { span, .. } => *span,
            SemaError::Unsupported { span, .. } => *span,
        }
    }
}

impl std::fmt::Display for SemaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SemaError::TypeError { span, expected, found } => {
                write!(f, "[{}] type error: expected {}, found {}", span, expected, found)
            }
            SemaError::OwnershipError { span, message } => {
                write!(f, "[{}] ownership error: {}", span, message)
            }
            SemaError::UndefinedName { span, name } => {
                write!(f, "[{}] undefined name: '{}'", span, name)
            }
            SemaError::DuplicateDefinition { span, name } => {
                write!(f, "[{}] duplicate definition: '{}'", span, name)
            }
            SemaError::Unsupported { span, feature } => {
                write!(f, "[{}] unsupported in Mk.I: {}", span, feature)
            }
        }
    }
}
