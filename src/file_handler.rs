use std::fs::File;
use std::io::{self, BufRead, BufReader};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileError {
    #[error("Failed to open file: {0}")]
    FileOpenError(#[from] io::Error),
    #[error("Invalid line in wordlist: {0}")]
    InvalidLine(String),
}

/// Reads a wordlist file and returns an iterator over its lines
pub fn read_wordlist(file_path: &str) -> Result<impl Iterator<Item = Result<String, FileError>>, FileError> {
    let file = File::open(file_path).map_err(FileError::FileOpenError)?;
    let reader = BufReader::new(file);
    
    Ok(reader.lines().map(|line| {
        line.map_err(FileError::FileOpenError)
            .and_then(|l| {
                if l.trim().is_empty() {
                    Err(FileError::InvalidLine("Empty line found".to_string()))
                } else {
                    Ok(l)
                }
            })
    }))
}