fn main() {
    let foo = "foo";
    let hex_ = hex::encode(&foo);
    let b64 = base64::encode(&foo);

    println!("{} {} {}", foo, hex_, b64);

    let exercise1 = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let string = String::from_utf8(hex::decode(&exercise1).unwrap()).unwrap();
    let b64 = base64::encode(&string);

    println!("{}", exercise1);
    println!("{}", string);
    println!("{}", b64);
}
