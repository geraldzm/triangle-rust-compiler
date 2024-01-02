use super::errors::AscciError;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub const EOL: char = '\n';
/// [End Of Transmission](http://en.wikipedia.org/wiki/End-of-Transmission_character)
pub const EOT: char = '\u{0004}';

pub struct SourceFile {
    file_content: String,
    current_line: usize,
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

        Ok(SourceFile {
            file_content,
            current_line: 1,
        })
    }

    pub fn get_current_line(&self) -> usize {
        self.current_line
    }
}

impl Iterator for SourceFile {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.file_content.pop() {
            Some(c) => {
                if c == EOL {
                    self.current_line += 1;
                }

                Some(c)
            }
            _ => None,
        }
    }
}
