fn bytes_to_hex(bytes: Vec<u8>) -> String {
    let mut hex = Vec::new();

    for byte in bytes.iter() {
        hex.push(format!("{:x}", byte));
    }

    hex.concat()
}

fn main() {
    let string = String::from("foo");
    let bytes = string.as_bytes().to_vec();
    let hex = bytes_to_hex(bytes);

    println!("{}", hex);
}
