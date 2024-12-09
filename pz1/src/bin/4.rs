use pz1::{decipher_vigenere_digits, dedigitalize, digitalize};

// variant 17
// 17 % 10 == 7

const CIPHERTEXT: &str = "62253825593953633817470164591";

const KEY: &str = "одо";

fn main() {
    let dig_key = digitalize(KEY);
    let dig_plaintext = decipher_vigenere_digits(CIPHERTEXT, &dig_key);
    let plaintext = dedigitalize(&dig_plaintext);
    println!("{plaintext}");
}
