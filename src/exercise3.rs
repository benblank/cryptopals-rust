use crate::cryptopals::*;

const INPUT: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

pub fn run_and_print() {
    let bytes = hex::decode(INPUT).unwrap();
    let key = get_single_byte_key(&bytes).unwrap();

    assert_eq!(key, b'X');  // Panic rather than printing a possibly-invalid string.
    println!("{}", String::from_utf8(xor_single_byte_key(&bytes, key)).unwrap());
}
