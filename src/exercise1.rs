const INPUT: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
const OUTPUT: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

pub fn run_and_print() {
    let bytes = hex::decode(&INPUT).unwrap();
    let base64 = base64::encode(&bytes);

    assert_eq!(OUTPUT, base64);
    println!("{}", String::from_utf8(bytes).unwrap());
}
