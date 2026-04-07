use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Category of a lens in the registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LensCategory {
    /// The 22 foundational lenses from the telescope specification.
    Core,
    /// The 10 domain-specific lens combinations.
    DomainCombo,
    /// Extended lenses added through incremental expansion (toward 411).
    Extended,
    /// User-defined custom lenses.
    Custom,
}

/// Metadata entry for a single lens in the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LensEntry {
    pub name: String,
    pub category: LensCategory,
    pub description: String,
    /// Domains where this lens is most effective.
    pub domain_affinity: Vec<String>,
    /// Other lenses that pair well with this one.
    pub complementary: Vec<String>,
}

/// Central registry for all lens metadata.
///
/// This is a *metadata* registry — it stores descriptions, affinities and
/// relationships. The actual scan logic lives in the `Lens` trait implementors.
/// The registry enables discovery ("which lenses suit domain X?") and
/// incremental growth toward the full 411-lens set.
pub struct LensRegistry {
    entries: HashMap<String, LensEntry>,
}

impl LensRegistry {
    /// Create a new registry pre-populated with the 22 Core lenses,
    /// 58 n6-industry lenses, 40 cross-project lenses, 103 TECS-L math
    /// lenses, 88 Anima consciousness lenses, 100 SEDI signal lenses,
    /// 58 accel ML lenses, 57 accel physics/neuro lenses, 55 accel
    /// engineering lenses, 63 accel humanities lenses, and 49 physics-deep
    /// lenses (693 total).
    pub fn new() -> Self {
        let mut reg = LensRegistry {
            entries: HashMap::new(),
        };
        for entry in super::core_lenses::core_lens_entries() {
            reg.entries.insert(entry.name.clone(), entry);
        }
        for entry in super::n6_lenses::n6_industry_lens_entries() {
            reg.entries.insert(entry.name.clone(), entry);
        }
        for entry in super::cross_lenses::cross_project_lens_entries() {
            reg.entries.insert(entry.name.clone(), entry);
        }
        for entry in super::tecs_lenses::tecs_math_lens_entries() {
            reg.entries.insert(entry.name.clone(), entry);
        }
        for entry in super::anima_lenses::anima_consciousness_lens_entries() {
            reg.entries.insert(entry.name.clone(), entry);
        }
        for entry in super::sedi_lenses::sedi_signal_lens_entries() {
            reg.entries.insert(entry.name.clone(), entry);
        }
        for entry in super::accel_lenses_a::accel_ml_lens_entries() {
            reg.entries.insert(entry.name.clone(), entry);
        }
        for entry in super::accel_lenses_b::accel_physics_neuro_lens_entries() {
            reg.entries.insert(entry.name.clone(), entry);
        }
        for entry in super::accel_lenses_c::accel_engineering_lens_entries() {
            reg.entries.insert(entry.name.clone(), entry);
        }
        for entry in super::accel_lenses_d::accel_humanities_lens_entries() {
            reg.entries.insert(entry.name.clone(), entry);
        }
        for entry in super::quantum_lenses::quantum_topology_lens_entries() {
            reg.entries.insert(entry.name.clone(), entry);
        }
        for entry in super::physics_deep_lenses::physics_deep_lens_entries() {
            reg.entries.insert(entry.name.clone(), entry);
        }
        for entry in super::frontier_lenses::frontier_lens_entries() {
            reg.entries.insert(entry.name.clone(), entry);
        }
        // Auto-load persisted custom lenses from disk (if present).
        let _ = reg.load_custom();
        reg
    }

    /// Path to the persisted custom lens file: ~/.nexus/custom_lenses.json
    pub fn custom_lens_path() -> Option<PathBuf> {
        let home = std::env::var("HOME").ok()?;
        Some(PathBuf::from(home).join(".nexus").join("custom_lenses.json"))
    }

    /// Load custom lenses from `~/.nexus/custom_lenses.json` into the registry.
    /// Returns the number of entries loaded. Missing file is not an error.
    pub fn load_custom(&mut self) -> std::io::Result<usize> {
        let path = match Self::custom_lens_path() {
            Some(p) => p,
            None => return Ok(0),
        };
        if !path.exists() {
            return Ok(0);
        }
        let data = std::fs::read_to_string(&path)?;
        let entries: Vec<LensEntry> = serde_json::from_str(&data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        let count = entries.len();
        for e in entries {
            self.entries.insert(e.name.clone(), e);
        }
        Ok(count)
    }

    /// Save all Custom-category lenses to `~/.nexus/custom_lenses.json`.
    /// Creates the ~/.nexus directory if needed. Returns number saved.
    ///
    /// 병렬 안전성(race-safe):
    ///   1. 디스크에서 기존 custom 렌즈를 다시 읽는다
    ///   2. 메모리의 custom 렌즈와 이름-기반 union으로 병합 (메모리 우선)
    ///   3. 임시 파일에 쓰고 atomic rename (read-during-write 방지)
    /// 서로 다른 렌즈를 단조한 두 프로세스가 동시에 save_custom을 호출해도
    /// 둘 다 보존된다.
    pub fn save_custom(&self) -> std::io::Result<usize> {
        let path = match Self::custom_lens_path() {
            Some(p) => p,
            None => return Ok(0),
        };
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // 1. 메모리의 custom 렌즈 수집
        let mut merged: std::collections::BTreeMap<String, LensEntry> = self
            .entries
            .values()
            .filter(|e| e.category == LensCategory::Custom)
            .map(|e| (e.name.clone(), e.clone()))
            .collect();

        // 2. 디스크에서 기존 custom 렌즈 병합 (메모리에 없는 것만 추가)
        if path.exists() {
            if let Ok(data) = std::fs::read_to_string(&path) {
                if let Ok(disk_entries) = serde_json::from_str::<Vec<LensEntry>>(&data) {
                    for e in disk_entries {
                        merged.entry(e.name.clone()).or_insert(e);
                    }
                }
            }
        }

        let final_list: Vec<&LensEntry> = merged.values().collect();
        let json = serde_json::to_string_pretty(&final_list)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        // 3. Atomic write: 임시 파일 → rename
        let tmp_path = path.with_extension(format!(
            "json.tmp.{}",
            std::process::id()
        ));
        std::fs::write(&tmp_path, json)?;
        std::fs::rename(&tmp_path, &path)?;
        Ok(final_list.len())
    }

    /// Register a new lens entry. Overwrites if name already exists.
    pub fn register(&mut self, entry: LensEntry) {
        self.entries.insert(entry.name.clone(), entry);
    }

    /// Look up a lens by name.
    pub fn get(&self, name: &str) -> Option<&LensEntry> {
        self.entries.get(name)
    }

    /// Return all lenses belonging to the given category.
    pub fn by_category(&self, cat: LensCategory) -> Vec<&LensEntry> {
        self.entries
            .values()
            .filter(|e| e.category == cat)
            .collect()
    }

    /// Recommend lenses for a given domain string (case-insensitive substring match).
    pub fn for_domain(&self, domain: &str) -> Vec<&LensEntry> {
        let domain_lower = domain.to_lowercase();
        self.entries
            .values()
            .filter(|e| {
                e.domain_affinity
                    .iter()
                    .any(|d| d.to_lowercase().contains(&domain_lower))
            })
            .collect()
    }

    /// Total number of registered lenses.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Iterator over all entries.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &LensEntry)> {
        self.entries.iter()
    }
}

impl Default for LensRegistry {
    fn default() -> Self {
        Self::new()
    }
}
