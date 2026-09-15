use checkdigit::{gs1, isbn10};
use std::env;
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

fn run_validate(raw: &str) -> ExitCode {
    let code = cleaned(raw);
    let result = match code.len() {
        10 => isbn10::validate(&code).map_err(|e| e.to_string()),
        8 | 12 | 13 | 14 => gs1::validate(&code).map_err(|e| e.to_string()),
        n => Err(format!("don't know how to validate a {n}-character code")),
    };
    match result {
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

fn usage() -> ExitCode {
    eprintln!("usage:");
    eprintln!("  checkdigit validate <code>       check whether a code's check digit is correct");
    eprintln!("  checkdigit check-digit <prefix>  compute and append the check digit for a prefix");
    ExitCode::FAILURE
}
