use pz1::shift_ceasar;

// результат 4го завдання: МОЄ ЗАВДАННЯ ПЕРШЕ

const NAME: &str = "Стоянов Станіслав Юрійович";

const BIRTHDAY: &str = "тринадцяте грудня";

const GROUP: usize = 1;

fn main() {
    let encoded_name = shift_ceasar(NAME, GROUP);
    println!("{encoded_name}");
    let encoded_birthday = shift_ceasar(BIRTHDAY, GROUP);
    println!("{encoded_birthday}");
}
