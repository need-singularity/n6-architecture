use crate::graph::persistence::DiscoveryGraph;
use crate::telescope::registry::{LensCategory, LensRegistry};
use crate::telescope::domain_combos;

/// Render an ASCII dashboard summarizing the NEXUS-6 engine state.
pub fn render_dashboard() -> String {
    let registry = LensRegistry::new();
    let combos = domain_combos::default_combos();
    let graph = DiscoveryGraph::new(); // fresh; in production would load from disk

    let core_count = registry.by_category(LensCategory::Core).len();
    let combo_count = combos.len();
    let ext_count = registry.by_category(LensCategory::Extended).len();
    let custom_count = registry.by_category(LensCategory::Custom).len();
    let total_lenses = registry.len();

    let node_count = graph.nodes.len();
    let edge_count = graph.edges.len();

    // Coverage: fraction of core lenses that are registered
    let target_core = 22;
    let coverage_pct = ((core_count as f64 / target_core as f64) * 100.0).min(100.0) as usize;
    let coverage_bar = progress_bar(coverage_pct, 10);

    let mut out = String::new();

    out.push_str("╔══════════════════════════════════════════════╗\n");
    out.push_str("║           NEXUS-6 Discovery Engine           ║\n");
    out.push_str("║           ════════════════════════            ║\n");
    out.push_str("╠══════════════════════════════════════════════╣\n");
    out.push_str(&format!(
        "║  Lenses: {:>2} core | {:>2} combos | {:>2} ext | {:>2} cust  ║\n",
        core_count, combo_count, ext_count, custom_count
    ));
    out.push_str(&format!(
        "║  Total:  {:>3} registered                       ║\n",
        total_lenses
    ));
    out.push_str(&format!(
        "║  Graph:  {:>4} nodes | {:>4} edges              ║\n",
        node_count, edge_count
    ));
    out.push_str(&format!(
        "║  Status: [{}] {:>3}% coverage        ║\n",
        coverage_bar, coverage_pct
    ));
    out.push_str("╠══════════════════════════════════════════════╣\n");
    out.push_str("║  n=6 Constants:                              ║\n");
    out.push_str("║    sigma=12  phi=2  tau=4  J2=24  sopfr=5    ║\n");
    out.push_str("║    sigma*phi=n*tau  <=>  n=6 (PROVED)        ║\n");
    out.push_str("╠══════════════════════════════════════════════╣\n");
    out.push_str("║  Domain Combos (10):                         ║\n");
    for combo in &combos {
        let lenses_str = combo.lenses.join("+");
        let line = format!("║    {:<14} {:<28} ║\n", combo.name, lenses_str);
        out.push_str(&line);
    }
    out.push_str("╠══════════════════════════════════════════════╣\n");
    out.push_str("║  Modules:                                    ║\n");
    out.push_str("║    telescope  graph     history   verifier   ║\n");
    out.push_str("║    encoder    gpu       materials ouroboros   ║\n");
    out.push_str("╚══════════════════════════════════════════════╝\n");

    out
}

/// Build an ASCII progress bar: e.g. "████████░░" for 80% with width=10.
fn progress_bar(pct: usize, width: usize) -> String {
    let filled = (pct * width / 100).min(width);
    let empty = width - filled;
    let mut bar = String::with_capacity(width * 3);
    for _ in 0..filled {
        bar.push('\u{2588}'); // █
    }
    for _ in 0..empty {
        bar.push('\u{2591}'); // ░
    }
    bar
}

