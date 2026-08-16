use crate::commands::Processor;
use crate::config::CaseMode;
use crate::error::TextkitError;
use std::fs;

pub struct CaseCommand {
    pub mode: CaseMode,
    pub file: String,
}

impl Processor for CaseCommand {
    fn run(&self) -> Result<(), TextkitError> {
        let contents = fs::read_to_string(&self.file)?;

        let transformed: String = match self.mode {
            CaseMode::Upper => contents.to_uppercase(),
        
            CaseMode::Lower => contents.to_lowercase(),
        
            CaseMode::Title => contents
                .split_whitespace()
                .map(|word| {
                    let mut chars = word.chars();
        
                    match chars.next() {
                        Some(first) => {
                            first.to_uppercase().collect::<String>()
                                + chars.as_str().to_lowercase().as_str()
                        }
                        None => String::new(),
                    }
                })
                .collect::<Vec<String>>()
                .join(" "),
        };

        println!("{transformed}");
        Ok(())
    }
}