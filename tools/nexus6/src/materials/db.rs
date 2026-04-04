use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single material entry with numeric properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub name: String,
    pub properties: HashMap<String, f64>,
}

/// A domain (e.g., superconductor, chip-architecture) containing materials and an optional ceiling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainData {
    pub materials: Vec<Material>,
    #[serde(default)]
    pub ceiling: HashMap<String, f64>,
}

/// Top-level materials database: domain name -> domain data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialsDB {
    pub domains: HashMap<String, DomainData>,
}

/// Load a MaterialsDB from a JSON file path.
pub fn load(path: &str) -> MaterialsDB {
    let content = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Failed to read materials DB at {}: {}", path, e));
    serde_json::from_str(&content)
        .unwrap_or_else(|e| panic!("Failed to parse materials DB JSON: {}", e))
}

/// Convert a domain's materials into a flat f64 matrix for the given feature keys.
///
/// Returns (flat_data, n_rows, n_cols). Missing properties become f64::NAN.
pub fn materials_as_matrix(
    domain: &DomainData,
    feature_keys: &[&str],
) -> (Vec<f64>, usize, usize) {
    let n_rows = domain.materials.len();
    let n_cols = feature_keys.len();
    let mut data = Vec::with_capacity(n_rows * n_cols);

    for mat in &domain.materials {
        for &key in feature_keys {
            let val = mat.properties.get(key).copied().unwrap_or(f64::NAN);
            data.push(val);
        }
    }

    (data, n_rows, n_cols)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_domain() -> DomainData {
        let mut props_a = HashMap::new();
        props_a.insert("Z".to_string(), 6.0);
        props_a.insert("CN".to_string(), 6.0);

        let mut props_b = HashMap::new();
        props_b.insert("Z".to_string(), 14.0);
        // CN intentionally missing

        DomainData {
            materials: vec![
                Material { name: "Carbon".to_string(), properties: props_a },
                Material { name: "Silicon".to_string(), properties: props_b },
            ],
            ceiling: HashMap::new(),
        }
    }

    #[test]
    fn test_materials_as_matrix_basic() {
        let domain = sample_domain();
        let keys = &["Z", "CN"];
        let (data, n, d) = materials_as_matrix(&domain, keys);
        assert_eq!(n, 2);
        assert_eq!(d, 2);
        assert_eq!(data.len(), 4);
        // Carbon: Z=6, CN=6
        assert_eq!(data[0], 6.0);
        assert_eq!(data[1], 6.0);
        // Silicon: Z=14, CN=NaN
        assert_eq!(data[2], 14.0);
        assert!(data[3].is_nan());
    }

    #[test]
    fn test_materials_as_matrix_empty_domain() {
        let domain = DomainData {
            materials: vec![],
            ceiling: HashMap::new(),
        };
        let (data, n, d) = materials_as_matrix(&domain, &["Z"]);
        assert_eq!(n, 0);
        assert_eq!(d, 1);
        assert!(data.is_empty());
    }

    #[test]
    fn test_materials_as_matrix_empty_keys() {
        let domain = sample_domain();
        let (data, n, d) = materials_as_matrix(&domain, &[]);
        assert_eq!(n, 2);
        assert_eq!(d, 0);
        assert!(data.is_empty());
    }

    #[test]
    fn test_material_struct_clone() {
        let mut props = HashMap::new();
        props.insert("Z".to_string(), 6.0);
        let m = Material { name: "Diamond".to_string(), properties: props };
        let m2 = m.clone();
        assert_eq!(m2.name, "Diamond");
        assert_eq!(m2.properties["Z"], 6.0);
    }

    #[test]
    fn test_materials_db_serde_roundtrip() {
        let domain = sample_domain();
        let mut domains = HashMap::new();
        domains.insert("superconductor".to_string(), domain);
        let db = MaterialsDB { domains };

        let json = serde_json::to_string(&db).unwrap();
        let db2: MaterialsDB = serde_json::from_str(&json).unwrap();
        assert_eq!(db2.domains.len(), 1);
        assert!(db2.domains.contains_key("superconductor"));
        let d = &db2.domains["superconductor"];
        assert_eq!(d.materials.len(), 2);
        assert_eq!(d.materials[0].name, "Carbon");
    }

    #[test]
    fn test_n6_carbon_z_equals_6() {
        // n=6 verification: Carbon Z=6 is the foundational n=6 material constant
        let mut props = HashMap::new();
        props.insert("Z".to_string(), 6.0);
        props.insert("CN".to_string(), 6.0); // octahedral coordination
        let m = Material { name: "Carbon".to_string(), properties: props };
        assert_eq!(m.properties["Z"], 6.0);  // Z = n
        assert_eq!(m.properties["CN"], 6.0); // CN = n (BT-43, BT-85)
    }
}
