use std::cmp::Ordering;
use std::collections::HashMap;

fn calculate_chi_squared(actual_counts: &HashMap<u8, f64>, expected_counts: &HashMap<u8, f64>) -> Result<f64, String> {
    (0u8..=255u8).map(|byte| {
        let actual = actual_counts.get(&byte).unwrap_or(&0.0);
        let maybe_expected = expected_counts.get(&byte);

        match maybe_expected {
            Some(expected) => Ok(if actual == expected {
                // Specifically, this handles the case of expecting 0 and
                // finding 0, but can short-circuit a little bit of math, too.
                0.0
            } else {
                (actual - expected).powf(2.0) / expected
            }),
            None => Err(format!("Expected hash is missing an entry for {}", byte)),
        }
    }).fold(Ok(0.0), |sum, result| {
        match (sum, result) {
            (Ok(sum), Ok(result)) => Ok(sum + result),
            (Ok(_), Err(result)) => Err(result),
            (Err(err), Ok(_)) => Err(err),
            (Err(err1), Err(err2)) => Err(format!("{}\n{}", err1, err2)),
        }
    })
}

fn count_bytes(message: &[u8]) -> HashMap<u8, usize> {
    let mut counts = HashMap::new();

    for &byte in message {
        counts.insert(byte, counts.get(&byte).unwrap_or(&0) + 1);
    }

    counts
}

fn get_byte_frequency(byte: u8) -> f64 {
    // FIXME: should really sum to 1.0
    match byte {
        b'A' | b'a' => 0.0855,
        b'B' | b'b' => 0.0160,
        b'C' | b'c' => 0.0316,
        b'D' | b'd' => 0.0387,
        b'E' | b'e' => 0.1210,
        b'F' | b'f' => 0.0218,
        b'G' | b'g' => 0.0209,
        b'H' | b'h' => 0.0496,
        b'I' | b'i' => 0.0733,
        b'J' | b'j' => 0.0022,
        b'K' | b'k' => 0.0081,
        b'L' | b'l' => 0.0421,
        b'M' | b'm' => 0.0253,
        b'N' | b'n' => 0.0717,
        b'O' | b'o' => 0.0747,
        b'P' | b'p' => 0.0207,
        b'Q' | b'q' => 0.0010,
        b'R' | b'r' => 0.0633,
        b'S' | b's' => 0.0673,
        b'T' | b't' => 0.0894,
        b'U' | b'u' => 0.0268,
        b'V' | b'v' => 0.0106,
        b'W' | b'w' => 0.0183,
        b'X' | b'x' => 0.0019,
        b'Y' | b'y' => 0.0172,
        b'Z' | b'z' => 0.0011,

        // FIXME: digits?

        // Spaces are much more likely...
        b' ' => 0.1210,

        // ... than other printable characters.
        b'\t' | b'\r' | b'\n' | b'!'..=b'~' => 0.0010,

        // Non-printables shouldn't appear at all.  (Causes a chi-squared of NaN.)
        _ => 0.0,
    }
}

pub fn get_single_byte_key(message: &[u8]) -> Result<u8, String> {
    let expected_counts = (0u8..=255u8)
        .map(|byte| (byte, message.len() as f64 * get_byte_frequency(byte)))
        .collect();
    let mut chi_squareds = (0u8..=255u8)
        .map(|byte| {
            let candidate = xor_single_byte_key(&message, byte);
            let actual_counts = count_bytes(&candidate)
                .iter()
                .map(|(&byte, &count)| (byte, count as f64))
                .collect::<HashMap<u8, f64>>();

            (byte, calculate_chi_squared(&actual_counts, &expected_counts))
        })
        .collect::<Vec<(u8, Result<f64, String>)>>();

    // Fall back to Ordering::Greater if the comparison is None (e.g. involves
    // NaN).
    chi_squareds.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Greater));

    match chi_squareds
        .iter()
        .filter(|(_, result)| result.is_ok())
        .collect::<Vec<&(u8, Result<f64, String>)>>()
        .len() {
        // A lack of candidates is caused by a bad `expected_counts`, caused by
        // problems with get_byte_frequency.
        0 => Err("No candidates found (this is a bug)".to_string()),
        _ => Ok(chi_squareds[0].0),
    }
}

