/// Natural language -> CLI command converter.

use std::collections::HashMap;

/// Parsed NLP command extracted from natural language text.
#[derive(Debug, Clone)]
pub struct NlpCommand {
    pub action: String,
    pub domain: Option<String>,
    pub parameters: HashMap<String, String>,
    pub original_text: String,
}

/// Parse natural language text into a structured NlpCommand.
///
/// Examples:
///   "물리에서 이상한 패턴 찾아줘" -> scan physics --full
///   "12가 n=6과 관련있나"         -> verify 12.0
///   "의식 렌즈 장력 실험"          -> experiment tension lens:consciousness
///   "최근 발견 보여줘"             -> search --recent
pub fn parse_natural_language(text: &str) -> NlpCommand {
    let intent = super::intent::classify_intent(text);

    match intent {
        super::intent::Intent::Scan { domain, full } => {
            let mut params = HashMap::new();
            if full {
                params.insert("full".to_string(), "true".to_string());
            }
            NlpCommand {
                action: "scan".to_string(),
                domain: Some(domain),
                parameters: params,
                original_text: text.to_string(),
            }
        }
        super::intent::Intent::Verify { value } => {
            let mut params = HashMap::new();
            params.insert("value".to_string(), value.to_string());
            NlpCommand {
                action: "verify".to_string(),
                domain: None,
                parameters: params,
                original_text: text.to_string(),
            }
        }
        super::intent::Intent::Experiment { exp_type, target } => {
            let mut params = HashMap::new();
            params.insert("type".to_string(), exp_type);
            // Extract lens from text if present
            let lower = text.to_lowercase();
            let lens_keywords = [
                ("의식", "consciousness"), ("위상", "topology"), ("인과", "causal"),
                ("양자", "quantum"), ("파동", "wave"), ("진화", "evolution"),
                ("열역학", "thermo"), ("중력", "gravity"),
            ];
            for &(kr, en) in &lens_keywords {
                if lower.contains(kr) || lower.contains(en) {
                    params.insert("lens".to_string(), en.to_string());
                    break;
                }
            }
            NlpCommand {
                action: "experiment".to_string(),
                domain: Some(target),
                parameters: params,
                original_text: text.to_string(),
            }
        }
        super::intent::Intent::Search { query } => {
            let mut params = HashMap::new();
            params.insert("query".to_string(), query);
            NlpCommand {
                action: "search".to_string(),
                domain: None,
                parameters: params,
                original_text: text.to_string(),
            }
        }
        super::intent::Intent::Help => NlpCommand {
            action: "help".to_string(),
            domain: None,
            parameters: HashMap::new(),
            original_text: text.to_string(),
        },
        super::intent::Intent::Unknown(s) => NlpCommand {
            action: "unknown".to_string(),
            domain: None,
            parameters: {
                let mut p = HashMap::new();
                p.insert("text".to_string(), s);
                p
            },
            original_text: text.to_string(),
        },
    }
}

/// Convert an NlpCommand to CLI argument tokens.
///
/// Returns a Vec of strings suitable for passing to `cli::parser::parse_args`.
pub fn to_cli_args(cmd: &NlpCommand) -> Vec<String> {
    let mut args = vec!["nexus".to_string(), cmd.action.clone()];

    match cmd.action.as_str() {
        "scan" => {
            if let Some(ref domain) = cmd.domain {
                args.push(domain.clone());
            }
            if cmd.parameters.get("full").map_or(false, |v| v == "true") {
                args.push("--full".to_string());
            }
        }
        "verify" => {
            if let Some(val) = cmd.parameters.get("value") {
                args.push(val.clone());
            }
        }
        "experiment" => {
            if let Some(t) = cmd.parameters.get("type") {
                args.push(t.clone());
            }
            if let Some(ref domain) = cmd.domain {
                args.push(domain.clone());
            }
        }
        "help" => {}
        _ => {}
    }

    args
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_korean_scan() {
        let cmd = parse_natural_language("물리에서 이상한 패턴 찾아줘");
        assert_eq!(cmd.action, "scan");
        assert_eq!(cmd.domain, Some("physics".to_string()));
    }

    #[test]
    fn test_parse_verify() {
        let cmd = parse_natural_language("12가 n=6과 관련있나 확인");
        assert_eq!(cmd.action, "verify");
        assert_eq!(cmd.parameters.get("value").unwrap(), "12");
    }

    #[test]
    fn test_parse_experiment_with_lens() {
        let cmd = parse_natural_language("의식 렌즈 장력 실험");
        assert_eq!(cmd.action, "experiment");
        assert_eq!(cmd.parameters.get("lens"), Some(&"consciousness".to_string()));
    }

    #[test]
    fn test_parse_search() {
        let cmd = parse_natural_language("최근 발견 보여줘");
        assert_eq!(cmd.action, "search");
    }

    #[test]
    fn test_to_cli_args_scan() {
        let cmd = parse_natural_language("physics 전체 스캔");
        let args = to_cli_args(&cmd);
        assert!(args.contains(&"scan".to_string()));
        assert!(args.contains(&"physics".to_string()));
        assert!(args.contains(&"--full".to_string()));
    }

    #[test]
    fn test_to_cli_args_verify() {
        let cmd = parse_natural_language("verify 24.0 확인");
        let args = to_cli_args(&cmd);
        assert!(args.contains(&"verify".to_string()));
        // Should contain the numeric value
        assert!(args.iter().any(|a| a.parse::<f64>().is_ok()));
    }

    #[test]
    fn test_roundtrip_help() {
        let cmd = parse_natural_language("도움");
        assert_eq!(cmd.action, "help");
        let args = to_cli_args(&cmd);
        assert_eq!(args, vec!["nexus", "help"]);
    }
}
