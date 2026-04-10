/// LaTeX document generator for NEXUS-6 discoveries.

/// Generate a complete LaTeX document from experiment results.
///
/// Produces a standalone article-class document with:
///   - Title, abstract, findings table
///   - n=6 connections section
///   - Testable predictions
pub fn generate_latex(
    title: &str,
    abstract_text: &str,
    findings: &[(String, f64)],
    n6_connections: &[String],
    predictions: &[String],
) -> String {
    let mut doc = String::new();

    // Preamble
    doc.push_str("\\documentclass{article}\n");
    doc.push_str("\\usepackage[utf8]{inputenc}\n");
    doc.push_str("\\usepackage{amsmath,amssymb}\n");
    doc.push_str("\\usepackage{booktabs}\n");
    doc.push_str("\\usepackage{hyperref}\n");
    doc.push_str("\n");
    doc.push_str(&format!("\\title{{{}}}\n", escape_latex(title)));
    doc.push_str("\\author{NEXUS-6 Discovery Engine}\n");
    doc.push_str("\\date{\\today}\n");
    doc.push_str("\n");
    doc.push_str("\\begin{document}\n");
    doc.push_str("\\maketitle\n\n");

    // Abstract
    doc.push_str("\\begin{abstract}\n");
    doc.push_str(&escape_latex(abstract_text));
    doc.push_str("\n\\end{abstract}\n\n");

    // Core theorem
    doc.push_str("\\section{Foundation}\n");
    doc.push_str("All results are analyzed through the lens of the n=6 uniqueness theorem:\n");
    doc.push_str("\\begin{equation}\n");
    doc.push_str("\\sigma(n) \\cdot \\varphi(n) = n \\cdot \\tau(n) \\iff n = 6\n");
    doc.push_str("\\end{equation}\n\n");

    // Findings table
    if !findings.is_empty() {
        doc.push_str("\\section{Findings}\n\n");
        doc.push_str("\\begin{table}[h]\n");
        doc.push_str("\\centering\n");
        doc.push_str("\\begin{tabular}{lrl}\n");
        doc.push_str("\\toprule\n");
        doc.push_str("Metric & Value & n=6 Match \\\\\n");
        doc.push_str("\\midrule\n");

        for (name, value) in findings {
            let (match_name, quality) = crate::verifier::n6_check::n6_match(*value);
            let grade = if quality >= 1.0 {
                "EXACT"
            } else if quality >= 0.8 {
                "CLOSE"
            } else if quality >= 0.5 {
                "WEAK"
            } else {
                "--"
            };
            doc.push_str(&format!(
                "{} & {:.4} & {} ({}) \\\\\n",
                escape_latex(name),
                value,
                match_name,
                grade
            ));
        }

        doc.push_str("\\bottomrule\n");
        doc.push_str("\\end{tabular}\n");
        doc.push_str("\\caption{Experimental findings with n=6 correspondence}\n");
        doc.push_str("\\end{table}\n\n");
    }

    // n=6 connections
    if !n6_connections.is_empty() {
        doc.push_str("\\section{n=6 Connections}\n\n");
        doc.push_str("\\begin{itemize}\n");
        for conn in n6_connections {
            doc.push_str(&format!("\\item {}\n", escape_latex(conn)));
        }
        doc.push_str("\\end{itemize}\n\n");
    }

    // Predictions
    if !predictions.is_empty() {
        doc.push_str("\\section{Testable Predictions}\n\n");
        doc.push_str("\\begin{enumerate}\n");
        for pred in predictions {
            doc.push_str(&format!("\\item {}\n", escape_latex(pred)));
        }
        doc.push_str("\\end{enumerate}\n\n");
    }

    // End
    doc.push_str("\\end{document}\n");

    doc
}

/// Escape special LaTeX characters.
fn escape_latex(text: &str) -> String {
    text.replace('\\', "\\textbackslash{}")
        .replace('&', "\\&")
        .replace('%', "\\%")
        .replace('$', "\\$")
        .replace('#', "\\#")
        .replace('_', "\\_")
        .replace('{', "\\{")
        .replace('}', "\\}")
        .replace('~', "\\textasciitilde{}")
        .replace('^', "\\textasciicircum{}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_latex_basic() {
        let doc = generate_latex(
            "Test Discovery",
            "We found n=6 patterns.",
            &[("sigma".to_string(), 12.0), ("tau".to_string(), 4.0)],
            &["sigma matches 12.0".to_string()],
            &["Prediction 1".to_string()],
        );
        assert!(doc.contains("\\documentclass{article}"));
        assert!(doc.contains("\\begin{document}"));
        assert!(doc.contains("\\end{document}"));
        assert!(doc.contains("Test Discovery"));
        assert!(doc.contains("\\begin{abstract}"));
    }

    #[test]
    fn test_generate_latex_findings_table() {
        let doc = generate_latex(
            "Findings",
            "Abstract",
            &[("n".to_string(), 6.0), ("J2".to_string(), 24.0)],
            &[],
            &[],
        );
        assert!(doc.contains("\\begin{tabular}"));
        assert!(doc.contains("EXACT"));
    }

    #[test]
    fn test_generate_latex_empty() {
        let doc = generate_latex("Empty", "No data.", &[], &[], &[]);
        assert!(doc.contains("\\begin{document}"));
        assert!(doc.contains("\\end{document}"));
        assert!(!doc.contains("\\begin{tabular}"));
    }

    #[test]
    fn test_escape_latex() {
        assert_eq!(escape_latex("a & b"), "a \\& b");
        assert_eq!(escape_latex("100%"), "100\\%");
        assert_eq!(escape_latex("$x$"), "\\$x\\$");
    }

    #[test]
    fn test_latex_has_n6_theorem() {
        let doc = generate_latex("T", "A", &[], &[], &[]);
        assert!(doc.contains("\\sigma(n)"));
        assert!(doc.contains("\\varphi(n)"));
    }
}
