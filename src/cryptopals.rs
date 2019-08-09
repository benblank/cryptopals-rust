use std::collections::HashMap;

fn chi_squared_etaoin_shrdlu(candidate: &[u8]) -> f64 {
    let byte_counts = count_bytes(&candidate);
    let expected_counts = get_expected_counts(candidate.len(), &byte_counts.keys().collect::<Vec<&u8>>());

    byte_counts.iter().map(|(byte, &actual)| {
        let expected = expected_counts.get(&byte).unwrap();

        (actual as f64 - expected).powf(2.0) / expected
    }).sum()
}

fn count_bytes(message: &[u8]) -> HashMap<u8, usize> {
    let mut counts = HashMap::new();

    for &byte in message {
        counts.insert(byte, counts.get(&byte).unwrap_or(&0) + 1);
    }

    counts
}

fn get_expected_counts(message_len: usize, bytes: &[&u8]) -> HashMap<u8, f64> {
    let mut expected_counts = HashMap::new();

    for &&byte in bytes {
        expected_counts.insert(byte, get_letter_frequency(byte) * message_len as f64);
    }

    expected_counts
}

fn get_letter_frequency(byte: u8) -> f64 {
    match byte {
        b'A' | b'a' => 8.55,
        b'B' | b'b' => 1.60,
        b'C' | b'c' => 3.16,
        b'D' | b'd' => 3.87,
        b'E' | b'e' => 12.10,
        b'F' | b'f' => 2.18,
        b'G' | b'g' => 2.09,
        b'H' | b'h' => 4.96,
        b'I' | b'i' => 7.33,
        b'J' | b'j' => 0.22,
        b'K' | b'k' => 0.81,
        b'L' | b'l' => 4.21,
        b'M' | b'm' => 2.53,
        b'N' | b'n' => 7.17,
        b'O' | b'o' => 7.47,
        b'P' | b'p' => 2.07,
        b'Q' | b'q' => 0.10,
        b'R' | b'r' => 6.33,
        b'S' | b's' => 6.73,
        b'T' | b't' => 8.94,
        b'U' | b'u' => 2.68,
        b'V' | b'v' => 1.06,
        b'W' | b'w' => 1.83,
        b'X' | b'x' => 0.19,
        b'Y' | b'y' => 1.72,
        b'Z' | b'z' => 0.11,

        // HACK: Without a frequency for spaces, chi-squared can't tell the
        // difference between uppercase and lowercase.
        // b' ' => 12.10,

        _ => 0.0001,
    }
}

pub fn get_single_byte_key(message: &[u8]) -> u8 {
    let mut chi_squareds = (0u8..255u8)
        .map(|byte| (byte, chi_squared_etaoin_shrdlu(&xor_single_byte_key(message, byte))))
        .collect::<Vec<(u8, f64)>>();

    chi_squareds.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

    chi_squareds[0].0
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

    // chi_squared_etaoin_shrdlu will be refactored into something easier to test.

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

    // get_expected_counts will be refactored into something easier to test.

    // get_letter_frequency is a "lookup table" and doesn't need testing?

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
        match xor_repeating_key(&vec![], &vec![]) {
            Ok(_) => panic!("Expected an empty key to result in an error."),
            Err(_) => (),
        }
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
