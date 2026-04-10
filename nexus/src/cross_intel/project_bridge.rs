/// Cross-project bridge — links discoveries across TECS-L family repos.

use std::collections::HashMap;

/// A reference to a discovery in another project.
#[derive(Debug, Clone)]
pub struct ProjectRef {
    pub project: String,
    pub path: String,
    pub discovery_id: String,
    pub relevance: f64,
}

/// Bridge connecting n6-architecture findings to other TECS-L repos.
pub struct ProjectBridge {
    refs: Vec<ProjectRef>,
    index: HashMap<String, Vec<usize>>,
}

impl ProjectBridge {
    pub fn new() -> Self {
        Self {
            refs: Vec::new(),
            index: HashMap::new(),
        }
    }

    /// Register a cross-project reference.
    pub fn add_ref(&mut self, pr: ProjectRef) {
        let idx = self.refs.len();
        self.index
            .entry(pr.project.clone())
            .or_default()
            .push(idx);
        self.refs.push(pr);
    }

    /// Find references by project name.
    pub fn by_project(&self, project: &str) -> Vec<&ProjectRef> {
        self.index
            .get(project)
            .map(|idxs| idxs.iter().map(|&i| &self.refs[i]).collect())
            .unwrap_or_default()
    }

    /// All references above a relevance threshold.
    pub fn above_relevance(&self, threshold: f64) -> Vec<&ProjectRef> {
        self.refs.iter().filter(|r| r.relevance >= threshold).collect()
    }

    /// Total reference count.
    pub fn len(&self) -> usize {
        self.refs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.refs.is_empty()
    }
}

impl Default for ProjectBridge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge_basic() {
        let mut bridge = ProjectBridge::new();
        bridge.add_ref(ProjectRef {
            project: "tecs-l".into(),
            path: "docs/hypotheses/".into(),
            discovery_id: "BT-56".into(),
            relevance: 0.95,
        });
        bridge.add_ref(ProjectRef {
            project: "anima".into(),
            path: "engine/".into(),
            discovery_id: "H-ANI-1".into(),
            relevance: 0.7,
        });
        assert_eq!(bridge.len(), 2);
        assert_eq!(bridge.by_project("tecs-l").len(), 1);
        assert_eq!(bridge.above_relevance(0.9).len(), 1);
    }
}
