use crate::commands::Processor;
use crate::error::TextkitError;
use std::collections::HashMap;
use std::fs;

pub struct CountCommand {
    pub file: String,
}

impl Processor for CountCommand {
    fn run(&self) -> Result<(), TextkitError> {
        let contents = fs::read_to_string(&self.file)?;
        
                let line_count = contents.lines().count();
                let word_count = contents.split_whitespace().count();
        
                let mut frequencies: HashMap<String, usize> = HashMap::new();
        
                for word in contents.split_whitespace() {
                    let word = word.to_lowercase();
        
                    *frequencies.entry(word).or_insert(0) += 1;
                }
        
                let mut top_words: Vec<(&String, &usize)> = frequencies.iter().collect();
        
                top_words.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
        
                println!("Lines: {line_count}");
                println!("Words: {word_count}");
                println!("Top 5 words:");
        
                for (word, count) in top_words.iter().take(5) {
                    println!("{word}: {count}");
                }
        
                Ok(())
    }
}