use crate::cryptopals::*;

const INPUT: &str = "1c0111001f010100061a024b53535009181c";
const KEY: &str = "686974207468652062756c6c277320657965";
const OUTPUT: &str = "746865206b696420646f6e277420706c6179";

pub fn run_and_print() {
    let input = hex::decode(INPUT).unwrap();
    let key = hex::decode(KEY).unwrap();
    let bytes = xor_repeating_key(&input, &key).unwrap();
    let output = hex::encode(&bytes);

    assert_eq!(OUTPUT, output);

    println!("{}", String::from_utf8(bytes).unwrap());
}
