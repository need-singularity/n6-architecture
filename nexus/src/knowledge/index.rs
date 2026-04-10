use std::collections::HashMap;

use super::store::KnowledgeBase;

/// A simple inverted index for fast keyword lookups.
pub struct KnowledgeIndex {
    /// Maps lowercase tokens to sets of entry IDs.
    token_to_ids: HashMap<String, Vec<String>>,
}

impl KnowledgeIndex {
    /// Build an index from a knowledge base.
    pub fn build(kb: &KnowledgeBase) -> Self {
        let mut token_to_ids: HashMap<String, Vec<String>> = HashMap::new();

        for entry in kb.entries() {
            let tokens = tokenize(&entry.content);
            for token in tokens {
                token_to_ids
                    .entry(token)
                    .or_default()
                    .push(entry.id.clone());
            }
            // Also index entry_type
            token_to_ids
                .entry(entry.entry_type.to_lowercase())
                .or_default()
                .push(entry.id.clone());
            // Index metadata values
            for val in entry.metadata.values() {
                for token in tokenize(val) {
                    token_to_ids
                        .entry(token)
                        .or_default()
                        .push(entry.id.clone());
                }
            }
        }

        Self { token_to_ids }
    }

    /// Find entry IDs matching a query (AND of all query tokens).
    pub fn lookup(&self, query: &str) -> Vec<String> {
        let query_tokens = tokenize(query);
        if query_tokens.is_empty() {
            return Vec::new();
        }

        // Get IDs for each token, then intersect
        let mut id_sets: Vec<std::collections::HashSet<&String>> = Vec::new();
        for token in &query_tokens {
            if let Some(ids) = self.token_to_ids.get(token) {
                id_sets.push(ids.iter().collect());
            } else {
                // Token not found -> no results
                return Vec::new();
            }
        }

        if id_sets.is_empty() {
            return Vec::new();
        }

        // Intersect all sets
        let mut result = id_sets[0].clone();
        for set in &id_sets[1..] {
            result = result.intersection(set).copied().collect();
        }

        result.into_iter().cloned().collect()
    }

    /// Number of unique tokens in the index.
    pub fn token_count(&self) -> usize {
        self.token_to_ids.len()
    }
}

/// Simple whitespace + punctuation tokenizer, returns lowercase tokens.
fn tokenize(text: &str) -> Vec<String> {
    text.split(|c: char| c.is_whitespace() || c == ',' || c == ';' || c == ':' || c == '(' || c == ')')
        .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric() && c != '_').to_lowercase())
        .filter(|s| !s.is_empty() && s.len() >= 2)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::knowledge::store::{KnowledgeBase, KnowledgeEntry};

    fn entry(id: &str, content: &str) -> KnowledgeEntry {
        KnowledgeEntry {
            id: id.to_string(),
            entry_type: "constant".to_string(),
            content: content.to_string(),
            metadata: std::collections::HashMap::new(),
            created: "0s".to_string(),
            confidence: 0.9,
            n6_score: 0.8,
            references: Vec::new(),
            verified: true,
            verified_by: Vec::new(),
        }
    }

    #[test]
    fn test_index_build_and_lookup() {
        let mut kb = KnowledgeBase::new("/tmp/test_idx.jsonl");
        kb.add(entry("e1", "sigma equals twelve physics"));
        kb.add(entry("e2", "tau equals four biology"));
        kb.add(entry("e3", "sigma and tau combined physics"));

        let idx = KnowledgeIndex::build(&kb);

        let results = idx.lookup("sigma");
        assert!(results.contains(&"e1".to_string()));
        assert!(results.contains(&"e3".to_string()));
        assert!(!results.contains(&"e2".to_string()));
    }

    #[test]
    fn test_index_and_query() {
        let mut kb = KnowledgeBase::new("/tmp/test_idx2.jsonl");
        kb.add(entry("e1", "sigma physics energy"));
        kb.add(entry("e2", "sigma biology"));

        let idx = KnowledgeIndex::build(&kb);

        // AND of "sigma" and "physics" -> only e1
        let results = idx.lookup("sigma physics");
        assert!(results.contains(&"e1".to_string()));
        assert!(!results.contains(&"e2".to_string()));
    }

    #[test]
    fn test_index_no_match() {
        let mut kb = KnowledgeBase::new("/tmp/test_idx3.jsonl");
        kb.add(entry("e1", "sigma physics"));

        let idx = KnowledgeIndex::build(&kb);
        let results = idx.lookup("nonexistent");
        assert!(results.is_empty());
    }

    #[test]
    fn test_token_count() {
        let mut kb = KnowledgeBase::new("/tmp/test_idx4.jsonl");
        kb.add(entry("e1", "alpha beta"));
        let idx = KnowledgeIndex::build(&kb);
        assert!(idx.token_count() >= 2); // at least alpha, beta + "constant" from entry_type
    }
}
