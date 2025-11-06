
use std::collections::HashMap;

use crate::{lexer::{tokenize, Token, TokenizerError}, vm::{execute, RuntimeError}};



mod lexer;
mod vm;
mod types;


pub fn run_file(filepath: &str) {
    
    let source = match std::fs::read_to_string(filepath) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filepath, e);
            return;
        }
    };


    let mut functions: HashMap<String, Vec<Token>> = HashMap::new();

    let tokens = match tokenize(&source, None, &mut functions) {
        Ok(t) => t,

        Err(error) => {
            match error {
                TokenizerError::InvalidNumberFormat(ref pos) => {
                    print_error(&source, &error.to_string(), pos.row, pos.col);
                },
                TokenizerError::UnexpectedSymbol(ref pos, _)  => {
                    print_error(&source, &error.to_string(), pos.row, pos.col);
                },
                TokenizerError::UnknownIdentifier(ref pos, _) => {
                    print_error(&source, &error.to_string(), pos.row, pos.col);
                },
                TokenizerError::BlockHadNoEnd(ref pos) => {
                    print_error(&source, &error.to_string(), pos.row, pos.col);
                },
                TokenizerError::StringHadNoEnd(ref pos) => {
                    print_error(&source, &error.to_string(), pos.row, pos.col);
                },
                TokenizerError::FunctionHasNoDefinition(ref pos, _) => {
                    print_error(&source, &error.to_string(), pos.row, pos.col);
                },
                TokenizerError::FunctionHasMultipleDefinitions(ref pos, _) => {
                    print_error(&source, &error.to_string(), pos.row, pos.col);
                }
            }
            return;
        }
    };
    //println!("{:#?}", functions);
    //println!("Tokenized with tokens {:#?}", tokens);


    if let Err(e) = execute(&tokens, None, &functions) {
        match e {
            RuntimeError::OperatorInvalidValues(ref pos, _) => 
                print_error(&source, &e.to_string(), pos.row, pos.col),
            RuntimeError::KeywordInvalidValues(ref pos, _) =>
                print_error(&source, &e.to_string(), pos.row, pos.col),
        }
    }

}



//Helper function
fn print_error(source: &str, error_description: &str, error_row: usize, error_col: usize) {
    eprintln!("{}", error_description);
    eprintln!("{}", source.lines().nth(error_row - 1).unwrap());
    eprintln!("{}\x1b[38;5;196m^\x1b[0m", " ".repeat(error_col - 1));
    //if it seems complicated: print col - 1 spaces. Then switch color to red, print '^' then reset color
    return;
}