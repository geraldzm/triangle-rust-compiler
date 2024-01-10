use std::error::Error;

use syntactic_analyzer::{scanner::Scanner, source_file::SourceFile, token};

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn help() -> &'static str {
        "[file_path]"
    }

    pub fn build<I>(mut args: I) -> Result<Config, &'static str>
    where
        I: Iterator<Item = String>,
    {
        args.next(); // ignore program name

        let file_path = match args.next() {
            Some(file_path) => file_path,
            None => return Err("File path arg was not found!"),
        };

        Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let source_file = SourceFile::build(config.file_path)?;
    let mut scanner = Scanner::new(source_file);

    loop {
        let token = scanner.scan();

        match token.kind() {
            token::TokenKind::EOT => {
                println!("EOT found!");
                break;
            }
            token::TokenKind::ERROR => {
                println!("Error found!");
                break;
            }
            _ => {
                println!("Token: {} at line {}", token.spelling(), token.position());
            }
        }
    }

    Ok(())
}
