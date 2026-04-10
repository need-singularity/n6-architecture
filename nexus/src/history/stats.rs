use std::collections::HashMap;
use super::recorder::ScanRecord;

#[derive(Debug, Clone)]
pub struct LensStats {
    pub used: usize,
    pub contributed: usize,
    pub hit_rate: f64,
}

#[derive(Debug, Clone)]
pub struct DomainStats {
    pub total_scans: usize,
    pub total_discoveries: usize,
    pub lens_stats: HashMap<String, LensStats>,
}

/// Compute aggregate statistics for a set of scan records in one domain.
pub fn compute_domain_stats(records: &[ScanRecord]) -> DomainStats {
    let total_scans = records.len();
    let total_discoveries: usize = records.iter().map(|r| r.discoveries.len()).sum();

    let mut used_count: HashMap<String, usize> = HashMap::new();
    let mut contributed_count: HashMap<String, usize> = HashMap::new();

    for record in records {
        for lens in &record.lenses_used {
            *used_count.entry(lens.clone()).or_insert(0) += 1;
        }

        // A lens "contributed" if it was used in a scan that produced discoveries
        if !record.discoveries.is_empty() {
            for lens in &record.lenses_used {
                *contributed_count.entry(lens.clone()).or_insert(0) += 1;
            }
        }
    }

    let mut lens_stats = HashMap::new();
    for (lens, used) in &used_count {
        let contributed = contributed_count.get(lens).copied().unwrap_or(0);
        let hit_rate = if *used > 0 {
            contributed as f64 / *used as f64
        } else {
            0.0
        };
        lens_stats.insert(
            lens.clone(),
            LensStats {
                used: *used,
                contributed,
                hit_rate,
            },
        );
    }

    DomainStats {
        total_scans,
        total_discoveries,
        lens_stats,
    }
}

/// Compute lens affinity: for each pair of lenses, how often they co-occur
/// in scans that produce discoveries, normalized by total discovery-producing scans.
pub fn compute_lens_affinity(records: &[ScanRecord]) -> HashMap<(String, String), f64> {
    let discovery_records: Vec<&ScanRecord> = records
        .iter()
        .filter(|r| !r.discoveries.is_empty())
        .collect();

    let total = discovery_records.len();
    if total == 0 {
        return HashMap::new();
    }

    let mut pair_counts: HashMap<(String, String), usize> = HashMap::new();

    for record in &discovery_records {
        let lenses = &record.lenses_used;
        for i in 0..lenses.len() {
            for j in (i + 1)..lenses.len() {
                let (a, b) = if lenses[i] <= lenses[j] {
                    (lenses[i].clone(), lenses[j].clone())
                } else {
                    (lenses[j].clone(), lenses[i].clone())
                };
                *pair_counts.entry((a, b)).or_insert(0) += 1;
            }
        }
    }

    pair_counts
        .into_iter()
        .map(|(pair, count)| (pair, count as f64 / total as f64))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_record(domain: &str, lenses: &[&str], discoveries: &[&str]) -> ScanRecord {
        ScanRecord {
            id: format!("{}-scan", domain),
            timestamp: "0".into(),
            domain: domain.into(),
            lenses_used: lenses.iter().map(|s| s.to_string()).collect(),
            discoveries: discoveries.iter().map(|s| s.to_string()).collect(),
            consensus_level: 6, // n=6
        }
    }

    #[test]
    fn test_domain_stats_basic() {
        let records = vec![
            make_record("ai", &["consciousness", "topology"], &["D-1", "D-2"]),
            make_record("ai", &["consciousness", "wave"], &[]),
            make_record("ai", &["topology"], &["D-3"]),
        ];
        let stats = compute_domain_stats(&records);
        assert_eq!(stats.total_scans, 3);
        assert_eq!(stats.total_discoveries, 3);

        // consciousness: used 2 times, contributed in 1 scan (first)
        let cs = stats.lens_stats.get("consciousness").unwrap();
        assert_eq!(cs.used, 2);
        assert_eq!(cs.contributed, 1);
        assert!((cs.hit_rate - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_domain_stats_empty_records() {
        let stats = compute_domain_stats(&[]);
        assert_eq!(stats.total_scans, 0);
        assert_eq!(stats.total_discoveries, 0);
        assert!(stats.lens_stats.is_empty());
    }

    #[test]
    fn test_lens_affinity_pairs() {
        let records = vec![
            make_record("ai", &["A", "B", "C"], &["D-1"]),
            make_record("ai", &["A", "B"], &["D-2"]),
            make_record("ai", &["C"], &[]), // no discoveries, ignored
        ];
        let affinity = compute_lens_affinity(&records);
        // 2 discovery records total
        // (A,B) appears in both => 2/2 = 1.0
        let ab = affinity.get(&("A".to_string(), "B".to_string())).copied().unwrap_or(0.0);
        assert!((ab - 1.0).abs() < 1e-10);

        // (A,C) appears in first only => 1/2 = 0.5
        let ac = affinity.get(&("A".to_string(), "C".to_string())).copied().unwrap_or(0.0);
        assert!((ac - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_lens_affinity_no_discoveries() {
        let records = vec![
            make_record("ai", &["A", "B"], &[]),
        ];
        let affinity = compute_lens_affinity(&records);
        assert!(affinity.is_empty());
    }

    #[test]
    fn test_domain_stats_all_lenses_have_stats() {
        // 6 records with 6 distinct lenses (n=6)
        let lenses = ["consciousness", "topology", "causal", "wave", "thermo", "evolution"];
        let records: Vec<ScanRecord> = lenses.iter().enumerate().map(|(i, lens)| {
            let disc = if i < 3 { vec!["d"] } else { vec![] };
            make_record("fusion", &[lens], &disc)
        }).collect();
        let stats = compute_domain_stats(&records);
        assert_eq!(stats.lens_stats.len(), 6); // n=6 lenses tracked
    }
}
