fn xor(message: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    assert_eq!(message.len(), key.len());

    (0..message.len()).map(|i| message[i] ^ key[i]).collect()
}

fn main() {
    let exercise1 = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let string = String::from_utf8(hex::decode(&exercise1).unwrap()).unwrap();
    let b64 = base64::encode(&string);

    assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", b64);

    println!("{}", string);

    let exercise2 = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
    let key = hex::decode("686974207468652062756c6c277320657965").unwrap();
    let xor_ = xor(&exercise2, &key);
    let b64 = hex::encode(&xor_);

    assert_eq!("746865206b696420646f6e277420706c6179", b64);

    println!("{}", String::from_utf8(xor_).unwrap());
}
