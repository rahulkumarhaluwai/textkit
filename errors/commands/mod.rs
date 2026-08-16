pub mod case;
pub mod count;
pub mod search;

use crate::config::{Command, Config};
use crate::error::TextkitError;
pub trait Processor {
    fn run(&self) -> Result<(), TextkitError>;
}

pub struct SearchMatch<'a> {
    pub line_number: usize,
    pub line: &'a str,
}

pub fn build_processor(config: &Config) -> Box<dyn Processor> {
    match config.command.clone() {
        Command::Search { pattern, file } => Box::new(search::SearchCommand { pattern, file }),
        Command::Count { file } => Box::new(count::CountCommand { file }),
        Command::Case { mode, file } => Box::new(case::CaseCommand { mode, file }),
    }
}