pub fn xor_repeating_key(message: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    if key.len() == 0 {
        Err("Key must not be empty.".to_string())
    } else {
        Ok((0..message.len()).map(|i| message[i] ^ key[i % key.len()]).collect())
    }
}

pub fn xor_single_byte_key(message: &[u8], key: u8) -> Vec<u8> {
    message.iter().map(|byte| byte ^ key).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Modified from the assert_eq! definition.
    macro_rules! assert_partial_eq {
        ($left:expr, $right:expr) => ({
            match (&$left, &$right) {
                (left_val, right_val) => {
                    let maybe_ordering = (*left_val).partial_cmp(&*right_val);

                    if maybe_ordering == None || maybe_ordering != Some(Ordering::Equal) {
                        panic!(r#"assertion failed: `(left == right)`
    left: `{:?}`,
    right: `{:?}`"#, &*left_val, &*right_val)
                    }
                }
            }
        });
    }

    #[test]
    fn calculate_chi_squared_same() {
        let counts = (0u8..=255u8)
            .map(|byte| (byte, byte as f64 + 1.0))
            .collect::<HashMap<u8, f64>>();

        assert_partial_eq!(Ok(0.0), calculate_chi_squared(&counts, &counts));
    }

    #[test]
    fn calculate_chi_squared_missing_expected_key() {
        assert!(calculate_chi_squared(&HashMap::new(), &HashMap::new()).is_err());
    }

    #[test]
    fn calculate_chi_squared_works() {
        let actual_counts = (0u8..=255u8).map(|byte| (byte, match byte {
            b'a' => 1.0,
            b'b' => 2.0,
            b'c' => 1.0,
            _ => 1.0,
        })).collect::<HashMap<u8, f64>>();

        let expected_counts = (0u8..=255u8).map(|byte| (byte, match byte {
            b'a' => 1.0,
            b'b' => 1.0,
            b'c' => 2.0,
            _ => 1.0,
        })).collect::<HashMap<u8, f64>>();

        assert_partial_eq!(Ok(1.5), calculate_chi_squared(&actual_counts, &expected_counts));
    }

    #[test]
    fn count_bytes_empty_message() {
        assert_eq!(HashMap::new(), count_bytes(&Vec::new()));
    }

    #[test]
    fn count_bytes_works() {
        let mut expected = HashMap::new();

        expected.insert(b'a', 2);
        expected.insert(b'b', 1);
        expected.insert(b'c', 1);
        expected.insert(b'd', 1);

        assert_eq!(expected, count_bytes(&vec![b'a', b'b', b'c', b'd', b'a']));
    }

    // get_byte_frequency is a "lookup table" and doesn't need testing?

    // get_single_byte_key will be tested once chi_squared_etaoin_shrdlu is refactored.

    #[test]
    fn xor_repeating_key_empty_message() {
        let empty: Vec<u8> = Vec::new();

        assert_eq!(empty, xor_repeating_key(&empty, &vec![b'a']).unwrap());
    }

    #[test]
    fn xor_repeating_key_equal_length_key() {
        assert_eq!(vec![b'A', b'b', b'C', b'd'], xor_repeating_key(&vec![b' ', b' ', b' ', b' '], &vec![b'a', b'B', b'c', b'D']).unwrap());
    }

    #[test]
    fn xor_repeating_key_short_key() {
        assert_eq!(vec![b'A', b'b', b'A', b'b'], xor_repeating_key(&vec![b' ', b' ', b' ', b' '], &vec![b'a', b'B']).unwrap());
    }

    #[test]
    fn xor_repeating_key_long_key() {
        assert_eq!(vec![b'A', b'b'], xor_repeating_key(&vec![b' ', b' '], &vec![b'a', b'B', b'c', b'D']).unwrap());
    }

    #[test]
    fn xor_repeating_key_bad_key() {
        assert!(xor_repeating_key(&vec![], &vec![]).is_err());
    }

    #[test]
    fn xor_single_byte_key_empty_message() {
        let empty: Vec<u8> = Vec::new();

        assert_eq!(empty, xor_single_byte_key(&empty, b' '));
    }

    #[test]
    fn xor_single_byte_key_works() {
        assert_eq!(vec![b'A', b'b', b'C', b'd'], xor_single_byte_key(&vec![b'a', b'B', b'c', b'D'], b' '));
    }
}