/// Render a self-contained HTML dashboard with inline CSS + JS + SVG.
pub fn render_html_dashboard() -> String {
    let registry = LensRegistry::new();
    let combos = domain_combos::default_combos();
    let graph = DiscoveryGraph::new();

    let core_count = registry.by_category(LensCategory::Core).len();
    let ext_count = registry.by_category(LensCategory::Extended).len();
    let custom_count = registry.by_category(LensCategory::Custom).len();
    let total_lenses = registry.len();
    let combo_count = combos.len();
    let node_count = graph.nodes.len();
    let edge_count = graph.edges.len();

    // Build lens table JSON data for the search/filter JS
    let all_entries: Vec<_> = registry.iter().collect();
    let mut lens_rows = String::new();
    for (name, entry) in &all_entries {
        let cat = match entry.category {
            LensCategory::Core => "Core",
            LensCategory::DomainCombo => "Combo",
            LensCategory::Extended => "Extended",
            LensCategory::Custom => "Custom",
        };
        let desc_escaped = entry.description.replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', " ");
        let domains = entry.domain_affinity.join(", ");
        lens_rows.push_str(&format!(
            "{{\"name\":\"{}\",\"category\":\"{}\",\"description\":\"{}\",\"domains\":\"{}\"}},\n",
            name, cat, desc_escaped, domains
        ));
    }

    // SVG donut chart segments
    let categories = [
        ("Core", core_count, "#4CAF50"),
        ("Extended", ext_count, "#2196F3"),
        ("Custom", custom_count, "#FF9800"),
    ];
    let donut_segments = build_donut_svg(&categories, total_lenses);

    let mut html = String::with_capacity(16384);

    html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"UTF-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str("<title>NEXUS-6 Discovery Engine Dashboard</title>\n");
    html.push_str("<style>\n");
    html.push_str(CSS);
    html.push_str("\n</style>\n</head>\n<body>\n");

    // Header
    html.push_str("<header>\n");
    html.push_str("<h1>NEXUS-6 Discovery Engine</h1>\n");
    html.push_str("<p class=\"subtitle\">\u{03c3}(n)\u{00b7}\u{03c6}(n) = n\u{00b7}\u{03c4}(n) \u{21d4} n = 6</p>\n");
    html.push_str("</header>\n");

    // Status cards
    html.push_str("<section class=\"cards\">\n");
    html.push_str(&format!(
        "<div class=\"card\"><div class=\"card-num\">{}</div><div class=\"card-label\">Total Lenses</div></div>\n",
        total_lenses
    ));
    html.push_str(&format!(
        "<div class=\"card\"><div class=\"card-num\">{}</div><div class=\"card-label\">Core</div></div>\n",
        core_count
    ));
    html.push_str(&format!(
        "<div class=\"card\"><div class=\"card-num\">{}</div><div class=\"card-label\">Domain Combos</div></div>\n",
        combo_count
    ));
    html.push_str(&format!(
        "<div class=\"card\"><div class=\"card-num\">{}</div><div class=\"card-label\">Graph Nodes</div></div>\n",
        node_count
    ));
    html.push_str(&format!(
        "<div class=\"card\"><div class=\"card-num\">{}</div><div class=\"card-label\">Graph Edges</div></div>\n",
        edge_count
    ));
    html.push_str("</section>\n");

    // Donut chart + constants
    html.push_str("<section class=\"chart-section\">\n");
    html.push_str("<div class=\"chart-container\">\n");
    html.push_str("<h2>Lens Categories</h2>\n");
    html.push_str(&donut_segments);
    html.push_str("<div class=\"legend\">\n");
    for (label, count, color) in &categories {
        html.push_str(&format!(
            "<span class=\"legend-item\"><span class=\"swatch\" style=\"background:{}\"></span>{}: {}</span>\n",
            color, label, count
        ));
    }
    html.push_str("</div>\n</div>\n");

    html.push_str("<div class=\"constants\">\n");
    html.push_str("<h2>n=6 Constants</h2>\n");
    html.push_str("<table class=\"const-table\">\n");
    html.push_str("<tr><td>\u{03c3}</td><td>12</td><td>\u{03c6}</td><td>2</td></tr>\n");
    html.push_str("<tr><td>\u{03c4}</td><td>4</td><td>J\u{2082}</td><td>24</td></tr>\n");
    html.push_str("<tr><td>sopfr</td><td>5</td><td>\u{03bc}</td><td>1</td></tr>\n");
    html.push_str("<tr><td>\u{03c3}-\u{03c6}</td><td>10</td><td>\u{03c3}-\u{03c4}</td><td>8</td></tr>\n");
    html.push_str("</table>\n</div>\n");
    html.push_str("</section>\n");

    // Search + lens table
    html.push_str("<section class=\"table-section\">\n");
    html.push_str("<h2>Lens Registry</h2>\n");
    html.push_str("<div class=\"filter-bar\">\n");
    html.push_str("<input type=\"text\" id=\"searchBox\" placeholder=\"Search lenses...\" oninput=\"filterTable()\">\n");
    html.push_str("<select id=\"catFilter\" onchange=\"filterTable()\">\n");
    html.push_str("<option value=\"\">All Categories</option>\n");
    html.push_str("<option value=\"Core\">Core</option>\n");
    html.push_str("<option value=\"Extended\">Extended</option>\n");
    html.push_str("<option value=\"Custom\">Custom</option>\n");
    html.push_str("<option value=\"Combo\">Combo</option>\n");
    html.push_str("</select>\n");
    html.push_str("<span id=\"resultCount\"></span>\n");
    html.push_str("</div>\n");
    html.push_str("<table id=\"lensTable\">\n");
    html.push_str("<thead><tr><th>#</th><th>Name</th><th>Category</th><th>Description</th><th>Domains</th></tr></thead>\n");
    html.push_str("<tbody id=\"lensBody\"></tbody>\n");
    html.push_str("</table>\n");
    html.push_str("</section>\n");

    // Footer
    html.push_str("<footer>NEXUS-6 Discovery Engine &mdash; \u{03c3}(n)\u{00b7}\u{03c6}(n) = n\u{00b7}\u{03c4}(n) &hArr; n = 6 (PROVED)</footer>\n");

    // JavaScript
    html.push_str("<script>\n");
    html.push_str("const LENSES = [\n");
    html.push_str(&lens_rows);
    html.push_str("];\n");
    html.push_str(JS);
    html.push_str("\n</script>\n");

    html.push_str("</body>\n</html>\n");

    html
}

