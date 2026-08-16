use crate::error::TextkitError;

#[derive(Debug, Clone)]
pub enum Command {
    Search { pattern: String, file: String },
    Count { file: String },
    Case { mode: CaseMode, file: String },
}

#[derive(Debug, Clone)]
pub enum CaseMode {
    Upper,
    Lower,
    Title,
}

pub struct Config {
    pub command: Command,
}

impl Config {
    pub fn parse(args: &[String]) -> Result<Config, TextkitError> {
        let subcommand = args.get(1).ok_or_else(|| {
            TextkitError::MissingArgument("subcommand (search|count|case)".to_string())
        })?;

        let command = match subcommand.as_str() {
            "search" => {
                let pattern = args
                    .get(2)
                    .ok_or_else(|| TextkitError::MissingArgument("pattern".to_string()))?;
                let file = args
                    .get(3)
                    .ok_or_else(|| TextkitError::MissingArgument("file".to_string()))?;
                Command::Search {
                    pattern: pattern.clone(),
                    file: file.clone(),
                }
            }
            "count" => {
                let file = args
                    .get(2)
                    .ok_or_else(|| TextkitError::MissingArgument("file".to_string()))?;
                Command::Count { file: file.clone() }
            }
            "case" => {
                let mode_str = args.get(2).ok_or_else(|| {
                    TextkitError::MissingArgument("mode (upper|lower|title)".to_string())
                })?;
                let file = args
                    .get(3)
                    .ok_or_else(|| TextkitError::MissingArgument("file".to_string()))?;

                let mode = match mode_str.as_str() {
                    "upper" => CaseMode::Upper,
                    "lower" => CaseMode::Lower,
                    "title" => CaseMode::Title,
                    other => return Err(TextkitError::UnknownCommand(other.to_string())),
                };

                Command::Case {
                    mode,
                    file: file.clone(),
                }
            }
            other => return Err(TextkitError::UnknownCommand(other.to_string())),
        };

        Ok(Config { command })
    }
}
