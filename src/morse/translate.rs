use super::error::TranslationError;
use super::error::UnknownCharacterError;
use std::char;

pub fn morse_to_ascii(morse: &String) -> Result<String, TranslationError> {
    let characters = morse.trim().split(" ");
    let mut ascii_text = String::new();

    for character in characters {
        let ascii_char = match morse_character_to_ascii(character) {
            Ok(ascii) => ascii,
            Err(e) => return Err(e)
        };

        ascii_text.push(ascii_char);
    }

    Ok(ascii_text)
}

fn morse_character_to_ascii(character: &str) -> Result<char, TranslationError> {
    match character {
        ".-"    => Ok('A'),
        "-..."  => Ok('B'),
        "-.-."  => Ok('C'),
        "-.."   => Ok('D'),
        "."     => Ok('E'),
        "..-."  => Ok('F'),
        "--."   => Ok('G'),
        "...."  => Ok('H'),
        ".."    => Ok('I'),
        ".---"  => Ok('J'),
        "-.-"   => Ok('K'),
        ".-.."  => Ok('L'),
        "--"    => Ok('M'),
        "-."    => Ok('N'),
        "---"   => Ok('O'),
        ".--."  => Ok('P'),
        "--.-"  => Ok('Q'),
        ".-."   => Ok('R'),
        "..."   => Ok('S'),
        "-"     => Ok('T'),
        "..-"   => Ok('U'),
        "...-"  => Ok('V'),
        ".--"   => Ok('W'),
        "-..-"  => Ok('X'),
        "-.--"  => Ok('Y'),
        "--.."  => Ok('Z'),
        ".----" => Ok('1'),
        "..---" => Ok('2'),
        "...--" => Ok('3'),
        "....-" => Ok('4'),
        "....." => Ok('5'),
        "-...." => Ok('6'),
        "--..." => Ok('7'),
        "---.." => Ok('8'),
        "----." => Ok('9'),
        "-----" => Ok('0'),
        _ => Err(TranslationError::UnknownCharacter(
            UnknownCharacterError {
                character: String::from(character)
            }
        ))
    }
}