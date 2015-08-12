use std::fmt;
use std::str;

const ISBN_13_MULTIPLIERS: [u32; 13] = [1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1];

#[derive(Debug)]
pub struct Isbn {
    raw: String,
    values: Vec<u32>
}

#[derive(Debug)]
pub struct IsbnParseError {
    raw: String,
    error_type: IsbnParseErrorType,
}

#[derive(Debug)]
pub enum IsbnParseErrorType {
    FailedCheck10,
    FailedCheck13,
    WrongLength,
}

impl IsbnParseError {
    fn from_raw<T: Into<String>>(error_type: IsbnParseErrorType, s: T) -> IsbnParseError {
        IsbnParseError {
            raw: s.into(),
            error_type: error_type,
        }
    }
}

impl str::FromStr for Isbn {
    type Err = IsbnParseError;

    fn from_str(s: &str) -> Result<Isbn, Self::Err> {
        let values: Vec<_> = s.chars().filter_map(|c| as_value(c)).collect();
        match values.len() {
            10 => if check_10(&values) {
                Ok(Isbn { raw: s.to_owned(), values: values })
            } else {
                Err(IsbnParseError::from_raw(IsbnParseErrorType::FailedCheck10, s))
            },
            13 => if check_13(&values) {
                Ok(Isbn { raw: s.to_owned(), values: values })
            } else {
                Err(IsbnParseError::from_raw(IsbnParseErrorType::FailedCheck13, s))
            },
            _ => Err(IsbnParseError::from_raw(IsbnParseErrorType::WrongLength, s)),
        }
    }
}

impl fmt::Display for Isbn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.raw)
    }
}


fn check_10(s: &[u32]) -> bool {
    s.iter().cloned().enumerate().map(|(idx, n)| n * (10 - idx as u32)).sum::<u32>() % 11 == 0
}

fn check_13(s: &[u32]) -> bool {
    s.iter().zip(ISBN_13_MULTIPLIERS.iter()).map(|(a, b)| a * b).sum::<u32>() % 10 == 0
}

fn as_value(c: char) -> Option<u32> {
    match c {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::Isbn;

    #[test]
    fn isnb_10_good() {
        "99921-58-10-7".parse::<Isbn>().unwrap();
    }

    #[test]
    #[should_panic]
    fn isbn_10_bad() {
        "99921-58-10-8".parse::<Isbn>().unwrap();
    }

    #[test]
    fn isbn_13_good() {
        "978-0-306-40615-7".parse::<Isbn>().unwrap();
    }

    #[test]
    #[should_panic]
    fn isbn_13_bad() {
        "978-0-306-40615-8".parse::<Isbn>().unwrap();
    }
}
