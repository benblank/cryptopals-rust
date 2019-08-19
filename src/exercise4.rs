use crate::cryptopals::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run_and_print() {
    let file = File::open("4.txt").unwrap();
    let lines = BufReader::new(file)
        .lines()
        .map(|line| hex::decode(line.unwrap()).unwrap())
        .collect::<Vec<Vec<u8>>>();
    let expected_counts = (0u8..=255u8)
        .map(|byte| (byte, lines[0].len() as f64 * get_byte_frequency(byte)))
        .collect();

    let mut messages = lines
        .iter()
        .map(|bytes| {
            let key = get_single_byte_key(&bytes).unwrap();
            let message = xor_single_byte_key(&bytes, key);
            let actual_counts = count_bytes(&message)
                .iter()
                .map(|(&byte, &count)| (byte, count as f64))
                .collect();
            let chi_squared = calculate_chi_squared(&actual_counts, &expected_counts).unwrap();

            (message, chi_squared)
        })
        .collect::<Vec<(Vec<u8>, f64)>>();

    messages.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

    println!("{}", String::from_utf8(messages[0].0.clone()).unwrap());
}
