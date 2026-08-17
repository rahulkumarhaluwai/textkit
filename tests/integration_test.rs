use textkit::config::{CaseMode, Command, Config};

fn args(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|s| s.to_string()).collect()
}

#[test]
fn parses_case_command() {
    let config = Config::parse(&args(&["textkit", "case", "upper", "sample.txt"]))
        .expect("valid case command should parse");

    assert_eq!(
        config.command,
        Command::Case { mode: CaseMode::Upper, file: "sample.txt".to_string() }
    );
}

#[test]
fn parses_search_command() {
    let config = Config::parse(&args(&["textkit", "search", "hello", "sample.txt"]))
        .expect("valid search command should parse");

    assert_eq!(
        config.command,
        Command::Search { pattern: "hello".to_string(), file: "sample.txt".to_string() }
    );
}

#[test]
fn parses_count_command() {
    let config = Config::parse(&args(&["textkit", "count", "sample.txt"]))
        .expect("valid count command should parse");

    assert_eq!(config.command, Command::Count { file: "sample.txt".to_string() });
}

#[test]
fn missing_subcommand_is_an_error() {
    let result = Config::parse(&args(&["textkit"]));
    assert!(result.is_err());
}

#[test]
fn unknown_subcommand_is_an_error() {
    let result = Config::parse(&args(&["textkit", "frobnicate", "sample.txt"]));
    assert!(result.is_err());
}

#[test]
fn missing_file_argument_is_an_error() {
    let result = Config::parse(&args(&["textkit", "count"]));
    assert!(result.is_err());
}

#[test]
fn unknown_case_mode_is_an_error() {
    let result = Config::parse(&args(&["textkit", "case", "sideways", "sample.txt"]));
    assert!(result.is_err());
}