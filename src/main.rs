mod morse;

use morse::image::get_morse_code;
use morse::translate::morse_to_ascii;

fn main() {
    let morse_code = &get_morse_code();
    let morse_code = match morse_code {
        Ok(code) => code,
        Err(e) => panic!("{}", e)
    };

    let plain_text = &morse_to_ascii(&morse_code);
    let plain_text = match plain_text {
        Ok(text) => text,
        Err(e) => panic!("{}", e)
    };

    println!("");
    println!("Morse Code: {}", morse_code);
    println!("");
    println!("Plain Text: {}", plain_text);
}
