//! REST API interface for external tool integration.
/// REST API server — request routing without external crates.
/// Only handle_request is implemented; TCP listener deferred.

pub struct ApiServer {
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct ApiResponse {
    pub status: u16,
    pub body: String,
}

impl ApiResponse {
    pub fn ok(body: String) -> Self {
        Self { status: 200, body }
    }

    pub fn not_found() -> Self {
        Self {
            status: 404,
            body: r#"{"error":"not found"}"#.to_string(),
        }
    }

    pub fn bad_request(msg: &str) -> Self {
        Self {
            status: 400,
            body: format!(r#"{{"error":"{}"}}"#, msg),
        }
    }
}

impl ApiServer {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    /// Route a request to the appropriate handler.
    pub fn handle_request(&self, method: &str, path: &str, body: &str) -> ApiResponse {
        match (method, path) {
            ("GET", "/health") => self.get_health(),
            ("GET", "/lenses") => self.get_lenses(),
            ("GET", "/lenses/count") => self.get_lens_count(),
            ("POST", "/scan") => self.post_scan(body),
            ("POST", "/verify") => self.post_verify(body),
            ("GET", "/constants") => self.get_constants(),
            ("GET", "/version") => self.get_version(),
            _ => ApiResponse::not_found(),
        }
    }

    fn get_health(&self) -> ApiResponse {
        ApiResponse::ok(r#"{"status":"ok","engine":"nexus6","port":PORT}"#
            .replace("PORT", &self.port.to_string()))
    }

