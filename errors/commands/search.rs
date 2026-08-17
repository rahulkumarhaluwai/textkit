use crate::commands::{Processor, SearchMatch};
use crate::error::TextkitError;
use std::env;
use std::fs;

pub struct SearchCommand {
    pub pattern: String,
    pub file: String,
}

pub fn search_contents<'a>(
    contents: &'a str,
    pattern: &str,
    case_insensitive: bool,
) -> Vec<SearchMatch<'a>> {
    contents
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let is_match = if case_insensitive {
                line.to_lowercase().contains(&pattern.to_lowercase())
            } else {
                line.contains(pattern)
            };

            if is_match {
                Some(SearchMatch {
                    line_number: index + 1,
                    line,
                })
            } else {
                None
            }
        })
        .collect()
}

impl Processor for SearchCommand {
    fn run(&self) -> Result<(), TextkitError> {
        let contents = fs::read_to_string(&self.file)?;
        let case_insensitive = env::var("TEXTKIT_CASE_INSENSITIVE")
            .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        let matches = search_contents(
            &contents,
            &self.pattern,
            case_insensitive,
        );
        for matched in matches {
            println!("{}: {}", matched.line_number, matched.line);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_matching_lines() {
        let contents = "hello world\nrust is great\nhello rust\n";

        let result = search_contents(contents, "rust", false);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].line_number, 2);
        assert_eq!(result[0].line, "rust is great");
        assert_eq!(result[1].line_number, 3);
        assert_eq!(result[1].line, "hello rust");
    }

    #[test]
    fn search_is_case_sensitive_by_default() {
        let contents = "Rust is great\nrust is fast\nRUST is popular\n";

        let result = search_contents(contents, "Rust", false);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].line, "Rust is great");
    }

    #[test]
    fn case_insensitive_search_finds_all_cases() {
        let contents = "Rust is great\nrust is fast\nRUST is popular\n";

        let result = search_contents(contents, "Rust", true);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].line_number, 1);
        assert_eq!(result[1].line_number, 2);
        assert_eq!(result[2].line_number, 3);
    }

    #[test]
    fn returns_empty_when_no_match() {
        let contents = "hello world\npython is great\n";

        let result = search_contents(contents, "rust", false);

        assert_eq!(result.len(), 0);
    }
}