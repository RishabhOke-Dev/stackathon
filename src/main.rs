use std::env;

use stackathon::{compile_file, run_file};




fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: stackathon <file-path>");
        std::process::exit(1);
    }
    let filepath = &args[1];
    if args.len() > 2 {
        if args[2] != "--lib" {
            eprintln!("Unknown argument '{}'", args[2]);
            std::process::exit(1);
        }
        compile_file(filepath);
    } else {
        run_file(&filepath);
    }
}
