use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScanRecord {
    pub id: String,
    pub timestamp: String,
    pub domain: String,
    pub lenses_used: Vec<String>,
    pub discoveries: Vec<String>,
    pub consensus_level: usize,
}

/// Append a record as a JSON line to a domain-specific .jsonl file.
pub fn append_record(dir: &str, domain: &str, record: &ScanRecord) -> io::Result<()> {
    let dir_path = Path::new(dir);
    fs::create_dir_all(dir_path)?;

    let file_path = dir_path.join(format!("{}.jsonl", domain));
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    let line = serde_json::to_string(record)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    writeln!(file, "{}", line)?;
    Ok(())
}

/// Load all records for a domain from its .jsonl file.
pub fn load_records(dir: &str, domain: &str) -> Vec<ScanRecord> {
    let file_path = Path::new(dir).join(format!("{}.jsonl", domain));
    let file = match fs::File::open(file_path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };

    let reader = io::BufReader::new(file);
    let mut records = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if let Ok(record) = serde_json::from_str::<ScanRecord>(trimmed) {
                records.push(record);
            }
        }
    }

    records
}
