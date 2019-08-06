use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};

fn break_repeating_xor(message: &Vec<u8>) -> Option<String> {
    let key_length = guess_key_length(&message);
    let stripes = chunk_and_transpose(&message, key_length);
    let key = stripes
        .iter()
        .map(|stripe| get_single_byte_key(stripe).unwrap_or(32))
        .collect::<Vec<u8>>();

    Some(String::from_utf8(xor_repeating_key(&message, &key)).unwrap())
}

fn break_single_byte_xor(encoded: &String) -> Option<String> {
    let raw = hex::decode(encoded).unwrap();

    get_single_byte_key(&raw).map(|key| String::from_utf8(xor_single_byte_key(&raw, key)).unwrap())
}

fn chunk_and_transpose(message: &Vec<u8>, key_length: usize) -> Vec<Vec<u8>> {
    let mut result = vec![Vec::new(); key_length];

    for chunk in message.chunks(key_length) {
        for (i, &byte) in chunk.iter().enumerate() {
            result[i].push(byte);
        }
    }

    result
}

fn get_single_byte_key(message: &Vec<u8>) -> Option<u8> {
    let potential_keys = (0u8..255u8)
        .filter(|byte| is_text_byte(*byte))
        .filter(|byte| is_text(&xor_single_byte_key(&message, *byte)))
        .collect::<Vec<u8>>();
    let most_frequent_chars: Vec<(&u8, char)> = potential_keys
        .iter()
        .map(|byte| (byte, most_frequent_char(&xor_single_byte_key(&message, *byte))))
        .collect();
    let lotta_spaces: Vec<&(&u8, char)> = most_frequent_chars.iter().filter(|(_, character)| *character == ' ').collect();

    if lotta_spaces.len() > 0 {
        Some(*lotta_spaces[0].0)
    } else {
        None
    }
}

fn guess_key_length(message: &Vec<u8>) -> usize {
    let mut distances: HashMap<usize, f64> = HashMap::new();

    // Sequences shorter than ten characters are unlikely to be useful.
    for size in 2..(message.len() / 10) {
        distances.insert(size, hamming_distance(&message[..size].to_vec(), &message[size..(2 * size)].to_vec()) as f64 / size as f64);
    }

    let mut counts: Vec<(&usize, &f64)> = distances.iter().collect();

    counts.sort_by(|(_, value_a), (_, value_b)| value_a.partial_cmp(value_b).unwrap());

    *counts[0].0
}

fn hamming_distance(left: &Vec<u8>, right: &Vec<u8>) -> u32 {
    let difference = xor_equal_key(left, right);

    difference.iter().map(|byte| byte.count_ones()).sum()
}

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

fn xor_repeating_key(message: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    (0..message.len()).map(|i| message[i] ^ key[i % key.len()]).collect()
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
    let maybe_broken = break_single_byte_xor(&String::from(exercise3));

    match maybe_broken {
        Some(broken) => println!("{}", broken),
        None => panic!("No match found"),
    }

    let exercise4_file = File::open("4.txt").unwrap();

    for line in BufReader::new(exercise4_file).lines().map(|line| line.unwrap()) {
        if let Some(broken) = break_single_byte_xor(&String::from(line)) {
            println!("{}", broken);
        }
    }

    let exercise5 = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let encoded = String::from(exercise5).into_bytes();
    let encrypted = xor_repeating_key(&encoded, &String::from("ICE").into_bytes());
    let encoded = hex::encode(encrypted);

    assert_eq!(encoded, String::from("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"));

    println!("{}", exercise5);

    assert_eq!(hamming_distance(&String::from("this is a test").into_bytes(), &String::from("wokka wokka!!!").into_bytes()), 37);

    let exercise6 = base64::decode(&fs::read_to_string("6.txt").unwrap().replace("\n", "")).unwrap();
    let decrypted = break_repeating_xor(&exercise6).unwrap();

    println!("{}", decrypted);
}
