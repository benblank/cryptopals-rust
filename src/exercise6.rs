use crate::cryptopals::*;
use std::fs;

pub fn run_and_print() {
    let message = base64::decode(&fs::read_to_string("6.txt").unwrap().replace("\n", "")).unwrap();
    let key_length = guess_key_length(&message);
    let stripes = chunk_and_transpose(&message, key_length).unwrap();
    let key = stripes
        .iter()
        .map(|stripe| get_single_byte_key(stripe).unwrap())
        .collect::<Vec<u8>>();

    let decrypted = String::from_utf8(xor_repeating_key(&message, &key).unwrap()).unwrap();

    println!("{}", decrypted);
}
