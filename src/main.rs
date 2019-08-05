use std::collections::HashMap;

fn is_text(message: &Vec<u8>) -> bool {
    message.iter().all(|byte| is_text_byte(*byte))
}

fn is_text_byte(byte: u8) -> bool {
    byte < 127 && byte > 31 || byte == 9 || byte == 10 || byte == 13
}

fn most_frequent_char(bytes: &Vec<u8>) -> char {
    let mut counts: HashMap<char, usize> = HashMap::new();

    for character in String::from_utf8(bytes.clone()).unwrap().to_ascii_uppercase().chars() {
        if !counts.contains_key(&character) {
            counts.insert(character, 1);
        } else {
            counts.insert(character, counts.get(&character).unwrap() + 1);
        }
    }

    let mut counts: Vec<(&char, &usize)> = counts.iter().collect();

    counts.sort_by(|(_, value_a), (_, value_b)| value_a.cmp(value_b).reverse());

    *counts[0].0
}

fn xor_equal_key(message: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    assert_eq!(message.len(), key.len());

    (0..message.len()).map(|i| message[i] ^ key[i]).collect()
}

fn xor_single_byte_key(message: &Vec<u8>, key: u8) -> Vec<u8> {
    message.iter().map(|byte| byte ^ key).collect()
}

fn main() {
    let exercise1 = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let string = String::from_utf8(hex::decode(&exercise1).unwrap()).unwrap();
    let b64 = base64::encode(&string);

    assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", b64);

    println!("{}", string);

    let exercise2 = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
    let key = hex::decode("686974207468652062756c6c277320657965").unwrap();
    let xor = xor_equal_key(&exercise2, &key);
    let b64 = hex::encode(&xor);

    assert_eq!("746865206b696420646f6e277420706c6179", b64);

    println!("{}", String::from_utf8(xor).unwrap());

    let exercise3 = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let raw = hex::decode(exercise3).unwrap();
    let potential_keys = (0u8..255u8)
        .filter(|byte| is_text_byte(*byte))
        .filter(|byte| is_text(&xor_single_byte_key(&raw, *byte)))
        .collect::<Vec<u8>>();
    let most_frequent_chars: Vec<(&u8, char)> = potential_keys
        .iter()
        .map(|byte| (byte, most_frequent_char(&xor_single_byte_key(&raw, *byte))))
        .collect();
    let lotta_spaces: Vec<&(&u8, char)> = most_frequent_chars.iter().filter(|(_, character)| *character == ' ').collect();
    let lotta_spaces_key = lotta_spaces[0].0;

    println!("{}", String::from_utf8(xor_single_byte_key(&raw, *lotta_spaces_key)).unwrap());
}
