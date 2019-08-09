mod cryptopals;
mod exercise1;
mod exercise2;

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};

fn break_repeating_xor(message: &Vec<u8>) -> String {
    let key_length = guess_key_length(&message);
    let stripes = chunk_and_transpose(&message, key_length);
    let key = stripes
        .iter()
        .map(|stripe| get_single_byte_key(stripe).unwrap())
        .collect::<Vec<u8>>();

    String::from_utf8(xor_repeating_key(&message, &key)).unwrap()
}

fn break_single_byte_xor(encoded: &String) -> Option<String> {
    let raw = hex::decode(encoded).unwrap();

    get_single_byte_key(&raw).map(|key| String::from_utf8(xor_single_byte_key(&raw, key)).unwrap())
}

fn chi_squared_etaoin_shrdlu(candidate: &Vec<u8>) -> f64 {
    let byte_counts = count_bytes(&candidate);
    let expected_counts = get_expected_counts(candidate.len(), &byte_counts.keys().collect());

    byte_counts.iter().map(|(byte, &actual)| {
        let expected = expected_counts.get(&byte).unwrap();

        (actual as f64 - expected).powf(2.0) / expected
    }).sum()
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

fn count_bytes(message: &Vec<u8>) -> HashMap<u8, usize> {
    let mut counts = HashMap::new();

    for &byte in message {
        counts.insert(byte, counts.get(&byte).unwrap_or(&0) + 1);
    }

    counts
}

fn get_expected_counts(message_len: usize, bytes: &Vec<&u8>) -> HashMap<u8, f64> {
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

fn get_single_byte_key(message: &Vec<u8>) -> Option<u8> {
    let mut chi_squareds = (0u8..255u8)
        .map(|byte| (byte, chi_squared_etaoin_shrdlu(&xor_single_byte_key(&message, byte))))
        .collect::<Vec<(u8, f64)>>();

    chi_squareds.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

    Some(chi_squareds[0].0)
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
    let difference = xor_repeating_key(left, right);

    difference.iter().map(|byte| byte.count_ones()).sum()
}

fn xor_repeating_key(message: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    (0..message.len()).map(|i| message[i] ^ key[i % key.len()]).collect()
}

fn xor_single_byte_key(message: &Vec<u8>, key: u8) -> Vec<u8> {
    message.iter().map(|byte| byte ^ key).collect()
}

fn main() {
    exercise1::run_and_print();
    exercise2::run_and_print();

    let exercise3 = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let maybe_broken = break_single_byte_xor(&String::from(exercise3));

    match maybe_broken {
        Some(broken) => println!("{}", broken),
        None => panic!("No match found"),
    }

    // let exercise4_file = File::open("4.txt").unwrap();

    // for line in BufReader::new(exercise4_file).lines().map(|line| line.unwrap()) {
    //     if let Some(broken) = break_single_byte_xor(&String::from(line)) {
    //         println!("{}", broken);
    //     }
    // }

    let exercise5 = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let encoded = String::from(exercise5).into_bytes();
    let encrypted = xor_repeating_key(&encoded, &String::from("ICE").into_bytes());
    let encoded = hex::encode(encrypted);

    assert_eq!(encoded, String::from("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"));

    println!("{}", exercise5);

    assert_eq!(hamming_distance(&String::from("this is a test").into_bytes(), &String::from("wokka wokka!!!").into_bytes()), 37);

    let exercise6 = base64::decode(&fs::read_to_string("6.txt").unwrap().replace("\n", "")).unwrap();
    let decrypted = break_repeating_xor(&exercise6);

    println!("{}", decrypted);
}
