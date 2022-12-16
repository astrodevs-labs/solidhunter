use regex::{Match, Regex};
use once_cell::sync::Lazy;
use crate::solc::parsing_error::{ErrorLocation};

pub static RE_SOL_ERROR_PRINT: Lazy<Regex> = Lazy::new(|| Regex::new(r"Error: (.*)\n").unwrap());
pub static RE_SOL_ERROR_FILE: Lazy<Regex> = Lazy::new(|| Regex::new(r"--> (?P<file>.*):(?P<line>\d+):(?P<column>\d+):").unwrap());
pub static RE_SOL_PRAGMA_VERSION: Lazy<Regex> = Lazy::new(|| Regex::new(r"pragma\s+solidity\s+(?P<version>\^?\d+\.\d+(?:\.\d+)?);").unwrap());

pub fn find_version_pragma(contract: &str) -> Option<Match> {
    RE_SOL_PRAGMA_VERSION.captures(contract)?.name("version")
}

fn count_length(error: String) -> usize {
    let mut count = 0;
    let mut n = error.len() - 3;

    while n > 0 {
        if error.chars().nth(n).unwrap() == '\n' {
            return count;
        }
        if error.chars().nth(n).unwrap() == '^' {
            count += 1;
        }
        n-=1;
    }
    return count;
}

pub fn get_error_message(stderr: &str) -> Result<String, ()> {
    let error = RE_SOL_ERROR_PRINT.captures(stderr);
    let error = match error {
        Some(error) => error.get(0),
        None => return Err(())
    };
    let error = match error {
        Some(error) => error.as_str(),
        None => return Err(())
    };
    let error = error.to_string();
    Ok(error)
}

pub fn get_error_location(stderr: &str) -> Result<ErrorLocation, ()> {
    let error = RE_SOL_ERROR_FILE.captures(stderr);
    let error = match error {
        Some(error) => error,
        None => return Err(())
    };
    let file = match error.name("file") {
        Some(file) => file.as_str().to_string(),
        None => return Err(())
    };
    let line = match error.name("line") {
        Some(line) => line.as_str().to_string(),
        None => return Err(())
    };
    let column = match error.name("column") {
        Some(line) => line.as_str().to_string(),
        None => return Err(())
    };
    let length = count_length(stderr.to_string());

    Ok(
        ErrorLocation {
            file,
            line: line.parse().unwrap(), //unwrap is safe due to the regex that matches only number
            column: column.parse().unwrap(), //unwrap is safe due to the regex that matches only number
            length: length
        }
    )
}