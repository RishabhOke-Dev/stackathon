use std::env;

use stackathon::run_file;




fn main() {
    let filepath = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: stackathon <file-path>");
        std::process::exit(1);
    });
    //println!("Running file {}...", filepath);
    run_file(&filepath);
}
