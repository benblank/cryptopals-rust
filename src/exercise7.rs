use crate::rijndael::decrypt_block;
use std::fs;

const KEY: &[u8] = b"YELLOW SUBMARINE";

pub fn run_and_print() {
    let message = base64::decode(&fs::read_to_string("7.txt").unwrap().replace("\n", "")).unwrap();
    let decrypted = message
        .chunks_exact(16)
        .map(|chunk| decrypt_block(&chunk, KEY).unwrap())
        .collect::<Vec<Vec<u8>>>()
        .concat();

    println!("{}", String::from_utf8(decrypted).unwrap());
}
