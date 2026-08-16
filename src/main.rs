use std::process;
use textkit::commands;
use textkit::Config;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = match Config::parse(&args) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    };

    let processor = commands::build_processor(&config);

    if let Err(e) = processor.run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}