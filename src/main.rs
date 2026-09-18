use checkdigit::{gs1, isbn10};
use std::env;
use std::fs;
use std::io::{self, Read};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("validate") => match args.get(2) {
            Some(code) => run_validate(code),
            None => usage(),
        },
        Some("check-digit") => match args.get(2) {
            Some(prefix) => run_check_digit(prefix),
            None => usage(),
        },
        Some("batch") => run_batch(args.get(2).map(String::as_str)),
        _ => usage(),
    }
}

/// Strips the punctuation people actually paste in: hyphens and spaces.
fn cleaned(input: &str) -> String {
    input
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '-')
        .collect()
}

/// Picks the algorithm from the cleaned code's length, same rule `validate`
/// and `batch` both use so a code is judged the same way either place.
fn classify_and_validate(code: &str) -> Result<bool, String> {
    match code.len() {
        10 => isbn10::validate(code).map_err(|e| e.to_string()),
        8 | 12 | 13 | 14 => gs1::validate(code).map_err(|e| e.to_string()),
        n => Err(format!("don't know how to validate a {n}-character code")),
    }
}

fn run_validate(raw: &str) -> ExitCode {
    let code = cleaned(raw);
    match classify_and_validate(&code) {
        Ok(true) => {
            println!("valid");
            ExitCode::SUCCESS
        }
        Ok(false) => {
            println!("invalid");
            ExitCode::FAILURE
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run_check_digit(raw: &str) -> ExitCode {
    let prefix = cleaned(raw);
    let result = match prefix.len() {
        9 => isbn10::check_digit(&prefix).map_err(|e| e.to_string()),
        7 | 11 | 12 | 13 => gs1::check_digit(&prefix).map_err(|e| e.to_string()),
        n => Err(format!(
            "don't know how to compute a check digit for {n} digits"
        )),
    };
    match result {
        Ok(c) => {
            println!("{prefix}{c}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

/// Validates one code per line, either from a file or, with no file given,
/// from stdin. Blank lines and lines starting with '#' are skipped so a
/// codes file can carry its own comments.
fn run_batch(file: Option<&str>) -> ExitCode {
    let input = match file {
        Some(path) => match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("error: {path}: {e}");
                return ExitCode::FAILURE;
            }
        },
        None => {
            let mut buf = String::new();
            if let Err(e) = io::stdin().read_to_string(&mut buf) {
                eprintln!("error: {e}");
                return ExitCode::FAILURE;
            }
            buf
        }
    };

    let mut all_ok = true;
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let code = cleaned(trimmed);
        match classify_and_validate(&code) {
            Ok(true) => println!("{trimmed}: valid"),
            Ok(false) => {
                println!("{trimmed}: invalid");
                all_ok = false;
            }
            Err(e) => {
                println!("{trimmed}: error: {e}");
                all_ok = false;
            }
        }
    }
    if all_ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn usage() -> ExitCode {
    eprintln!("usage:");
    eprintln!("  checkdigit validate <code>       check whether a code's check digit is correct");
    eprintln!("  checkdigit check-digit <prefix>  compute and append the check digit for a prefix");
    eprintln!("  checkdigit batch [file]          validate one code per line, from a file or stdin");
    ExitCode::FAILURE
}
