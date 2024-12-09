use pz1::{decipher_vigenere, extract_key_vigenere};

// variant 17
// 17 % 7 == 3

const CIPHERTEXT: &str = "Ґчзйюд ґжє еєзгншвдс іящь зкфґя чсчжґтрзс Оца Йсдщж 1984 згвї. Ряе \
ґжяґіиєюф, ке сятя ч'рфшгтер дгщґшуаеип рюггжяеиєріфоия нв \
фщцгдюішб гапм щґ'с отє ґенєефж счдчиж Сґщзя, єе лф єєчттрюґг т \
иягтудї ґжєіччжжї оуіфдішїаяслща ужцп-нвєсе ифдиж. Цєрфяю лте ячфр \
Ноґадс юогяйсґозс аюнф взозярер яжяґиєсжтзямвер сеґгуєаедяер, огш р \
2000 жєія, юоучнвя гцещб фщцєбяв ржтчгяреиєя ф фгаґияїдаю \
взшєігфзоїаь, ячфп фусґгзс риааюіш у ьшиін.";

const PLAINTEXT_PART: &str = "Першим про спрощення";

fn main() {
    let key = extract_key_vigenere(CIPHERTEXT, PLAINTEXT_PART);
    println!("{key}");
    let plaintext = decipher_vigenere(CIPHERTEXT, &key);
    println!("{plaintext}");
}
