fn bytes_to_hex(bytes: &Vec<u8>) -> String {
    bytes
        .iter()
        .map(|byte| format!("{:x}", byte))
        .collect::<Vec<_>>()
        .concat()
}

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    // https://stackoverflow.com/a/52992629/46387
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

fn main() {
    let string1 = String::from("foo");
    let bytes1 = string1.as_bytes().to_vec();
    let hex = bytes_to_hex(&bytes1);
    let bytes2 = hex_to_bytes(&hex);

    println!("{} {:?} {} {:?}", string1, bytes1, hex, bytes2);
}
