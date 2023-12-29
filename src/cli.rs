use std::error::Error;

use crate::syntactic_analyzer::source_file::SourceFile;

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

    for c in source_file {
        println!("{c}");
    }

    Ok(())
}