    fn get_version(&self) -> ApiResponse {
        ApiResponse::ok(r#"{"version":"0.1.0","n":6,"sigma":12,"phi":2,"tau":4}"#.to_string())
    }

    fn get_lenses(&self) -> ApiResponse {
        use crate::telescope::registry::LensRegistry;
        let registry = LensRegistry::new();
        let mut names: Vec<String> = registry.iter().map(|(name, _)| name.clone()).collect();
        names.sort();

        let json_list: Vec<String> = names.iter().map(|n| format!("\"{}\"", n)).collect();
        ApiResponse::ok(format!(
            r#"{{"total":{},"lenses":[{}]}}"#,
            names.len(),
            json_list.join(",")
        ))
    }

    fn get_lens_count(&self) -> ApiResponse {
        use crate::telescope::registry::{LensCategory, LensRegistry};
        let registry = LensRegistry::new();
        let core = registry.by_category(LensCategory::Core).len();
        let extended = registry.by_category(LensCategory::Extended).len();
        let custom = registry.by_category(LensCategory::Custom).len();
        let total = registry.len();

        ApiResponse::ok(format!(
            r#"{{"total":{},"core":{},"extended":{},"custom":{}}}"#,
            total, core, extended, custom
        ))
    }

    fn post_scan(&self, body: &str) -> ApiResponse {
        // Expect body like: {"domain":"physics"} or just the domain name
        let domain = extract_json_string(body, "domain")
            .unwrap_or_else(|| body.trim().trim_matches('"').to_string());

        if domain.is_empty() {
            return ApiResponse::bad_request("missing domain");
        }

        use crate::telescope::Telescope;
        let telescope = Telescope::new();
        let probe_data: Vec<f64> = vec![6.0, 12.0, 24.0, 4.0, 2.0, 5.0];
        let results = telescope.scan_all(&probe_data, probe_data.len(), 1);
        let hits = results.len();

        ApiResponse::ok(format!(
            r#"{{"domain":"{}","hits":{},"lens_count":{}}}"#,
            domain, hits, telescope.lens_count()
        ))
    }

    fn post_verify(&self, body: &str) -> ApiResponse {
        // Expect body like: {"value":12.0} or just a number
        let value_str = extract_json_string(body, "value")
            .unwrap_or_else(|| body.trim().to_string());

        let value: f64 = match value_str.parse() {
            Ok(v) => v,
            Err(_) => return ApiResponse::bad_request("invalid number"),
        };

        use crate::verifier::n6_check;
        let (closest_name, quality) = n6_check::n6_match(value);
        let n6_matches = if quality >= 1.0 { 1 } else { 0 };
        let distance = if quality > 0.0 { 1.0 - quality } else { 1.0 };

        ApiResponse::ok(format!(
            r#"{{"value":{},"n6_matches":{},"closest":"{}","distance":{}}}"#,
            value,
            n6_matches,
            closest_name,
            distance
        ))
    }

    fn get_constants(&self) -> ApiResponse {
        ApiResponse::ok(r#"{"n":6,"sigma":12,"phi":2,"tau":4,"J2":24,"sopfr":5,"mu":1,"sigma_phi":10,"sigma_tau":8}"#.to_string())
    }
}

/// Minimal JSON string extractor — no serde needed.
/// Finds `"key":"value"` or `"key":number` in a JSON-like string.
fn extract_json_string(json: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{}\"", key);
    let start = json.find(&pattern)?;
    let after_key = &json[start + pattern.len()..];
    // Skip whitespace and colon
    let after_colon = after_key.trim_start().strip_prefix(':')?;
    let trimmed = after_colon.trim_start();

    if trimmed.starts_with('"') {
        // String value
        let inner = &trimmed[1..];
        let end = inner.find('"')?;
        Some(inner[..end].to_string())
    } else {
        // Numeric or other value — take until comma, brace, or end
        let end = trimmed.find(|c: char| c == ',' || c == '}' || c == ']')
            .unwrap_or(trimmed.len());
        Some(trimmed[..end].trim().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health() {
        let server = ApiServer::new(8080);
        let resp = server.handle_request("GET", "/health", "");
        assert_eq!(resp.status, 200);
        assert!(resp.body.contains("ok"));
        assert!(resp.body.contains("8080"));
    }

    #[test]
    fn test_not_found() {
        let server = ApiServer::new(8080);
        let resp = server.handle_request("GET", "/nonexistent", "");
        assert_eq!(resp.status, 404);
    }

    #[test]
    fn test_lenses() {
        let server = ApiServer::new(8080);
        let resp = server.handle_request("GET", "/lenses", "");
        assert_eq!(resp.status, 200);
        assert!(resp.body.contains("total"));
        assert!(resp.body.contains("consciousness"));
    }

    #[test]
    fn test_lens_count() {
        let server = ApiServer::new(8080);
        let resp = server.handle_request("GET", "/lenses/count", "");
        assert_eq!(resp.status, 200);
        assert!(resp.body.contains("total"));
        assert!(resp.body.contains("core"));
    }

    #[test]
    fn test_verify() {
        let server = ApiServer::new(8080);
        let resp = server.handle_request("POST", "/verify", r#"{"value":12.0}"#);
        assert_eq!(resp.status, 200);
        assert!(resp.body.contains("n6_matches"));
    }

    #[test]
    fn test_scan() {
        let server = ApiServer::new(8080);
        let resp = server.handle_request("POST", "/scan", r#"{"domain":"physics"}"#);
        assert_eq!(resp.status, 200);
        assert!(resp.body.contains("physics"));
        assert!(resp.body.contains("hits"));
    }

    #[test]
    fn test_constants() {
        let server = ApiServer::new(8080);
        let resp = server.handle_request("GET", "/constants", "");
        assert_eq!(resp.status, 200);
        assert!(resp.body.contains("\"n\":6"));
        assert!(resp.body.contains("\"sigma\":12"));
    }

    #[test]
    fn test_version() {
        let server = ApiServer::new(8080);
        let resp = server.handle_request("GET", "/version", "");
        assert_eq!(resp.status, 200);
        assert!(resp.body.contains("0.1.0"));
    }

    #[test]
    fn test_extract_json_string() {
        assert_eq!(extract_json_string(r#"{"domain":"physics"}"#, "domain"), Some("physics".to_string()));
        assert_eq!(extract_json_string(r#"{"value":12.5}"#, "value"), Some("12.5".to_string()));
        assert_eq!(extract_json_string(r#"{"key":"val"}"#, "missing"), None);
    }
}
