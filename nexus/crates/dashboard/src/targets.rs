use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DimensionTarget {
    pub key: String,
    pub label: String,
    pub target: f64,
    pub color: String,
}

pub fn all_targets() -> Vec<DimensionTarget> {
    vec![
        dt("performance",     "Performance",   10_000.0, "#ff6384"),
        dt("architecture",    "Architecture",     100.0, "#36a2eb"),
        dt("lenses",          "Lenses",           200.0, "#ffce56"),
        dt("modules",         "Modules",            4.0, "#4bc0c0"),
        dt("tests",           "Tests",           1000.0, "#9966ff"),
        dt("hypotheses",      "Hypotheses",       150.0, "#ff9f40"),
        dt("dse",             "DSE",              322.0, "#e7e9ed"),
        dt("experiments",     "Experiments",       50.0, "#c9cbcf"),
        dt("calculators",     "Calculators",       50.0, "#7cb342"),
        dt("cross_resonance", "CrossReson",       100.0, "#e91e63"),
        dt("knowledge_graph", "KnowledgeGr",      500.0, "#00bcd4"),
        dt("red_team",        "RedTeam",          100.0, "#ff5722"),
        dt("atlas",           "Atlas",           2000.0, "#795548"),
        dt("documentation",   "Docs",              90.0, "#607d8b"),
        dt("integration",     "Integration",       50.0, "#8bc34a"),
    ]
}

fn dt(key: &str, label: &str, target: f64, color: &str) -> DimensionTarget {
    DimensionTarget {
        key: key.to_string(),
        label: label.to_string(),
        target,
        color: color.to_string(),
    }
}
