use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::source_position::SourcePosition;

#[derive(Debug)]
pub struct AscciError;

impl Error for AscciError {}

impl Display for AscciError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid ascci: corrupt contents")
    }
}

//
#[derive(Debug)]
pub struct SyntaxError;

impl Error for SyntaxError {}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Syntax Error found")
    }
}

//
pub struct ErrorReporter {
    num_errors: u32,
}

impl ErrorReporter {
    pub fn new() -> ErrorReporter {
        ErrorReporter { num_errors: 0 }
    }

    pub fn report_error(&mut self, message: &str, token_name: &str, pos: &SourcePosition) {
        println!("ERROR: ");

        message.chars().for_each(|c| {
            if c == '%' {
                print!("{token_name}");
            } else {
                print!("{c}");
            }
        });

        println!(" {pos}");
        self.num_errors += 1;
    }

    pub fn report_restriction(&self, message: &str) {
        println!("RESTRICTION: {message}");
    }

    pub fn get_number_errors(&self) -> u32 {
        self.num_errors
    }
}
