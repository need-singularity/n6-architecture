//! Sandboxed execution environment for safe code generation.
/// Safe experiment sandbox — copy-on-write data isolation.
/// No external crates.

use std::fmt;

#[derive(Debug, Clone)]
pub struct Sandbox {
    pub id: String,
    pub original_data: Vec<f64>,
    pub working_data: Vec<f64>,
    pub modifications: Vec<String>,
}

impl Sandbox {
    /// Create a new sandbox with a deep copy of the data.
    pub fn new(data: &[f64]) -> Self {
        let id = format!("sandbox-{}", data.len());
        Self {
            id,
            original_data: data.to_vec(),
            working_data: data.to_vec(),
            modifications: Vec::new(),
        }
    }

    /// Apply an operation to the working data.
    /// Supported operations:
    ///   "scale:<factor>"  — multiply all elements
    ///   "shift:<offset>"  — add to all elements
    ///   "normalize"       — normalize to [0,1]
    ///   "set:<idx>:<val>" — set a specific index
    ///   "sort"            — sort ascending
    ///   "reverse"         — reverse order
    pub fn modify(&mut self, operation: &str) {
        let parts: Vec<&str> = operation.splitn(3, ':').collect();
        match parts[0] {
            "scale" => {
                if let Some(factor) = parts.get(1).and_then(|s| s.parse::<f64>().ok()) {
                    for v in &mut self.working_data {
                        *v *= factor;
                    }
                    self.modifications.push(format!("scale({})", factor));
                }
            }
            "shift" => {
                if let Some(offset) = parts.get(1).and_then(|s| s.parse::<f64>().ok()) {
                    for v in &mut self.working_data {
                        *v += offset;
                    }
                    self.modifications.push(format!("shift({})", offset));
                }
            }
            "normalize" => {
                let min = self.working_data.iter().cloned().fold(f64::INFINITY, f64::min);
                let max = self.working_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                let range = max - min;
                if range > 0.0 {
                    for v in &mut self.working_data {
                        *v = (*v - min) / range;
                    }
                }
                self.modifications.push("normalize".to_string());
            }
            "set" => {
                if parts.len() == 3 {
                    // parts = ["set", "<idx>", "<val>"]
                    // But we split by 3, and the format is "set:idx:val"
                    if let (Ok(idx), Ok(val)) = (parts[1].parse::<usize>(), parts[2].parse::<f64>()) {
                        if idx < self.working_data.len() {
                            self.working_data[idx] = val;
                            self.modifications.push(format!("set([{}]={})", idx, val));
                        }
                    }
                }
            }
            "sort" => {
                self.working_data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                self.modifications.push("sort".to_string());
            }
            "reverse" => {
                self.working_data.reverse();
                self.modifications.push("reverse".to_string());
            }
            _ => {
                self.modifications.push(format!("unknown({})", operation));
            }
        }
    }

    /// Reset working data to the original snapshot.
    pub fn reset(&mut self) {
        self.working_data = self.original_data.clone();
        self.modifications.push("reset".to_string());
    }

    /// Return indices where original and working data differ.
    /// Each tuple: (index, original_value, working_value)
    pub fn diff(&self) -> Vec<(usize, f64, f64)> {
        let mut diffs = Vec::new();
        let max_len = self.original_data.len().max(self.working_data.len());
        for i in 0..max_len {
            let orig = self.original_data.get(i).copied().unwrap_or(0.0);
            let work = self.working_data.get(i).copied().unwrap_or(0.0);
            if (orig - work).abs() > f64::EPSILON {
                diffs.push((i, orig, work));
            }
        }
        diffs
    }

    /// Commit: return a clone of the working data as the finalized result.
    pub fn commit(&self) -> Vec<f64> {
        self.working_data.clone()
    }

    /// Number of modifications applied so far.
    pub fn modification_count(&self) -> usize {
        self.modifications.len()
    }
}

impl fmt::Display for Sandbox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sandbox({}, {} pts, {} mods)", self.id, self.working_data.len(), self.modifications.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_isolation() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let mut sb = Sandbox::new(&data);
        sb.modify("scale:2.0");
        // Original unchanged
        assert_eq!(sb.original_data, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        assert_eq!(sb.working_data, vec![2.0, 4.0, 6.0, 8.0, 10.0, 12.0]);
    }

    #[test]
    fn test_reset() {
        let data = vec![6.0, 12.0, 24.0];
        let mut sb = Sandbox::new(&data);
        sb.modify("shift:10.0");
        assert_eq!(sb.working_data, vec![16.0, 22.0, 34.0]);
        sb.reset();
        assert_eq!(sb.working_data, vec![6.0, 12.0, 24.0]);
    }

    #[test]
    fn test_diff_and_commit() {
        let data = vec![1.0, 2.0, 3.0];
        let mut sb = Sandbox::new(&data);
        sb.modify("set:1:99.0");
        let diffs = sb.diff();
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0], (1, 2.0, 99.0));
        let committed = sb.commit();
        assert_eq!(committed, vec![1.0, 99.0, 3.0]);
    }

    #[test]
    fn test_normalize() {
        let data = vec![0.0, 5.0, 10.0];
        let mut sb = Sandbox::new(&data);
        sb.modify("normalize");
        assert!((sb.working_data[0] - 0.0).abs() < 1e-10);
        assert!((sb.working_data[1] - 0.5).abs() < 1e-10);
        assert!((sb.working_data[2] - 1.0).abs() < 1e-10);
    }
}
