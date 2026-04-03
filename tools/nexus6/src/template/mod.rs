//! Code generation templates for lenses, tests, and experiments.
/// Experiment template store — reusable experiment blueprints.
/// No external crates.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ExperimentTemplate {
    pub name: String,
    pub description: String,
    pub steps: Vec<String>,
    pub parameters: HashMap<String, String>,
    pub expected_metrics: Vec<String>,
}

pub struct TemplateStore {
    templates: Vec<ExperimentTemplate>,
    path: String,
}

impl TemplateStore {
    pub fn new(path: &str) -> Self {
        Self {
            templates: Vec::new(),
            path: path.to_string(),
        }
    }

    /// Save a template to the store. Overwrites if name already exists.
    pub fn save_template(&mut self, template: ExperimentTemplate) {
        // Remove existing with same name
        self.templates.retain(|t| t.name != template.name);
        self.templates.push(template);
    }

    /// Load a template by name.
    pub fn load_template(&self, name: &str) -> Option<&ExperimentTemplate> {
        self.templates.iter().find(|t| t.name == name)
    }

    /// List all templates.
    pub fn list(&self) -> &[ExperimentTemplate] {
        &self.templates
    }

    /// Instantiate a template with parameter substitutions.
    /// Replaces `{{key}}` patterns in steps and description with provided values.
    /// Falls back to template defaults for unspecified keys.
    pub fn instantiate(&self, name: &str, params: &HashMap<String, String>) -> Option<ExperimentTemplate> {
        let template = self.load_template(name)?;

        // Merge: provided params override template defaults
        let mut merged = template.parameters.clone();
        for (k, v) in params {
            merged.insert(k.clone(), v.clone());
        }

        let substitute = |text: &str, params: &HashMap<String, String>| -> String {
            let mut result = text.to_string();
            for (k, v) in params {
                let pattern = format!("{{{{{}}}}}", k);
                result = result.replace(&pattern, v);
            }
            result
        };

        let steps: Vec<String> = template.steps.iter()
            .map(|s| substitute(s, &merged))
            .collect();

        let description = substitute(&template.description, &merged);

        Some(ExperimentTemplate {
            name: format!("{}-instance", template.name),
            description,
            steps,
            parameters: merged,
            expected_metrics: template.expected_metrics.clone(),
        })
    }

    /// Get store path.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Number of templates.
    pub fn len(&self) -> usize {
        self.templates.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.templates.is_empty()
    }
}

/// Built-in n=6 templates for common experiment patterns.
pub fn builtin_templates() -> Vec<ExperimentTemplate> {
    let mut templates = Vec::new();

    // 1. Basic lens scan template
    let mut params1 = HashMap::new();
    params1.insert("domain".to_string(), "physics".to_string());
    params1.insert("lenses".to_string(), "consciousness,topology,causal".to_string());
    templates.push(ExperimentTemplate {
        name: "n6-basic-scan".to_string(),
        description: "Basic 3-lens scan on {{domain}}".to_string(),
        steps: vec![
            "Initialize telescope with lenses: {{lenses}}".to_string(),
            "Run scan on domain: {{domain}}".to_string(),
            "Collect n=6 resonance hits".to_string(),
            "Report consensus level".to_string(),
        ],
        parameters: params1,
        expected_metrics: vec!["n6_score".to_string(), "consensus".to_string(), "discovery_count".to_string()],
    });

    // 2. Full 22-lens scan
    let mut params2 = HashMap::new();
    params2.insert("domain".to_string(), "all".to_string());
    templates.push(ExperimentTemplate {
        name: "n6-full-scan".to_string(),
        description: "Full 22-lens scan on {{domain}}".to_string(),
        steps: vec![
            "Load all 22 lenses from registry".to_string(),
            "Run parallel scan on {{domain}}".to_string(),
            "Cross-validate with 3+ lens consensus".to_string(),
            "Grade: 3+=candidate, 7+=high-confidence, 12+=confirmed".to_string(),
        ],
        parameters: params2,
        expected_metrics: vec!["n6_score".to_string(), "consensus".to_string(), "lens_agreement".to_string()],
    });

    // 3. OUROBOROS evolution
    let mut params3 = HashMap::new();
    params3.insert("domain".to_string(), "discovery".to_string());
    params3.insert("cycles".to_string(), "6".to_string());
    templates.push(ExperimentTemplate {
        name: "n6-ouroboros".to_string(),
        description: "OUROBOROS evolution on {{domain}} for {{cycles}} cycles".to_string(),
        steps: vec![
            "Seed initial hypotheses for {{domain}}".to_string(),
            "Run {{cycles}} evolution cycles".to_string(),
            "Forge new lenses from discoveries".to_string(),
            "Report convergence and new BT candidates".to_string(),
        ],
        parameters: params3,
        expected_metrics: vec!["discoveries".to_string(), "convergence".to_string(), "lenses_forged".to_string()],
    });

    templates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_and_load() {
        let mut store = TemplateStore::new("/tmp/test_templates");
        let tmpl = ExperimentTemplate {
            name: "test-scan".to_string(),
            description: "A test template".to_string(),
            steps: vec!["step 1".to_string(), "step 2".to_string()],
            parameters: HashMap::new(),
            expected_metrics: vec!["metric_a".to_string()],
        };
        store.save_template(tmpl);
        assert_eq!(store.len(), 1);
        let loaded = store.load_template("test-scan").unwrap();
        assert_eq!(loaded.steps.len(), 2);
    }

    #[test]
    fn test_instantiate() {
        let mut store = TemplateStore::new("/tmp/test_inst");
        let mut params = HashMap::new();
        params.insert("domain".to_string(), "default_domain".to_string());
        let tmpl = ExperimentTemplate {
            name: "scan".to_string(),
            description: "Scan {{domain}}".to_string(),
            steps: vec!["Run on {{domain}}".to_string()],
            parameters: params,
            expected_metrics: vec![],
        };
        store.save_template(tmpl);

        let mut override_params = HashMap::new();
        override_params.insert("domain".to_string(), "biology".to_string());
        let instance = store.instantiate("scan", &override_params).unwrap();
        assert_eq!(instance.description, "Scan biology");
        assert_eq!(instance.steps[0], "Run on biology");
    }

    #[test]
    fn test_builtin_templates() {
        let builtins = builtin_templates();
        assert_eq!(builtins.len(), 3);
        assert_eq!(builtins[0].name, "n6-basic-scan");
        assert_eq!(builtins[1].name, "n6-full-scan");
        assert_eq!(builtins[2].name, "n6-ouroboros");
    }

    #[test]
    fn test_overwrite() {
        let mut store = TemplateStore::new("/tmp/test_overwrite");
        store.save_template(ExperimentTemplate {
            name: "dup".to_string(),
            description: "v1".to_string(),
            steps: vec![],
            parameters: HashMap::new(),
            expected_metrics: vec![],
        });
        store.save_template(ExperimentTemplate {
            name: "dup".to_string(),
            description: "v2".to_string(),
            steps: vec![],
            parameters: HashMap::new(),
            expected_metrics: vec![],
        });
        assert_eq!(store.len(), 1);
        assert_eq!(store.load_template("dup").unwrap().description, "v2");
    }
}