/// Build an SVG donut chart from category slices.
fn build_donut_svg(categories: &[(&str, usize, &str)], total: usize) -> String {
    if total == 0 {
        return "<svg width=\"200\" height=\"200\"></svg>".to_string();
    }

    let cx = 100.0_f64;
    let cy = 100.0_f64;
    let r = 80.0_f64;
    let stroke_w = 30.0_f64;
    let circumference = 2.0 * std::f64::consts::PI * r;

    let mut svg = String::new();
    svg.push_str("<svg width=\"220\" height=\"220\" viewBox=\"-10 -10 220 220\">\n");

    let mut offset = 0.0_f64;
    for (_, count, color) in categories {
        if *count == 0 { continue; }
        let frac = *count as f64 / total as f64;
        let dash_len = frac * circumference;
        let gap_len = circumference - dash_len;
        svg.push_str(&format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\" \
             stroke-dasharray=\"{:.2} {:.2}\" stroke-dashoffset=\"{:.2}\" />\n",
            cx, cy, r, color, stroke_w, dash_len, gap_len, -offset
        ));
        offset += dash_len;
    }

    // Center text
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" dominant-baseline=\"central\" \
         font-size=\"24\" font-weight=\"bold\" fill=\"#333\">{}</text>\n",
        cx, cy, total
    ));

    svg.push_str("</svg>\n");
    svg
}

const CSS: &str = r#"
* { margin: 0; padding: 0; box-sizing: border-box; }
body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
       background: #0d1117; color: #c9d1d9; line-height: 1.6; }
