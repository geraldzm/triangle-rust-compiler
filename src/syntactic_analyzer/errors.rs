use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

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
