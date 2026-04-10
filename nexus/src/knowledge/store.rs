use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;

/// A single entry in the knowledge base.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KnowledgeEntry {
    pub id: String,
    pub entry_type: String, // "constant", "formula", "pattern", "law", "bt", "lens"
    pub content: String,
    pub metadata: HashMap<String, String>,
    pub created: String,
    pub confidence: f64,
    pub n6_score: f64,
    pub references: Vec<String>, // related entry IDs
    pub verified: bool,
    pub verified_by: Vec<String>, // lenses/experiments that verified this
}

/// Statistics about the knowledge base.
#[derive(Debug, Clone)]
pub struct KnowledgeStats {
    pub total: usize,
    pub by_type: HashMap<String, usize>,
    pub verified_count: usize,
    pub avg_confidence: f64,
    pub avg_n6_score: f64,
}

/// In-memory knowledge base backed by a JSONL file.
pub struct KnowledgeBase {
    entries: Vec<KnowledgeEntry>,
    path: String,
}

impl KnowledgeBase {
    /// Create a new empty knowledge base that will persist to `path`.
    pub fn new(path: &str) -> Self {
        Self {
            entries: Vec::new(),
            path: path.to_string(),
        }
    }

    /// Load from a JSONL file. Returns empty KB if file doesn't exist.
    pub fn load(path: &str) -> io::Result<Self> {
        if !Path::new(path).exists() {
            return Ok(Self::new(path));
        }

        let file = fs::File::open(path)?;
        let reader = io::BufReader::new(file);
        let mut entries = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let entry: KnowledgeEntry = serde_json::from_str(trimmed)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            entries.push(entry);
        }

        Ok(Self {
            entries,
            path: path.to_string(),
        })
    }

    /// Save all entries to JSONL (atomic: write to .tmp then rename).
    pub fn save(&self) -> io::Result<()> {
        let tmp_path = format!("{}.tmp", self.path);

        // Ensure parent directory exists
        if let Some(parent) = Path::new(&self.path).parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        let mut file = fs::File::create(&tmp_path)?;
        for entry in &self.entries {
            let json = serde_json::to_string(entry)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            writeln!(file, "{}", json)?;
        }
        file.flush()?;
        fs::rename(&tmp_path, &self.path)?;
        Ok(())
    }

    /// Add an entry to the knowledge base.
    pub fn add(&mut self, entry: KnowledgeEntry) {
        self.entries.push(entry);
    }

    /// Get an entry by ID.
    pub fn get(&self, id: &str) -> Option<&KnowledgeEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    /// Keyword search across content, entry_type, and metadata values.
    pub fn search(&self, query: &str) -> Vec<&KnowledgeEntry> {
        let lower_query = query.to_lowercase();
        self.entries
            .iter()
            .filter(|e| {
                e.content.to_lowercase().contains(&lower_query)
                    || e.entry_type.to_lowercase().contains(&lower_query)
                    || e.id.to_lowercase().contains(&lower_query)
                    || e.metadata.values().any(|v| v.to_lowercase().contains(&lower_query))
            })
            .collect()
    }

    /// Filter entries by type.
    pub fn by_type(&self, entry_type: &str) -> Vec<&KnowledgeEntry> {
        self.entries
            .iter()
            .filter(|e| e.entry_type == entry_type)
            .collect()
    }

    /// Return only verified entries.
    pub fn verified_only(&self) -> Vec<&KnowledgeEntry> {
        self.entries.iter().filter(|e| e.verified).collect()
    }

    /// Compute statistics about the knowledge base.
    pub fn stats(&self) -> KnowledgeStats {
        let total = self.entries.len();
        let mut by_type: HashMap<String, usize> = HashMap::new();
        let mut verified_count = 0;
        let mut sum_conf = 0.0_f64;
        let mut sum_n6 = 0.0_f64;

        for e in &self.entries {
            *by_type.entry(e.entry_type.clone()).or_insert(0) += 1;
            if e.verified {
                verified_count += 1;
            }
            sum_conf += e.confidence;
            sum_n6 += e.n6_score;
        }

        let avg_confidence = if total > 0 {
            sum_conf / total as f64
        } else {
            0.0
        };
        let avg_n6_score = if total > 0 {
            sum_n6 / total as f64
        } else {
            0.0
        };

        KnowledgeStats {
            total,
            by_type,
            verified_count,
            avg_confidence,
            avg_n6_score,
        }
    }

    /// Total number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the KB is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get all entries (read-only).
    pub fn entries(&self) -> &[KnowledgeEntry] {
        &self.entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(id: &str, etype: &str, content: &str, verified: bool) -> KnowledgeEntry {
        KnowledgeEntry {
            id: id.to_string(),
            entry_type: etype.to_string(),
            content: content.to_string(),
            metadata: HashMap::new(),
            created: "0s".to_string(),
            confidence: 0.9,
            n6_score: 0.8,
            references: Vec::new(),
            verified,
            verified_by: Vec::new(),
        }
    }

    #[test]
    fn test_add_and_get() {
        let mut kb = KnowledgeBase::new("/tmp/test_kb.jsonl");
        kb.add(make_entry("e1", "constant", "sigma=12", true));
        assert_eq!(kb.len(), 1);
        let got = kb.get("e1").unwrap();
        assert_eq!(got.content, "sigma=12");
    }

    #[test]
    fn test_search() {
        let mut kb = KnowledgeBase::new("/tmp/test_kb.jsonl");
        kb.add(make_entry("e1", "constant", "sigma=12 physics", true));
        kb.add(make_entry("e2", "pattern", "repeating cycle biology", false));
        let results = kb.search("physics");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "e1");
    }

    #[test]
    fn test_by_type() {
        let mut kb = KnowledgeBase::new("/tmp/test_kb.jsonl");
        kb.add(make_entry("e1", "constant", "a", true));
        kb.add(make_entry("e2", "pattern", "b", false));
        kb.add(make_entry("e3", "constant", "c", true));
        assert_eq!(kb.by_type("constant").len(), 2);
        assert_eq!(kb.by_type("pattern").len(), 1);
    }

    #[test]
    fn test_verified_only() {
        let mut kb = KnowledgeBase::new("/tmp/test_kb.jsonl");
        kb.add(make_entry("e1", "constant", "a", true));
        kb.add(make_entry("e2", "pattern", "b", false));
        assert_eq!(kb.verified_only().len(), 1);
    }

    #[test]
    fn test_stats() {
        let mut kb = KnowledgeBase::new("/tmp/test_kb.jsonl");
        kb.add(make_entry("e1", "constant", "a", true));
        kb.add(make_entry("e2", "pattern", "b", false));
        let s = kb.stats();
        assert_eq!(s.total, 2);
        assert_eq!(s.verified_count, 1);
        assert_eq!(*s.by_type.get("constant").unwrap(), 1);
    }

    #[test]
    fn test_save_and_load() {
        let path = "/tmp/nexus_test_kb_save_load.jsonl";
        // Clean up from previous runs
        let _ = std::fs::remove_file(path);

        let mut kb = KnowledgeBase::new(path);
        kb.add(make_entry("e1", "constant", "sigma=12", true));
        kb.add(make_entry("e2", "formula", "sigma*phi=n*tau", false));
        kb.save().unwrap();

        let loaded = KnowledgeBase::load(path).unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded.get("e1").unwrap().content, "sigma=12");
        assert_eq!(loaded.get("e2").unwrap().content, "sigma*phi=n*tau");

        // Clean up
        let _ = std::fs::remove_file(path);
    }
}
