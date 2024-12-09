use pz1::decipher_ceasar;

// variant 17

const PLAINTEXT: &str = "Узщшч цл очїфжкріьж, жу изткзу \
хпцяі Оічцлвеупт, Ьящьеупт р \
Фчбхзцьеупт шчщчїп. Тчїч ьхрфпіз т \
їчщкз кяґз шзфзфз дзьюжх. Ічкжцз \
уящжіз т ищпоуп юрфеуп \
шщчбчфчкняізфп тчхя фпвл.";

const KEY: usize = 9;

fn main() {
    let plaintext = decipher_ceasar(PLAINTEXT, KEY);
    println!("{plaintext}");
}