header { background: linear-gradient(135deg, #161b22, #1a2332); padding: 2rem; text-align: center;
         border-bottom: 1px solid #30363d; }
h1 { color: #58a6ff; font-size: 1.8rem; }
h2 { color: #58a6ff; margin-bottom: 0.8rem; font-size: 1.2rem; }
.subtitle { color: #8b949e; font-size: 1rem; margin-top: 0.3rem; }
.cards { display: flex; gap: 1rem; padding: 1.5rem; flex-wrap: wrap; justify-content: center; }
.card { background: #161b22; border: 1px solid #30363d; border-radius: 8px; padding: 1.2rem 1.5rem;
        text-align: center; min-width: 120px; }
.card-num { font-size: 2rem; font-weight: bold; color: #58a6ff; }
.card-label { color: #8b949e; font-size: 0.85rem; }
.chart-section { display: flex; gap: 2rem; padding: 1.5rem; flex-wrap: wrap; justify-content: center; }
.chart-container { background: #161b22; border: 1px solid #30363d; border-radius: 8px; padding: 1.5rem;
                   text-align: center; }
.legend { display: flex; gap: 1rem; justify-content: center; margin-top: 1rem; flex-wrap: wrap; }
.legend-item { display: flex; align-items: center; gap: 0.3rem; font-size: 0.85rem; }
.swatch { width: 12px; height: 12px; border-radius: 2px; display: inline-block; }
.constants { background: #161b22; border: 1px solid #30363d; border-radius: 8px; padding: 1.5rem; }
.const-table { border-collapse: collapse; }
.const-table td { padding: 0.4rem 1rem; }
.const-table td:nth-child(odd) { color: #58a6ff; font-weight: bold; }
.const-table td:nth-child(even) { color: #f0f6fc; font-family: monospace; font-size: 1.1rem; }
.table-section { padding: 1.5rem; }
.filter-bar { display: flex; gap: 1rem; margin-bottom: 1rem; align-items: center; flex-wrap: wrap; }
#searchBox { background: #0d1117; border: 1px solid #30363d; color: #c9d1d9; padding: 0.5rem 1rem;
             border-radius: 6px; font-size: 0.9rem; width: 260px; }
#catFilter { background: #0d1117; border: 1px solid #30363d; color: #c9d1d9; padding: 0.5rem;
             border-radius: 6px; }
#resultCount { color: #8b949e; font-size: 0.85rem; }
#lensTable { width: 100%; border-collapse: collapse; }
#lensTable th { background: #161b22; color: #58a6ff; padding: 0.6rem; text-align: left;
                border-bottom: 2px solid #30363d; font-size: 0.85rem; position: sticky; top: 0; }
#lensTable td { padding: 0.5rem 0.6rem; border-bottom: 1px solid #21262d; font-size: 0.8rem; }
#lensTable tbody tr:hover { background: #161b22; }
footer { text-align: center; padding: 1.5rem; color: #484f58; font-size: 0.8rem;
         border-top: 1px solid #30363d; margin-top: 2rem; }
"#;

const JS: &str = r#"
function renderTable(data) {
    const body = document.getElementById('lensBody');
    body.innerHTML = '';
    data.forEach(function(lens, i) {
        const tr = document.createElement('tr');
        tr.innerHTML = '<td>' + (i+1) + '</td><td>' + lens.name + '</td><td>' +
            lens.category + '</td><td>' + lens.description + '</td><td>' + lens.domains + '</td>';
        body.appendChild(tr);
    });
    document.getElementById('resultCount').textContent = data.length + ' / ' + LENSES.length + ' lenses';
}
function filterTable() {
    const q = document.getElementById('searchBox').value.toLowerCase();
    const cat = document.getElementById('catFilter').value;
    const filtered = LENSES.filter(function(l) {
        const matchCat = !cat || l.category === cat;
        const matchQ = !q || l.name.toLowerCase().indexOf(q) >= 0
            || l.description.toLowerCase().indexOf(q) >= 0
            || l.domains.toLowerCase().indexOf(q) >= 0;
        return matchCat && matchQ;
    });
    renderTable(filtered);
}
renderTable(LENSES);
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_bar() {
        let bar = progress_bar(80, 10);
        assert_eq!(bar.chars().count(), 10);
        assert_eq!(bar.chars().filter(|&c| c == '\u{2588}').count(), 8);
    }

    #[test]
    fn test_render_dashboard_contains_header() {
        let out = render_dashboard();
        assert!(out.contains("NEXUS-6 Discovery Engine"));
        assert!(out.contains("sigma=12"));
    }

    #[test]
    fn test_render_html_dashboard_structure() {
        let html = render_html_dashboard();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("NEXUS-6 Discovery Engine"));
        assert!(html.contains("<svg"));
        assert!(html.contains("filterTable"));
        assert!(html.contains("lensTable"));
        assert!(html.contains("</html>"));
    }

    #[test]
    fn test_render_html_dashboard_has_lens_data() {
        let html = render_html_dashboard();
        assert!(html.contains("const LENSES = ["));
        // Should have at least the core lenses in data
        assert!(html.contains("\"category\":\"Core\"") || html.contains("\"category\":\"Extended\""));
    }

    #[test]
    fn test_build_donut_svg_empty() {
        let svg = build_donut_svg(&[], 0);
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn test_build_donut_svg_segments() {
        let cats = [("A", 50, "#f00"), ("B", 30, "#0f0"), ("C", 20, "#00f")];
        let svg = build_donut_svg(&cats, 100);
        assert!(svg.contains("circle"));
        assert!(svg.contains("#f00"));
        assert!(svg.contains("100")); // total in center text
    }
}
