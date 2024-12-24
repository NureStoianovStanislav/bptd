use std::collections::HashMap;

const ALPHABET: &str = "АБВГҐДЕЄЖЗИІЇЙКЛМНОПРСТУФХЦЧШЩЬЮЯ";

pub fn decipher_ceasar(ciphertext: &str, key: usize) -> String {
    let alphabet_size = ALPHABET.chars().count();
    let decoding_key = alphabet_size - key;
    shift_ceasar(ciphertext, decoding_key)
}

pub fn decipher_vigenere(ciphertext: &str, key: &str) -> String {
    let alphabet_size = ALPHABET.chars().count();
    shift_vigenere(ciphertext, gen_keystream(key).map(|k| alphabet_size - k))
}

pub fn extract_key_vigenere(ciphertext: &str, plaintext_part: &str) -> String {
    let alphabet_size = ALPHABET.chars().count();
    let keystream = gen_keystream(plaintext_part).map(|k| alphabet_size - k);
    let repeated_key = shift_vigenere(ciphertext, keystream)
        .chars()
        .take(plaintext_part.chars().count())
        .filter(|l| ALPHABET.contains(&l.to_uppercase().to_string()))
        .collect::<String>();
    (1..=repeated_key.chars().count())
        .map(|n| {
            let mut chars = repeated_key.chars().flat_map(char::to_lowercase);
            let key = chars.by_ref().take(n).collect::<String>();
            let rest = chars.collect::<String>();
            (key, rest)
        })
        .find(|(key, rest)| key.chars().cycle().zip(rest.chars()).all(|(a, b)| a == b))
        .map(|(key, _)| key)
        .unwrap()
}

pub fn shift_ceasar(text: &str, key: usize) -> String {
    text.chars()
        .map(|l| {
            if ALPHABET.contains(&l.to_uppercase().to_string()) {
                shift_letter(l, key)
            } else {
                l
            }
        })
        .collect()
}

fn gen_keystream(key: &str) -> impl Iterator<Item = usize> + '_ {
    key.chars()
        .cycle()
        .flat_map(|k| k.to_uppercase())
        .flat_map(|k| ALPHABET.chars().position(|x| x == k))
}

fn shift_vigenere(text: &str, mut keystream: impl Iterator<Item = usize>) -> String {
    text.chars()
        .map(|l| {
            if ALPHABET.contains(&l.to_uppercase().to_string()) {
                shift_letter(l, keystream.next().unwrap())
            } else {
                l
            }
        })
        .collect()
}

fn shift_letter(letter: char, offset: usize) -> char {
    letter
        .to_uppercase()
        .next()
        .and_then(|l| ALPHABET.chars().position(|x| x == l))
        .map(|i| (i + offset) % ALPHABET.chars().count())
        .and_then(|i| ALPHABET.chars().nth(i))
        .and_then(|l| {
            if letter.is_uppercase() {
                Some(l)
            } else {
                l.to_lowercase().next()
            }
        })
        .unwrap()
}

pub fn digitalize(text: &str) -> String {
    let cb = checkerboard().collect::<HashMap<_, _>>();
    text.to_uppercase()
        .chars()
        .flat_map(|letter| cb.get(&letter).unwrap().chars())
        .collect()
}

pub fn dedigitalize(text: &str) -> String {
    let cb = checkerboard()
        .map(|(letter, code)| (code, letter))
        .collect::<HashMap<_, _>>();
    let mut text = text.chars();
    core::iter::from_fn(|| {
        text.next().and_then(|prefix| match prefix {
            prefix @ '1'..='7' => Some(prefix.to_string()),
            prefix => text.next().map(|suffix| [prefix, suffix].iter().collect()),
        })
    })
    .map(|code| cb.get(&code).unwrap())
    .collect()
}

pub fn decipher_vigenere_digits(ciphertext: &str, key: &str) -> String {
    let keystream = key
        .chars()
        .map(|key| key.to_digit(10).unwrap())
        .map(|encoding_key| 10 - encoding_key)
        .cycle();
    ciphertext
        .chars()
        .map(|digit| digit.to_digit(10).unwrap())
        .zip(keystream)
        .map(|(digit, key)| (digit + key) % 10)
        .map(|digit| digit.to_string().chars().next().unwrap())
        .collect()
}

fn checkerboard() -> impl Iterator<Item = (char, String)> {
    [
        (None, vec!['А', 'И', 'Т', 'Е', 'С', 'Н', 'О']),
        (
            Some('8'),
            vec!['Б', 'В', 'Г', 'Ґ', 'Д', 'Є', 'Ж', 'З', 'І', 'Ї'],
        ),
        (
            Some('9'),
            vec!['Й', 'К', 'Л', 'М', 'П', 'Р', 'У', 'Ф', 'Х', 'Ц'],
        ),
        (Some('0'), vec!['Ч', 'Ш', 'Щ', 'Ь', 'Ю', 'Я', ' ']),
    ]
    .into_iter()
    .flat_map(|(row, letters)| {
        letters
            .into_iter()
            .enumerate()
            .map(|(i, l)| ((i + 1) % 10, l))
            .map(move |(col, l)| {
                (
                    l,
                    row.iter()
                        .copied()
                        .chain(col.to_string().chars())
                        .collect::<String>(),
                )
            })
    })
}
