use super::store::{KnowledgeBase, KnowledgeEntry};
use std::collections::{HashSet, VecDeque};

/// BFS traversal to find related entries up to `depth` hops away.
pub fn query_related<'a>(
    kb: &'a KnowledgeBase,
    entry_id: &str,
    depth: usize,
) -> Vec<&'a KnowledgeEntry> {
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<(String, usize)> = VecDeque::new();
    let mut result: Vec<&KnowledgeEntry> = Vec::new();

    visited.insert(entry_id.to_string());
    queue.push_back((entry_id.to_string(), 0));

    while let Some((current_id, current_depth)) = queue.pop_front() {
        if current_depth > depth {
            break;
        }

        if let Some(entry) = kb.get(&current_id) {
            if current_depth > 0 {
                result.push(entry);
            }
            if current_depth < depth {
                for ref_id in &entry.references {
                    if !visited.contains(ref_id) {
                        visited.insert(ref_id.clone());
                        queue.push_back((ref_id.clone(), current_depth + 1));
                    }
                }
            }
        }
    }

    result
}

/// Return all unverified entries.
pub fn query_unverified<'a>(kb: &'a KnowledgeBase) -> Vec<&'a KnowledgeEntry> {
    kb.entries().iter().filter(|e| !e.verified).collect()
}

/// Return entries with confidence >= threshold.
pub fn query_high_confidence<'a>(
    kb: &'a KnowledgeBase,
    threshold: f64,
) -> Vec<&'a KnowledgeEntry> {
    kb.entries()
        .iter()
        .filter(|e| e.confidence >= threshold)
        .collect()
}

/// Return entries with n6_score >= threshold.
pub fn query_high_n6<'a>(kb: &'a KnowledgeBase, threshold: f64) -> Vec<&'a KnowledgeEntry> {
    kb.entries()
        .iter()
        .filter(|e| e.n6_score >= threshold)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::knowledge::store::KnowledgeEntry;
    use std::collections::HashMap;

    fn entry(id: &str, refs: Vec<&str>, verified: bool, conf: f64) -> KnowledgeEntry {
        KnowledgeEntry {
            id: id.to_string(),
            entry_type: "constant".to_string(),
            content: format!("content-{}", id),
            metadata: HashMap::new(),
            created: "0s".to_string(),
            confidence: conf,
            n6_score: 0.8,
            references: refs.iter().map(|s| s.to_string()).collect(),
            verified,
            verified_by: Vec::new(),
        }
    }

    #[test]
    fn test_query_related_depth1() {
        let mut kb = KnowledgeBase::new("/tmp/test_qr.jsonl");
        kb.add(entry("a", vec!["b", "c"], true, 0.9));
        kb.add(entry("b", vec!["d"], true, 0.8));
        kb.add(entry("c", vec![], true, 0.7));
        kb.add(entry("d", vec![], true, 0.6));

        let related = query_related(&kb, "a", 1);
        let ids: Vec<&str> = related.iter().map(|e| e.id.as_str()).collect();
        assert!(ids.contains(&"b"));
        assert!(ids.contains(&"c"));
        assert!(!ids.contains(&"d")); // depth 2
    }

    #[test]
    fn test_query_related_depth2() {
        let mut kb = KnowledgeBase::new("/tmp/test_qr2.jsonl");
        kb.add(entry("a", vec!["b"], true, 0.9));
        kb.add(entry("b", vec!["c"], true, 0.8));
        kb.add(entry("c", vec![], true, 0.7));

        let related = query_related(&kb, "a", 2);
        let ids: Vec<&str> = related.iter().map(|e| e.id.as_str()).collect();
        assert!(ids.contains(&"b"));
        assert!(ids.contains(&"c"));
    }

    #[test]
    fn test_query_unverified() {
        let mut kb = KnowledgeBase::new("/tmp/test_qu.jsonl");
        kb.add(entry("a", vec![], true, 0.9));
        kb.add(entry("b", vec![], false, 0.5));
        let unv = query_unverified(&kb);
        assert_eq!(unv.len(), 1);
        assert_eq!(unv[0].id, "b");
    }

    #[test]
    fn test_query_high_confidence() {
        let mut kb = KnowledgeBase::new("/tmp/test_qhc.jsonl");
        kb.add(entry("a", vec![], true, 0.9));
        kb.add(entry("b", vec![], true, 0.3));
        let high = query_high_confidence(&kb, 0.8);
        assert_eq!(high.len(), 1);
        assert_eq!(high[0].id, "a");
    }
}
