//! # The Stackathon Library
//! **Holds all the code to run stackthon snippets and files**
//! 
//! This library makes it easy to use stackathon in any application
//! 
//! ## Usage
//! 
//! Usually you will use the run_string() function.
//! ```
//! use stackathon::run_string;
//! 
//! run_string("2 2 + print".to_string());
//! //Expected Output: 4
//! run_string("\"hello \" \"world\" + print".to_string());
//! //Expected Output: "hello world"
//! ```
//! ## Modules
//! * 'lexer': Handles tokenizing valid stackathon source code
//! * 'vm': Handles running the tokens given by the lexer
//! * 'types': Defines types used throughout the library
//! 



use std::collections::HashMap;

use crate::{lexer::{tokenize, Token}, vm::execute};



mod lexer;
mod vm;
mod types;

/// Used when running stackthon code from a file.
/// 
/// **Arguments:**
/// * `filepath`: The path to the source file to run
pub fn run_file(filepath: &str) {
    
    let source = match std::fs::read_to_string(filepath) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filepath, e);
            return;
        }
    };

    run_string(source);
    

}

/// Used when running stackathon code from a string
/// 
/// **Arguments:**
/// * `source`: The stackathon source code to run
pub fn run_string(source: String) {
    let mut functions: HashMap<String, Vec<Token>> = HashMap::new();

    let tokens = match tokenize(&source, None, &mut functions) {
        Ok(t) => t,

        Err(error) => {
            let pos = error.position();
            print_error(&source, &error.to_string(), pos.row, pos.col);
            return;
        }
    };
    //println!("{:#?}", functions);
    //println!("Tokenized with tokens {:#?}", tokens);

    let mut stack = vm::Stack::new();
    if let Err(e) = execute(&tokens, &mut stack, &functions) {
        let pos = e.position();
        print_error(&source, &e.to_string(), pos.row, pos.col);
    }
}


/// A helper function that reduces the code required to print an error
/// 
/// Gives the helpful arrow to the error
/// 
/// **Arguments:**
/// * `source`: The source file from which the error came from
/// * `error_description`: The error's description, printed
/// * `error_row`: The position row where the error happened
/// * `error_col`: The position column where the error happened
fn print_error(source: &str, error_description: &str, error_row: usize, error_col: usize) {
    eprintln!("{}", error_description);
    eprintln!("{}", source.lines().nth(error_row - 1).unwrap());
    eprintln!("{}\x1b[38;5;196m^\x1b[0m", " ".repeat(error_col - 1));
    //if it seems complicated: print col - 1 spaces. Then switch color to red, print '^' then reset color
    return;
}