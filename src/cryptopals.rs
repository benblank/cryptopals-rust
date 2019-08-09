pub fn xor_repeating_key(message: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    if key.len() == 0 {
        Err("Key must not be empty.".to_string())
    } else {
        Ok((0..message.len()).map(|i| message[i] ^ key[i % key.len()]).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
