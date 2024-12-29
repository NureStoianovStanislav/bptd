mod des;
mod entropy;
mod feistel;
mod keys;
mod permutation;

use entropy::calculate_entropy;

use base64::prelude::*;

fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next().expect("please enter a command to execute");
    match command.as_str() {
        "keygen" => {
            let seed_key = args
                .next()
                .expect("please provide your arbitrary-length key");
            let key = keys::seed_key(&seed_key);
            println!("64-bit key: {key:064b}");
        }
        "encrypt" => {
            let plaintext = args.next().expect("please enter a message to encrypt");
            let entropy_before = calculate_entropy(plaintext.as_bytes());
            let key = expect_valid_key(args.next());
            let ciphertext = des::encrypt(&plaintext, key);
            let entropy_after = calculate_entropy(&ciphertext);
            let base64_encoded = BASE64_STANDARD.encode(ciphertext);
            println!("encrypted message: {base64_encoded}");
            println!("entropy before encryption: {entropy_before}");
            println!("entropy after encryption: {entropy_after}");
        }
        "decrypt" => {
            let base64_encoded = args.next().expect("please enter a message to decrypt");
            let ciphertext = BASE64_STANDARD
                .decode(base64_encoded.as_bytes())
                .expect("expected ciphertext to be base64 encoded");
            let entropy_before = calculate_entropy(&ciphertext);
            let key = expect_valid_key(args.next());
            let plaintext_bytes = match des::decrypt(&ciphertext, key) {
                Ok(p) => p,
                Err(err) => return eprintln!("failed to decrypt message: {err:?}"),
            };
            let entropy_after = calculate_entropy(&plaintext_bytes);
            let plaintext = String::from_utf8_lossy(&plaintext_bytes).to_string();
            println!("decrypted message: {plaintext}");
            println!("entropy before decryption: {entropy_before}");
            println!("entropy after decryption: {entropy_after}");
        }
        cmd => panic!("unknown command: {cmd}"),
    }
}

fn expect_valid_key(arg: Option<String>) -> u64 {
    let key = arg.and_then(|arg| u64::from_str_radix(&arg, 2).ok())
                .expect("please provide key formatted as a 64 bit binary number, use keygen command to generate one");
    match keys::check_key_regression(key) {
        Ok(key) => key,
        Err(()) => panic!("found key regression: not every key byte contains odd number of bits"),
    }
}
