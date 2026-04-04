/// Diagnostic Renderer — terminal output with ANSI colors
///
/// Renders diagnostics in the familiar Rust/GCC style:
///
///   error[E0001]: use of moved value 'x'
///    --> src/main.hx:42:5
///     |
///   42|     foo(x);
///     |         ^ value moved here
///     |
///     = note: consider borrowing instead
///     = help: use `&x` to pass by reference

use super::message::{Diagnostic, Level};

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const BLUE: &str = "\x1b[1;34m";

/// Render a single diagnostic to a string with ANSI colors
pub fn render_diagnostic(diag: &Diagnostic, source: Option<&str>) -> String {
    let mut out = String::new();

    // Header line: error: message
    out.push_str(diag.level.color_code());
    out.push_str(diag.level.label());
    out.push_str(RESET);
    out.push_str(BOLD);
    out.push_str(": ");
    out.push_str(&diag.message);
    out.push_str(RESET);
    out.push('\n');

    // Location line: --> file:line:col
    out.push_str(BLUE);
    out.push_str(" --> ");
    out.push_str(RESET);
    out.push_str(&format!("{}:{}\n", diag.span.line, diag.span.col));

    // Source context (if available)
    if let Some(src) = source {
        let line_num = diag.span.line as usize;
        let lines: Vec<&str> = src.lines().collect();

        if line_num > 0 && line_num <= lines.len() {
            let source_line = lines[line_num - 1];

            // Line number gutter
            let gutter_width = format!("{}", line_num).len();

            // Empty gutter line
            out.push_str(BLUE);
            out.push_str(&format!("{:>width$} |", "", width = gutter_width));
            out.push_str(RESET);
            out.push('\n');

            // Source line
            out.push_str(BLUE);
            out.push_str(&format!("{:>width$} | ", line_num, width = gutter_width));
            out.push_str(RESET);
            out.push_str(source_line);
            out.push('\n');

            // Caret line pointing to the error
            out.push_str(BLUE);
            out.push_str(&format!("{:>width$} | ", "", width = gutter_width));
            out.push_str(RESET);
            let col = diag.span.col as usize;
            let span_len = diag.span.len().max(1);
            if col > 0 {
                out.push_str(&" ".repeat(col.saturating_sub(1)));
            }
            out.push_str(diag.level.color_code());
            out.push_str(&"^".repeat(span_len));
            out.push_str(RESET);
            out.push('\n');
        }
    }

    // Secondary labels
    for label in &diag.labels {
        out.push_str(BLUE);
        out.push_str("   = ");
        out.push_str(RESET);
        out.push_str(DIM);
        out.push_str(&format!("{}:{}: ", label.span.line, label.span.col));
        out.push_str(RESET);
        out.push_str(&label.message);
        out.push('\n');
    }

    // Notes
    for note in &diag.notes {
        out.push_str(BLUE);
        out.push_str("   = ");
        out.push_str(RESET);
        out.push_str(Level::Note.color_code());
        out.push_str("note");
        out.push_str(RESET);
        out.push_str(": ");
        out.push_str(note);
        out.push('\n');
    }

    // Suggestions
    for suggestion in &diag.suggestions {
        out.push_str(BLUE);
        out.push_str("   = ");
        out.push_str(RESET);
        out.push_str(Level::Help.color_code());
        out.push_str("help");
        out.push_str(RESET);
        out.push_str(": ");
        out.push_str(suggestion);
        out.push('\n');
    }

    out
}

/// Render all diagnostics and print a summary
pub fn render_all(diagnostics: &[Diagnostic], source: Option<&str>) -> String {
    let mut out = String::new();
    for diag in diagnostics {
        out.push_str(&render_diagnostic(diag, source));
        out.push('\n');
    }

    // Summary
    let errors = diagnostics.iter().filter(|d| d.level == Level::Error).count();
    let warnings = diagnostics.iter().filter(|d| d.level == Level::Warning).count();

    if errors > 0 || warnings > 0 {
        out.push_str(BOLD);
        if errors > 0 {
            out.push_str(Level::Error.color_code());
            out.push_str(&format!("{} error{}", errors, if errors == 1 { "" } else { "s" }));
            out.push_str(RESET);
            if warnings > 0 {
                out.push_str("; ");
            }
        }
        if warnings > 0 {
            out.push_str(Level::Warning.color_code());
            out.push_str(&format!("{} warning{}", warnings, if warnings == 1 { "" } else { "s" }));
            out.push_str(RESET);
        }
        out.push_str(" emitted\n");
    }

    out
}
