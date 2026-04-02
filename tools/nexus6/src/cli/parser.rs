/// CLI argument parser — no external crates, pure std::env::args().

/// Parsed CLI command with all options.
#[derive(Debug, Clone, PartialEq)]
pub enum CliCommand {
    Scan {
        domain: String,
        lenses: Option<Vec<String>>,
        full: bool,
    },
    Verify {
        value: f64,
        tolerance: Option<f64>,
    },
    Graph {
        domain: Option<String>,
        format: GraphFormat,
    },
    History {
        domain: String,
    },
    Recommend {
        domain: String,
    },
    Evolve {
        domain: String,
        max_cycles: usize,
        seeds: Vec<String>,
    },
    Auto {
        domain: String,
        max_meta_cycles: usize,
        max_ouroboros_cycles: usize,
    },
    Lenses {
        category: Option<LensFilter>,
        domain: Option<String>,
        search: Option<String>,
        count_only: bool,
        complementary: Option<String>,
        export_json: bool,
    },
    Bench,
    Dashboard {
        html: bool,
        output: Option<String>,
    },
    Help,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GraphFormat {
    Ascii,
    Dot,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LensFilter {
    Core,
    Combo,
    Extended,
    Custom,
}

/// Parse command-line arguments into a CliCommand.
///
/// `args` should be the full args list (args[0] = program name).
pub fn parse_args(args: &[String]) -> Result<CliCommand, String> {
    if args.len() < 2 {
        return Ok(CliCommand::Help);
    }

    let sub = args[1].as_str();
    let rest = &args[2..];

    match sub {
        "scan" => parse_scan(rest),
        "verify" => parse_verify(rest),
        "graph" => parse_graph(rest),
        "history" => parse_history(rest),
        "recommend" => parse_recommend(rest),
        "evolve" => parse_evolve(rest),
        "auto" => parse_auto(rest),
        "lenses" => parse_lenses(rest),
        "bench" => Ok(CliCommand::Bench),
        "dashboard" => parse_dashboard(rest),
        "help" | "--help" | "-h" => Ok(CliCommand::Help),
        other => Err(format!("Unknown command: '{}'. Run 'nexus6 help' for usage.", other)),
    }
}

fn parse_scan(args: &[String]) -> Result<CliCommand, String> {
    if args.is_empty() {
        return Err("scan requires a <domain> argument".to_string());
    }
    let domain = args[0].clone();
    let mut lenses: Option<Vec<String>> = None;
    let mut full = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--lenses" => {
                i += 1;
                if i >= args.len() {
                    return Err("--lenses requires a comma-separated list".to_string());
                }
                lenses = Some(
                    args[i]
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect(),
                );
            }
            "--full" => {
                full = true;
            }
            other => {
                return Err(format!("scan: unknown option '{}'", other));
            }
        }
        i += 1;
    }

    Ok(CliCommand::Scan { domain, lenses, full })
}

fn parse_verify(args: &[String]) -> Result<CliCommand, String> {
    if args.is_empty() {
        return Err("verify requires a <value> argument".to_string());
    }
    let value: f64 = args[0]
        .parse()
        .map_err(|_| format!("verify: '{}' is not a valid number", args[0]))?;

    let mut tolerance: Option<f64> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--tolerance" => {
                i += 1;
                if i >= args.len() {
                    return Err("--tolerance requires a value".to_string());
                }
                tolerance = Some(
                    args[i]
                        .parse()
                        .map_err(|_| format!("--tolerance: '{}' is not a valid number", args[i]))?,
                );
            }
            other => {
                return Err(format!("verify: unknown option '{}'", other));
            }
        }
        i += 1;
    }

    Ok(CliCommand::Verify { value, tolerance })
}

fn parse_graph(args: &[String]) -> Result<CliCommand, String> {
    let mut domain: Option<String> = None;
    let mut format = GraphFormat::Ascii;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--domain" => {
                i += 1;
                if i >= args.len() {
                    return Err("--domain requires a value".to_string());
                }
                domain = Some(args[i].clone());
            }
            "--format" => {
                i += 1;
                if i >= args.len() {
                    return Err("--format requires 'ascii' or 'dot'".to_string());
                }
                format = match args[i].as_str() {
                    "ascii" => GraphFormat::Ascii,
                    "dot" => GraphFormat::Dot,
                    other => {
                        return Err(format!("--format: unknown format '{}' (use ascii or dot)", other));
                    }
                };
            }
            other => {
                return Err(format!("graph: unknown option '{}'", other));
            }
        }
        i += 1;
    }

    Ok(CliCommand::Graph { domain, format })
}

