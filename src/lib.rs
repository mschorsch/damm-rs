//! Damm algorithm.
//!
//! _"In error detection, the Damm algorithm is a check digit algorithm that
//! detects all single-digit errors and all adjacent transposition errors."_ (Wikipedia)
//!
//! https://en.wikipedia.org/wiki/Damm_algorithm
const MATRIX: [[usize; 10]; 10] = [[0, 3, 1, 7, 5, 9, 8, 6, 4, 2],
                                   [7, 0, 9, 2, 1, 5, 4, 8, 6, 3],
                                   [4, 2, 0, 6, 8, 7, 1, 3, 5, 9],
                                   [1, 7, 5, 0, 9, 8, 3, 4, 2, 6],
                                   [6, 1, 2, 3, 0, 4, 5, 9, 7, 8],
                                   [3, 6, 7, 4, 2, 0, 9, 5, 8, 1],
                                   [5, 8, 6, 9, 7, 2, 0, 1, 3, 4],
                                   [8, 9, 4, 5, 3, 6, 2, 0, 1, 7],
                                   [9, 4, 3, 8, 6, 1, 7, 2, 0, 5],
                                   [2, 5, 8, 1, 4, 3, 6, 7, 9, 0]];

/// Calculates the checksum.
///
/// # Examples
///
/// ```
/// use damm_rs::encode;
///
/// let number = "572";
/// assert_eq!(encode(number), Some(4));
/// ```
pub fn encode(number: &str) -> Option<usize> {
    let mut interim = 0;
    for c in number.chars() {
        match c.to_digit(10) {
            Some(_) => {
                // UTF-8
                // '0' -> 30 (hex) -> 48 (decimal)
                // '1' -> 31 (hex) -> 49 (decimal)
                // ...
                let index = c as usize - 48;
                interim = MATRIX[interim][index];
            }
            _ => return None,
        }
    }

    Some(interim)
}

/// Calculates and adds the checkum to the provided string.
///
/// # Examples
///
/// ```
/// use damm_rs::check_sum;
///
/// let number = "572";
/// assert_eq!(check_sum(number), Some(String::from("5724")));
/// ```
pub fn check_sum(number: &str) -> Option<String> {
    encode(number).map(|digit| format!("{}{}", number, digit))
}

/// Validates the checksum of the string.
///
/// # Examples
///
/// ```
/// use damm_rs::is_valid;
///
/// let number = "5724";
/// assert!(is_valid(number));
/// ```
pub fn is_valid(number: &str) -> bool {
    encode(number) == Some(0)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(encode("43881234567"), Some(9));
        assert_eq!(encode("572"), Some(4));
    }

    #[test]
    fn test_check_sum() {
        assert_eq!(check_sum("43881234567"), Some("438812345679".to_string()));
        assert_eq!(check_sum("572"), Some("5724".to_string()));
    }

    #[test]
    fn test_is_valid() {
        assert!(is_valid("438812345679"));
        assert!(is_valid("5724"));
    }
}
