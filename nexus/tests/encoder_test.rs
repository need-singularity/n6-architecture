use std::collections::HashMap;
use nexus::encoder::parser::parse_hypotheses;
use nexus::encoder::vectorize::vectorize;
use nexus::materials::db;

// ─── Parser tests ───

#[test]
fn test_parse_hypotheses() {
    let md = "\
## H-SC-01: MgB2 superconductor
- Tc = 39 K
- Pressure = 0 GPa
- Gap = 7.1 meV

## H-SC-02: YBCO cuprate
- Tc = 93 K
- Pressure = 0 GPa

## H-SC-03: H3S compressed hydride
- Tc = 203 K
- Pressure = 155 GPa
";
    let entries = parse_hypotheses(md);
    assert_eq!(entries.len(), 3, "should parse 3 hypotheses");

    // Check IDs
    assert_eq!(entries[0].get("id").unwrap(), "H-SC-01");
    assert_eq!(entries[1].get("id").unwrap(), "H-SC-02");
    assert_eq!(entries[2].get("id").unwrap(), "H-SC-03");

    // Check titles
    assert!(entries[0].get("title").unwrap().contains("MgB2"));

    // Check unit stripping
    assert_eq!(entries[0].get("Tc").unwrap(), "39");
    assert_eq!(entries[2].get("Pressure").unwrap(), "155");
}

// ─── Vectorize tests ───

#[test]
fn test_vectorize() {
    let mut e1 = HashMap::new();
    e1.insert("Tc".to_string(), "39".to_string());
    e1.insert("Pressure".to_string(), "0".to_string());

    let mut e2 = HashMap::new();
    e2.insert("Tc".to_string(), "93".to_string());
    e2.insert("Pressure".to_string(), "0".to_string());

    let mut e3 = HashMap::new();
    e3.insert("Tc".to_string(), "203".to_string());
    e3.insert("Pressure".to_string(), "155".to_string());

    let entries = vec![e1, e2, e3];
    let (data, rows, cols) = vectorize(&entries, &["Tc", "Pressure"]);

    assert_eq!(rows, 3);
    assert_eq!(cols, 2);
    assert_eq!(data.len(), 6);

    // Row 0: Tc=39, P=0
    assert_eq!(data[0], 39.0);
    assert_eq!(data[1], 0.0);

    // Row 2: Tc=203, P=155
    assert_eq!(data[4], 203.0);
    assert_eq!(data[5], 155.0);
}

// ─── Materials DB tests ───

#[test]
fn test_materials_load() {
    let db_path = concat!(env!("CARGO_MANIFEST_DIR"), "/data/materials-db.json");
    let mdb = db::load(db_path);

    assert_eq!(mdb.domains.len(), 3, "should have 3 domains");
    assert!(mdb.domains.contains_key("superconductor"));
    assert!(mdb.domains.contains_key("chip-architecture"));
    assert!(mdb.domains.contains_key("battery"));

    // Superconductor domain has 4 materials
    let sc = &mdb.domains["superconductor"];
    assert_eq!(sc.materials.len(), 4);

    // Ceiling exists
    assert_eq!(*sc.ceiling.get("Tc").unwrap(), 300.0);
}

#[test]
fn test_materials_as_matrix() {
    let db_path = concat!(env!("CARGO_MANIFEST_DIR"), "/data/materials-db.json");
    let mdb = db::load(db_path);

    let sc = &mdb.domains["superconductor"];
    let (data, rows, cols) = db::materials_as_matrix(sc, &["Tc", "pressure_GPa"]);

    assert_eq!(rows, 4);
    assert_eq!(cols, 2);
    assert_eq!(data.len(), 8);

    // MgB2: Tc=39, pressure=0
    assert_eq!(data[0], 39.0);
    assert_eq!(data[1], 0.0);

    // LaH10: Tc=250, pressure=170
    assert_eq!(data[6], 250.0);
    assert_eq!(data[7], 170.0);
}