fn parse_history(args: &[String]) -> Result<CliCommand, String> {
    if args.is_empty() {
        return Err("history requires a <domain> argument".to_string());
    }
    Ok(CliCommand::History {
        domain: args[0].clone(),
    })
}

fn parse_recommend(args: &[String]) -> Result<CliCommand, String> {
    if args.is_empty() {
        return Err("recommend requires a <domain> argument".to_string());
    }
    Ok(CliCommand::Recommend {
        domain: args[0].clone(),
    })
}

fn parse_evolve(args: &[String]) -> Result<CliCommand, String> {
    if args.is_empty() {
        return Err("evolve requires a <domain> argument".to_string());
    }
    let domain = args[0].clone();
    let mut max_cycles: usize = 6; // n=6 default
    let mut seeds: Vec<String> = Vec::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--max-cycles" => {
                i += 1;
                if i >= args.len() {
                    return Err("--max-cycles requires a number".to_string());
                }
                max_cycles = args[i]
                    .parse()
                    .map_err(|_| format!("--max-cycles: '{}' is not a valid number", args[i]))?;
            }
            "--seeds" => {
                i += 1;
                if i >= args.len() {
                    return Err("--seeds requires a comma-separated list".to_string());
                }
                seeds = args[i]
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            other => {
                return Err(format!("evolve: unknown option '{}'", other));
            }
        }
        i += 1;
    }

    Ok(CliCommand::Evolve {
        domain,
        max_cycles,
        seeds,
    })
}

fn parse_auto(args: &[String]) -> Result<CliCommand, String> {
    if args.is_empty() {
        return Err("auto requires a <domain> argument".to_string());
    }
    let domain = args[0].clone();
    let mut max_meta_cycles: usize = 6;       // n=6 default
    let mut max_ouroboros_cycles: usize = 6;   // n=6 default

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--meta-cycles" => {
                i += 1;
                if i >= args.len() {
                    return Err("--meta-cycles requires a number".to_string());
                }
                max_meta_cycles = args[i]
                    .parse()
                    .map_err(|_| format!("--meta-cycles: '{}' is not a valid number", args[i]))?;
            }
            "--ouroboros-cycles" => {
                i += 1;
                if i >= args.len() {
                    return Err("--ouroboros-cycles requires a number".to_string());
                }
                max_ouroboros_cycles = args[i]
                    .parse()
                    .map_err(|_| format!("--ouroboros-cycles: '{}' is not a valid number", args[i]))?;
            }
            other => {
                return Err(format!("auto: unknown option '{}'", other));
            }
        }
        i += 1;
    }

    Ok(CliCommand::Auto {
        domain,
        max_meta_cycles,
        max_ouroboros_cycles,
    })
}

fn parse_lenses(args: &[String]) -> Result<CliCommand, String> {
    let mut category: Option<LensFilter> = None;
    let mut domain: Option<String> = None;
    let mut search: Option<String> = None;
    let mut count_only = false;
    let mut complementary: Option<String> = None;
    let mut export_json = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--category" => {
                i += 1;
                if i >= args.len() {
                    return Err("--category requires one of: core, combo, extended, custom".to_string());
                }
                category = Some(match args[i].as_str() {
                    "core" => LensFilter::Core,
                    "combo" => LensFilter::Combo,
                    "extended" => LensFilter::Extended,
                    "custom" => LensFilter::Custom,
                    other => {
                        return Err(format!(
                            "--category: unknown '{}' (use core, combo, extended, custom)",
                            other
                        ));
                    }
                });
            }
            "--domain" => {
                i += 1;
                if i >= args.len() {
                    return Err("--domain requires a domain name".to_string());
                }
                domain = Some(args[i].clone());
            }
            "--search" => {
                i += 1;
                if i >= args.len() {
                    return Err("--search requires a keyword".to_string());
                }
                search = Some(args[i].clone());
            }
            "--count" => {
                count_only = true;
            }
            "--complementary" => {
                i += 1;
                if i >= args.len() {
                    return Err("--complementary requires a lens name".to_string());
                }
                complementary = Some(args[i].clone());
            }
            "--export" => {
                i += 1;
                if i >= args.len() {
                    return Err("--export requires a format (json)".to_string());
                }
                match args[i].as_str() {
                    "json" => { export_json = true; }
                    other => {
                        return Err(format!("--export: unknown format '{}' (use json)", other));
                    }
                }
            }
            other => {
                return Err(format!("lenses: unknown option '{}'", other));
            }
        }
        i += 1;
    }

    Ok(CliCommand::Lenses { category, domain, search, count_only, complementary, export_json })
}

