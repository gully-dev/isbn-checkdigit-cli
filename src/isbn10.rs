//! ISBN-10 check digits.
//!
//! The last digit of an ISBN-10 is chosen so that summing digit[i] * (10 - i)
//! for i in 0..10 (digits 1-indexed by position, weights 10 down to 1) is a
//! multiple of 11. Because that sum can require a remainder of 10, the check
//! digit position may hold the letter 'X' instead of a numeral - it never
//! appears anywhere else in a well-formed ISBN-10.

use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    WrongLength(usize),
    BadDigit(char),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::WrongLength(n) => write!(f, "expected 10 characters, got {n}"),
            Error::BadDigit(c) => write!(f, "unexpected character {c:?}"),
        }
    }
}

impl std::error::Error for Error {}

fn digit_value(c: char) -> Option<u32> {
    if c.is_ascii_digit() {
        Some(c as u32 - '0' as u32)
    } else if c == 'X' || c == 'x' {
        Some(10)
    } else {
        None
    }
}

/// Computes the check digit for the first nine digits of an ISBN-10.
/// Returns one of '0'..'9' or 'X'.
pub fn check_digit(first_nine: &str) -> Result<char, Error> {
    let chars: Vec<char> = first_nine.chars().collect();
    if chars.len() != 9 {
        return Err(Error::WrongLength(chars.len()));
    }
    let mut sum = 0u32;
    for (i, c) in chars.iter().enumerate() {
        let d = c.to_digit(10).ok_or(Error::BadDigit(*c))?;
        sum += d * (10 - i as u32);
    }
    let check = (11 - sum % 11) % 11;
    Ok(if check == 10 {
        'X'
    } else {
        char::from_digit(check, 10).unwrap()
    })
}

/// Validates a full 10-character ISBN-10, check digit included.
pub fn validate(isbn: &str) -> Result<bool, Error> {
    let chars: Vec<char> = isbn.chars().collect();
    if chars.len() != 10 {
        return Err(Error::WrongLength(chars.len()));
    }
    let mut sum = 0u32;
    for (i, c) in chars.iter().enumerate() {
        let d = digit_value(*c).ok_or(Error::BadDigit(*c))?;
        // 'X' only ever means "ten", and only in the check digit slot.
        if d == 10 && i != 9 {
            return Err(Error::BadDigit(*c));
        }
        sum += d * (10 - i as u32);
    }
    Ok(sum % 11 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_digit_cases() {
        let cases: &[(&str, char)] = &[
            ("012345678", '9'), // the "0123456789" example that shows up in every ISBN writeup
            ("100000001", 'X'), // sum lands exactly where a remainder of 1 forces an 'X'
            ("000000000", '0'), // all zeros: check digit is also zero
            ("999999999", '9'), // a repeated digit doesn't do anything special to the modulus
        ];
        for (prefix, want) in cases {
            match check_digit(prefix) {
                Ok(got) => assert_eq!(got, *want, "check_digit({prefix:?})"),
                Err(e) => panic!("check_digit({prefix:?}) failed: {e}"),
            }
        }
    }

    #[test]
    fn check_digit_rejects_bad_input() {
        assert_eq!(check_digit("12345678"), Err(Error::WrongLength(8)));
        assert_eq!(check_digit("1234567890"), Err(Error::WrongLength(10)));
        assert_eq!(check_digit("12345678X"), Err(Error::BadDigit('X')));
        assert_eq!(check_digit(""), Err(Error::WrongLength(0)));
    }

    #[test]
    fn validate_cases() {
        let cases: &[(&str, bool)] = &[
            ("0123456789", true),
            ("100000001X", true),
            ("100000001x", true), // lowercase x reads the same as a printed check digit
            ("0213456789", false), // two digits transposed from a valid ISBN
            ("0123456780", false), // wrong check digit, everything else unchanged
            ("000000000X", false), // 'X' only means ten in the check digit slot
        ];
        for (isbn, want) in cases {
            match validate(isbn) {
                Ok(got) => assert_eq!(got, *want, "validate({isbn:?})"),
                Err(e) => panic!("validate({isbn:?}) failed: {e}"),
            }
        }
    }

    #[test]
    fn validate_rejects_bad_input() {
        assert_eq!(validate("012345678"), Err(Error::WrongLength(9)));
        assert_eq!(validate("01234567890"), Err(Error::WrongLength(11)));
        assert_eq!(validate("012345678Y"), Err(Error::BadDigit('Y')));
        assert_eq!(validate("X123456789"), Err(Error::BadDigit('X')));
    }
}
