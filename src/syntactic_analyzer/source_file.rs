use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub struct AscciError;

impl Error for AscciError {}

impl Display for AscciError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid ascci: corrupt contents")
    }
}

pub struct SourceFile {
    file_content: String,
}

impl SourceFile {
    pub fn build(file_path: String) -> Result<SourceFile, Box<dyn Error>> {
        //
        let file = File::open(file_path)?;

        // read file content
        let mut buf_reader = BufReader::new(file);
        let mut file_content = String::new();
        buf_reader.read_to_string(&mut file_content)?;

        // validate ascii
        if !file_content.is_ascii() {
            return Err(Box::new(AscciError));
        }

        let file_content = file_content.chars().rev().collect();

        Ok(SourceFile { file_content })
    }
}

impl Iterator for SourceFile {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.file_content.pop()
    }
}