fn parse_dashboard(args: &[String]) -> Result<CliCommand, String> {
    let mut html = false;
    let mut output: Option<String> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--html" => {
                html = true;
            }
            "--output" | "-o" => {
                i += 1;
                if i >= args.len() {
                    return Err("--output requires a file path".to_string());
                }
                output = Some(args[i].clone());
            }
            other => {
                return Err(format!("dashboard: unknown option '{}'", other));
            }
        }
        i += 1;
    }

    Ok(CliCommand::Dashboard { html, output })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(s: &str) -> Vec<String> {
        s.split_whitespace().map(|w| w.to_string()).collect()
    }

    #[test]
    fn test_parse_scan() {
        let cmd = parse_args(&args("nexus6 scan physics --lenses consciousness,topology")).unwrap();
        assert_eq!(
            cmd,
            CliCommand::Scan {
                domain: "physics".to_string(),
                lenses: Some(vec!["consciousness".to_string(), "topology".to_string()]),
                full: false,
            }
        );
    }

    #[test]
    fn test_parse_scan_full() {
        let cmd = parse_args(&args("nexus6 scan biology --full")).unwrap();
        assert_eq!(
            cmd,
            CliCommand::Scan {
                domain: "biology".to_string(),
                lenses: None,
                full: true,
            }
        );
    }

    #[test]
    fn test_parse_verify() {
        let cmd = parse_args(&args("nexus6 verify 12.0")).unwrap();
        assert_eq!(
            cmd,
            CliCommand::Verify {
                value: 12.0,
                tolerance: None,
            }
        );
    }

    #[test]
    fn test_parse_verify_tolerance() {
        let cmd = parse_args(&args("nexus6 verify 11.9 --tolerance 0.05")).unwrap();
        assert_eq!(
            cmd,
            CliCommand::Verify {
                value: 11.9,
                tolerance: Some(0.05),
            }
        );
    }

    #[test]
    fn test_parse_help() {
        assert_eq!(parse_args(&args("nexus6 help")).unwrap(), CliCommand::Help);
        assert_eq!(parse_args(&args("nexus6")).unwrap(), CliCommand::Help);
        assert_eq!(parse_args(&args("nexus6 --help")).unwrap(), CliCommand::Help);
    }

    #[test]
    fn test_parse_dashboard() {
        assert_eq!(
            parse_args(&args("nexus6 dashboard")).unwrap(),
            CliCommand::Dashboard { html: false, output: None }
        );
    }

    #[test]
    fn test_parse_dashboard_html() {
        assert_eq!(
            parse_args(&args("nexus6 dashboard --html")).unwrap(),
            CliCommand::Dashboard { html: true, output: None }
        );
    }

    #[test]
    fn test_parse_dashboard_html_output() {
        assert_eq!(
            parse_args(&args("nexus6 dashboard --html --output report.html")).unwrap(),
            CliCommand::Dashboard { html: true, output: Some("report.html".to_string()) }
        );
    }

    #[test]
    fn test_parse_bench() {
        assert_eq!(
            parse_args(&args("nexus6 bench")).unwrap(),
            CliCommand::Bench
        );
    }

    #[test]
    fn test_parse_evolve() {
        let cmd = parse_args(&args("nexus6 evolve physics --max-cycles 10")).unwrap();
        assert_eq!(
            cmd,
            CliCommand::Evolve {
                domain: "physics".to_string(),
                max_cycles: 10,
                seeds: vec![],
            }
        );
    }

    #[test]
    fn test_parse_evolve_seeds() {
        let cmd = parse_args(&args("nexus6 evolve cosmology --seeds BT-97,BT-98 --max-cycles 3")).unwrap();
        assert_eq!(
            cmd,
            CliCommand::Evolve {
                domain: "cosmology".to_string(),
                max_cycles: 3,
                seeds: vec!["BT-97".to_string(), "BT-98".to_string()],
            }
        );
    }

    #[test]
    fn test_parse_lenses() {
        let cmd = parse_args(&args("nexus6 lenses --category core")).unwrap();
        assert_eq!(
            cmd,
            CliCommand::Lenses {
                category: Some(LensFilter::Core),
                domain: None,
                search: None,
                count_only: false,
                complementary: None,
                export_json: false,
            }
        );
    }

    #[test]
    fn test_parse_lenses_all() {
        let cmd = parse_args(&args("nexus6 lenses")).unwrap();
        assert_eq!(
            cmd,
            CliCommand::Lenses {
                category: None,
                domain: None,
                search: None,
                count_only: false,
                complementary: None,
                export_json: false,
            }
        );
    }

    #[test]
    fn test_parse_lenses_search() {
        let cmd = parse_args(&args("nexus6 lenses --search quantum")).unwrap();
        assert_eq!(
            cmd,
            CliCommand::Lenses {
                category: None,
                domain: None,
                search: Some("quantum".to_string()),
                count_only: false,
                complementary: None,
                export_json: false,
            }
        );
    }

    #[test]
    fn test_parse_lenses_domain() {
        let cmd = parse_args(&args("nexus6 lenses --domain physics")).unwrap();
        assert_eq!(
            cmd,
            CliCommand::Lenses {
                category: None,
                domain: Some("physics".to_string()),
                search: None,
                count_only: false,
                complementary: None,
                export_json: false,
            }
        );
    }

    #[test]
    fn test_parse_lenses_count() {
        let cmd = parse_args(&args("nexus6 lenses --count")).unwrap();
        assert_eq!(
            cmd,
            CliCommand::Lenses {
                category: None,
                domain: None,
                search: None,
                count_only: true,
                complementary: None,
                export_json: false,
            }
        );
    }

    #[test]
    fn test_parse_lenses_complementary() {
        let cmd = parse_args(&args("nexus6 lenses --complementary wave")).unwrap();
        assert_eq!(
            cmd,
            CliCommand::Lenses {
                category: None,
                domain: None,
                search: None,
                count_only: false,
                complementary: Some("wave".to_string()),
                export_json: false,
            }
        );
    }

    #[test]
    fn test_parse_lenses_export() {
        let cmd = parse_args(&args("nexus6 lenses --export json")).unwrap();
        assert_eq!(
            cmd,
            CliCommand::Lenses {
                category: None,
                domain: None,
                search: None,
                count_only: false,
                complementary: None,
                export_json: true,
            }
        );
    }

    #[test]
    fn test_parse_graph() {
        let cmd = parse_args(&args("nexus6 graph --domain physics --format dot")).unwrap();
        assert_eq!(
            cmd,
            CliCommand::Graph {
                domain: Some("physics".to_string()),
                format: GraphFormat::Dot,
            }
        );
    }

    #[test]
    fn test_parse_history() {
        let cmd = parse_args(&args("nexus6 history energy")).unwrap();
        assert_eq!(
            cmd,
            CliCommand::History {
                domain: "energy".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_recommend() {
        let cmd = parse_args(&args("nexus6 recommend biology")).unwrap();
        assert_eq!(
            cmd,
            CliCommand::Recommend {
                domain: "biology".to_string(),
            }
        );
    }

    #[test]
    fn test_unknown_command() {
        assert!(parse_args(&args("nexus6 foobar")).is_err());
    }

    #[test]
    fn test_scan_missing_domain() {
        assert!(parse_args(&args("nexus6 scan")).is_err());
    }

    #[test]
    fn test_verify_bad_number() {
        assert!(parse_args(&args("nexus6 verify abc")).is_err());
    }

    #[test]
    fn test_parse_auto_default() {
        let cmd = parse_args(&args("nexus6 auto physics")).unwrap();
        assert_eq!(
            cmd,
            CliCommand::Auto {
                domain: "physics".to_string(),
                max_meta_cycles: 6,
                max_ouroboros_cycles: 6,
            }
        );
    }

    #[test]
    fn test_parse_auto_with_options() {
        let cmd = parse_args(&args("nexus6 auto biology --meta-cycles 3 --ouroboros-cycles 12")).unwrap();
        assert_eq!(
            cmd,
            CliCommand::Auto {
                domain: "biology".to_string(),
                max_meta_cycles: 3,
                max_ouroboros_cycles: 12,
            }
        );
    }

    #[test]
    fn test_parse_auto_missing_domain() {
        assert!(parse_args(&args("nexus6 auto")).is_err());
    }
}
