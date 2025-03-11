use hashpass::file_handler::{read_wordlist, FileError};
use std::fs::write;
use tempfile::NamedTempFile;

#[test]
fn test_read_wordlist_valid_file() {
    let temp_file = NamedTempFile::new().unwrap();
    write(temp_file.path(), b"password1\npassword2\npassword3\n").unwrap();

    let result: Vec<_> = read_wordlist(temp_file.path().to_str().unwrap())
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    assert_eq!(result, vec!["password1", "password2", "password3"]);
}

#[test]
fn test_read_wordlist_empty_lines() {
    let temp_file = NamedTempFile::new().unwrap();
    write(temp_file.path(), b"password1\n\npassword2\n  \n").unwrap();

    let result: Vec<_> = read_wordlist(temp_file.path().to_str().unwrap())
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    assert_eq!(result, vec!["password1", "password2"]);
}

#[test]
fn test_read_wordlist_nonexistent_file() {
    let result = read_wordlist("nonexistent_file.txt");
    assert!(result.is_err());
}