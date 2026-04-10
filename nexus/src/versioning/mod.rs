//! Version tracking for lens definitions and scan protocols.
/// Discovery version control — git-like versioning for discoveries.
/// No external crates.

#[derive(Debug, Clone)]
pub struct DiscoveryVersion {
    pub id: String,
    pub version: usize,
    pub timestamp: String,
    pub content: String,
    pub parent: Option<String>,
    pub branch: String,
}

pub struct VersionStore {
    versions: Vec<DiscoveryVersion>,
    path: String,
}

impl VersionStore {
    pub fn new(path: &str) -> Self {
        Self {
            versions: Vec::new(),
            path: path.to_string(),
        }
    }

    /// Commit a new version for the given discovery id.
    /// Returns the new version number.
    pub fn commit(&mut self, id: &str, content: &str) -> usize {
        let existing: Vec<&DiscoveryVersion> = self.versions.iter()
            .filter(|v| v.id == id && v.branch == "main")
            .collect();

        let version = existing.len() + 1;
        let parent = existing.last().map(|v| format!("{}@v{}", v.id, v.version));

        let ts = format!("2026-04-03T{:02}:{:02}:00Z", version / 60, version % 60);

        self.versions.push(DiscoveryVersion {
            id: id.to_string(),
            version,
            timestamp: ts,
            content: content.to_string(),
            parent,
            branch: "main".to_string(),
        });

        version
    }

    /// Compute a line-by-line diff between two versions of a discovery.
    pub fn diff(&self, id: &str, v1: usize, v2: usize) -> String {
        let content1 = self.versions.iter()
            .find(|v| v.id == id && v.version == v1)
            .map(|v| v.content.as_str())
            .unwrap_or("");
        let content2 = self.versions.iter()
            .find(|v| v.id == id && v.version == v2)
            .map(|v| v.content.as_str())
            .unwrap_or("");

        let lines1: Vec<&str> = content1.lines().collect();
        let lines2: Vec<&str> = content2.lines().collect();

        let mut result = String::new();
        result.push_str(&format!("--- {}@v{}\n", id, v1));
        result.push_str(&format!("+++ {}@v{}\n", id, v2));

        let max_len = lines1.len().max(lines2.len());
        for i in 0..max_len {
            let l1 = lines1.get(i).copied().unwrap_or("");
            let l2 = lines2.get(i).copied().unwrap_or("");
            if l1 != l2 {
                if !l1.is_empty() {
                    result.push_str(&format!("- {}\n", l1));
                }
                if !l2.is_empty() {
                    result.push_str(&format!("+ {}\n", l2));
                }
            } else {
                result.push_str(&format!("  {}\n", l1));
            }
        }

        result
    }

    /// Create a branch from the latest version of a discovery.
    /// Returns a description of the branch point.
    pub fn branch(&mut self, id: &str, branch_name: &str) -> String {
        let latest = self.versions.iter()
            .filter(|v| v.id == id && v.branch == "main")
            .last();

        match latest {
            Some(v) => {
                let branch_ver = DiscoveryVersion {
                    id: id.to_string(),
                    version: v.version,
                    timestamp: v.timestamp.clone(),
                    content: v.content.clone(),
                    parent: Some(format!("{}@v{}", v.id, v.version)),
                    branch: branch_name.to_string(),
                };
                let desc = format!("branch '{}' from {}@v{}", branch_name, id, v.version);
                self.versions.push(branch_ver);
                desc
            }
            None => {
                format!("no versions found for '{}'", id)
            }
        }
    }

    /// Merge two branches. Returns a description of the merge.
    /// Currently a simplified merge that records a merge commit on main.
    pub fn merge(&mut self, branch1: &str, branch2: &str) -> String {
        let latest_b1 = self.versions.iter()
            .filter(|v| v.branch == branch1)
            .last()
            .cloned();
        let latest_b2 = self.versions.iter()
            .filter(|v| v.branch == branch2)
            .last()
            .cloned();

        match (latest_b1, latest_b2) {
            (Some(v1), Some(v2)) => {
                let merged_content = format!("{}\n---merged---\n{}", v1.content, v2.content);
                let id = v1.id.clone();
                let main_count = self.versions.iter()
                    .filter(|v| v.id == id && v.branch == "main")
                    .count();
                let new_version = main_count + 1;

                self.versions.push(DiscoveryVersion {
                    id: id.clone(),
                    version: new_version,
                    timestamp: format!("2026-04-03T12:{:02}:00Z", new_version),
                    content: merged_content,
                    parent: Some(format!("merge({}@{}, {}@{})", v1.id, branch1, v2.id, branch2)),
                    branch: "main".to_string(),
                });

                format!("merged {} + {} -> {}@v{}", branch1, branch2, id, new_version)
            }
            _ => {
                format!("cannot merge: missing branch '{}' or '{}'", branch1, branch2)
            }
        }
    }

    /// Return the full version history for a discovery, ordered by version.
    pub fn history(&self, id: &str) -> Vec<&DiscoveryVersion> {
        let mut hist: Vec<&DiscoveryVersion> = self.versions.iter()
            .filter(|v| v.id == id)
            .collect();
        hist.sort_by_key(|v| v.version);
        hist
    }

    /// Get the store path.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Total number of stored versions.
    pub fn len(&self) -> usize {
        self.versions.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.versions.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commit_and_history() {
        let mut store = VersionStore::new("/tmp/test_versioning");
        assert_eq!(store.commit("disc-1", "initial content"), 1);
        assert_eq!(store.commit("disc-1", "updated content"), 2);
        assert_eq!(store.commit("disc-2", "other discovery"), 1);

        let hist = store.history("disc-1");
        assert_eq!(hist.len(), 2);
        assert_eq!(hist[0].version, 1);
        assert_eq!(hist[1].version, 2);
        assert_eq!(hist[1].content, "updated content");
    }

    #[test]
    fn test_diff() {
        let mut store = VersionStore::new("/tmp/test_diff");
        store.commit("d1", "line1\nline2\nline3");
        store.commit("d1", "line1\nchanged\nline3");

        let diff = store.diff("d1", 1, 2);
        assert!(diff.contains("- line2"));
        assert!(diff.contains("+ changed"));
    }

    #[test]
    fn test_branch_and_merge() {
        let mut store = VersionStore::new("/tmp/test_branch");
        store.commit("d1", "base content");
        let branch_desc = store.branch("d1", "experiment");
        assert!(branch_desc.contains("experiment"));
        assert!(branch_desc.contains("v1"));

        let merge_desc = store.merge("main", "experiment");
        assert!(merge_desc.contains("merged"));
    }
}
