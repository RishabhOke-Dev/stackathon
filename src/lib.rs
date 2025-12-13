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
//! * 'serial': Handles serializing libraries efficiently



use std::{collections::HashMap, fs::File, io::{BufWriter, Write}};

use crate::{lexer::{Token, tokenize}, serial::ByteSized, vm::execute};



mod lexer;
mod vm;
mod types;
mod serial;

/// Used for libraries
/// 
/// `a.b.c` turns into `abc`.
/// 
/// `2.3.4` becomes `234`.
/// 
/// `0.3.5` becomes `35`.
const VERSION: u32 = 50;

///Used when turning a stackathon file into a lib file
/// 
/// **Arguments**
/// * `filepath`: The stackathon file to compile
pub fn compile_file(filepath: &str) {
    let source = match std::fs::read_to_string(filepath) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filepath, e);
            return;
        }
    };
    compile_string(source, filepath);
}

///Used when turning a stackathon code into a lib file
/// 
/// **Arguments**
/// * `source`: The stackathon source code to compile
/// * `libname`: The name of the new lib file
pub fn compile_string(source: String, libname: &str) {
    let mut functions: HashMap<String, Vec<Token>> = HashMap::new();

    if let Err(error) = tokenize(&source, None, &mut functions) {
       
        let pos = error.position();
        print_error(&source, &error.to_string(), pos.row, pos.col);
        return;
    
    }

    //Instead of executing the code, we serialze the function table.

    let filename = libname.to_string() + ".lib";

    let library = match File::create(filename) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error making library file: {}", e);
            return;
        }
    };

    let mut writer = BufWriter::new(library);
    //We will use the writer later
    //For now we will make a buffer
    let mut buffer = Vec::new();
    //Magic number (4 bytes)
    buffer.extend_from_slice(b"STKL");
    //Version (4 bytes)
    buffer.extend_from_slice(&VERSION.to_be_bytes());
    //How many functions/tags there are (4 bytes)
    buffer.extend_from_slice(&(functions.len() as u32).to_be_bytes());

    //End of header (12 bytes)

    //write each key-value pair
    for function in functions {

        //key serialization

        let key_length = function.0.len();
        //length of the key (4 bytes)
        buffer.extend_from_slice(&(key_length as u32).to_be_bytes());
        //the key
        buffer.extend_from_slice(function.0.as_bytes());
        
        //value serialization

        let data_length = function.1.len();
        //length of data (4 bytes)
        buffer.extend_from_slice(&(data_length as u32).to_be_bytes());
        //the data
        for token in function.1 {
            buffer.extend_from_slice(&token.to_bytes());
        }
    }

    if let Err(error) = writer.write_all(&buffer) {
        eprintln!("Error writing library to file: {}", error);
        return;
    }

    if let Err(error) = writer.flush() {
        eprintln!("Error flushing data to file: {}", error);
        return;
    }

    

}
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