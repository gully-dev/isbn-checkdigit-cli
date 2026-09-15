//! The GS1 check digit algorithm shared by UPC-A, EAN-8, EAN-13 and ISBN-13.
//!
//! Starting from the digit immediately to the left of the check digit and
//! moving left, weights alternate 3, 1, 3, 1, ... The check digit is whatever
//! makes the weighted sum a multiple of 10. Because the weighting is defined
//! relative to the check digit rather than to the start of the code, the same
//! routine works regardless of how many digits come before it - that's why
//! one function covers every one of these barcode formats.

use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Empty,
    BadDigit(char),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Empty => write!(f, "no digits given"),
            Error::BadDigit(c) => write!(f, "unexpected character {c:?}"),
        }
    }
}

impl std::error::Error for Error {}

/// Computes the check digit for `data`, which must be the digits of the
/// barcode *excluding* the check digit itself.
pub fn check_digit(data: &str) -> Result<char, Error> {
    if data.is_empty() {
        return Err(Error::Empty);
    }
    let mut sum = 0u32;
    let mut weight = 3u32;
    for c in data.chars().rev() {
        let d = c.to_digit(10).ok_or(Error::BadDigit(c))?;
        sum += d * weight;
        weight = if weight == 3 { 1 } else { 3 };
    }
    let check = (10 - sum % 10) % 10;
    Ok(char::from_digit(check, 10).unwrap())
}

/// Validates a full barcode string, check digit included.
pub fn validate(code: &str) -> Result<bool, Error> {
    if code.is_empty() {
        return Err(Error::Empty);
    }
    let (data, check) = code.split_at(code.len() - 1);
    let expected = check_digit(data)?;
    let actual = check.chars().next().unwrap();
    if !actual.is_ascii_digit() {
        return Err(Error::BadDigit(actual));
    }
    Ok(actual == expected)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_digit_cases() {
        // The barcode formats' own worked examples, so the expected values
        // aren't just whatever this code happens to produce.
        let cases: &[(&str, char)] = &[
            ("03600029145", '2'),  // UPC-A
            ("978030640615", '7'), // ISBN-13 / EAN-13
            ("4012345", '5'),      // EAN-8
            ("0000000", '0'),      // all zeros
            ("9", '3'),            // shortest possible input
        ];
        for (data, want) in cases {
            match check_digit(data) {
                Ok(got) => assert_eq!(got, *want, "check_digit({data:?})"),
                Err(e) => panic!("check_digit({data:?}) failed: {e}"),
            }
        }
    }

    #[test]
    fn check_digit_rejects_bad_input() {
        assert_eq!(check_digit(""), Err(Error::Empty));
        assert_eq!(check_digit("12a4"), Err(Error::BadDigit('a')));
    }

    #[test]
    fn validate_cases() {
        let cases: &[(&str, bool)] = &[
            ("036000291452", true),  // full UPC-A
            ("9780306406157", true), // full ISBN-13 / EAN-13
            ("40123455", true),      // full EAN-8
            ("036000291459", false), // wrong check digit
            ("036000921452", false), // two data digits transposed
            ("00000000", true),      // degenerate but internally consistent EAN-8
        ];
        for (code, want) in cases {
            match validate(code) {
                Ok(got) => assert_eq!(got, *want, "validate({code:?})"),
                Err(e) => panic!("validate({code:?}) failed: {e}"),
            }
        }
    }

    #[test]
    fn validate_rejects_bad_input() {
        assert_eq!(validate(""), Err(Error::Empty));
        assert_eq!(validate("1234567a"), Err(Error::BadDigit('a')));
    }
}
