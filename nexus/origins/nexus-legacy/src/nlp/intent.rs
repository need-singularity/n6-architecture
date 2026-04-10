/// Intent classification for natural language queries.

/// Classified user intent.
#[derive(Debug, Clone, PartialEq)]
pub enum Intent {
    Scan { domain: String, full: bool },
    Verify { value: f64 },
    Experiment { exp_type: String, target: String },
    Search { query: String },
    Help,
    Unknown(String),
}

/// Classify a natural language text into an Intent.
///
/// Supports Korean and English keywords:
///   scan/찾아/스캔/탐색 -> Scan
///   verify/검증/확인 -> Verify
///   experiment/실험 -> Experiment
///   search/보여줘/검색 -> Search
///   help/도움 -> Help
pub fn classify_intent(text: &str) -> Intent {
    let lower = text.to_lowercase();

    // Help
    if contains_any(&lower, &["help", "도움", "도와줘", "사용법"]) {
        return Intent::Help;
    }

    // Verify — look for a number after verify keywords
    if contains_any(&lower, &["verify", "검증", "확인", "관련있나"]) {
        if let Some(val) = extract_first_number(text) {
            return Intent::Verify { value: val };
        }
    }

    // Experiment
    if contains_any(&lower, &["experiment", "실험"]) {
        let exp_type = extract_experiment_type(&lower);
        let target = extract_domain_keyword(&lower).unwrap_or_else(|| "general".to_string());
        return Intent::Experiment { exp_type, target };
    }

    // Scan
    if contains_any(&lower, &["scan", "찾아", "스캔", "탐색", "이상", "패턴"]) {
        let domain = extract_domain_keyword(&lower).unwrap_or_else(|| "general".to_string());
        let full = contains_any(&lower, &["전체", "full", "전수", "풀"]);
        return Intent::Scan { domain, full };
    }

    // Search
    if contains_any(&lower, &["search", "보여줘", "검색", "최근", "발견"]) {
        let query = extract_search_query(text);
        return Intent::Search { query };
    }

    Intent::Unknown(text.to_string())
}

fn contains_any(text: &str, keywords: &[&str]) -> bool {
    keywords.iter().any(|kw| text.contains(kw))
}

fn extract_first_number(text: &str) -> Option<f64> {
    for token in text.split_whitespace() {
        // Strip common punctuation
        let cleaned: String = token.chars().filter(|c| c.is_ascii_digit() || *c == '.' || *c == '-').collect();
        if !cleaned.is_empty() {
            if let Ok(val) = cleaned.parse::<f64>() {
                return Some(val);
            }
        }
    }
    None
}

/// Known domains for extraction.
const KNOWN_DOMAINS: &[&str] = &[
    "physics", "물리", "biology", "생물", "chemistry", "화학",
    "energy", "에너지", "chip", "칩", "semiconductor", "반도체",
    "fusion", "핵융합", "cosmology", "우주", "quantum", "양자",
    "ai", "battery", "배터리", "solar", "태양", "plasma", "플라즈마",
    "robotics", "로봇", "crypto", "암호", "network", "네트워크",
    "material", "소재", "display", "디스플레이", "audio", "오디오",
    "math", "수학", "software", "소프트웨어", "environment", "환경",
];

fn extract_domain_keyword(text: &str) -> Option<String> {
    // Map Korean domains to English
    let korean_map: &[(&str, &str)] = &[
        ("물리", "physics"), ("생물", "biology"), ("화학", "chemistry"),
        ("에너지", "energy"), ("칩", "chip"), ("반도체", "semiconductor"),
        ("핵융합", "fusion"), ("우주", "cosmology"), ("양자", "quantum"),
        ("배터리", "battery"), ("태양", "solar"), ("플라즈마", "plasma"),
        ("로봇", "robotics"), ("암호", "crypto"), ("네트워크", "network"),
        ("소재", "material"), ("디스플레이", "display"), ("오디오", "audio"),
        ("수학", "math"), ("소프트웨어", "software"), ("환경", "environment"),
    ];

    for &(kr, en) in korean_map {
        if text.contains(kr) {
            return Some(en.to_string());
        }
    }

    for &domain in KNOWN_DOMAINS {
        if text.contains(domain) {
            return Some(domain.to_string());
        }
    }
    None
}

fn extract_experiment_type(text: &str) -> String {
    let exp_types = [
        "tension", "장력", "fusion", "핵융합", "phase", "위상",
        "resonance", "공명", "symmetry", "대칭",
    ];
    let type_map: &[(&str, &str)] = &[
        ("장력", "tension"), ("핵융합", "fusion"), ("위상", "phase"),
        ("공명", "resonance"), ("대칭", "symmetry"),
    ];
    for &(kr, en) in type_map {
        if text.contains(kr) {
            return en.to_string();
        }
    }
    for &t in &exp_types {
        if text.contains(t) {
            return t.to_string();
        }
    }
    "tension".to_string() // default
}

fn extract_search_query(text: &str) -> String {
    // Remove common command prefixes and return the rest
    let prefixes = ["search", "보여줘", "검색", "최근", "발견"];
    let mut query = text.to_string();
    for prefix in &prefixes {
        query = query.replace(prefix, "");
    }
    let trimmed = query.trim().to_string();
    if trimmed.is_empty() {
        "recent".to_string()
    } else {
        trimmed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_scan_korean() {
        let intent = classify_intent("물리에서 이상한 패턴 찾아줘");
        match intent {
            Intent::Scan { domain, full } => {
                assert_eq!(domain, "physics");
                assert!(!full);
            }
            _ => panic!("Expected Scan, got {:?}", intent),
        }
    }

    #[test]
    fn test_classify_verify() {
        let intent = classify_intent("12가 n=6과 관련있나 확인");
        match intent {
            Intent::Verify { value } => {
                assert!((value - 12.0).abs() < 0.01);
            }
            _ => panic!("Expected Verify, got {:?}", intent),
        }
    }

    #[test]
    fn test_classify_experiment_korean() {
        let intent = classify_intent("의식 렌즈 장력 실험");
        match intent {
            Intent::Experiment { exp_type, .. } => {
                assert_eq!(exp_type, "tension");
            }
            _ => panic!("Expected Experiment, got {:?}", intent),
        }
    }

    #[test]
    fn test_classify_search() {
        let intent = classify_intent("최근 발견 보여줘");
        match intent {
            Intent::Search { query } => {
                assert!(!query.is_empty());
            }
            _ => panic!("Expected Search, got {:?}", intent),
        }
    }

    #[test]
    fn test_classify_help() {
        assert_eq!(classify_intent("help"), Intent::Help);
        assert_eq!(classify_intent("도움"), Intent::Help);
    }

    #[test]
    fn test_classify_unknown() {
        let intent = classify_intent("random gibberish with no keywords");
        assert!(matches!(intent, Intent::Unknown(_)));
    }

    #[test]
    fn test_classify_scan_full() {
        let intent = classify_intent("physics 전체 스캔");
        match intent {
            Intent::Scan { domain, full } => {
                assert_eq!(domain, "physics");
                assert!(full);
            }
            _ => panic!("Expected Scan, got {:?}", intent),
        }
    }
}
