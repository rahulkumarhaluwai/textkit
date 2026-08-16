use crate::commands::Processor;
use crate::error::TextkitError;
use std::fs;
use crate::commands::SearchMatch;

pub struct SearchCommand {
    pub pattern: String,
    pub file: String,
}

impl Processor for SearchCommand {
    fn run(&self) -> Result<(), TextkitError> {
        let contents = fs::read_to_string(&self.file)?;
        
                let matches: Vec<SearchMatch<'_>> = contents
                    .lines()
                    .enumerate()
                    .filter_map(|(index, line)| {
                        if line.contains(&self.pattern) {
                            Some(SearchMatch {
                                line_number: index + 1,
                                line,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
        
                for matched in matches {
                    println!("{}: {}", matched.line_number, matched.line);
                }
        
                Ok(())
    }
}