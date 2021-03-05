use image;
use image::io::Reader;
use std::char;
use std::convert::TryInto;
use super::error::ImageError;

pub fn get_morse_code() -> Result<String, ImageError> {
    let img = match Reader::open("PNG.png") {
        Ok(reader) => match reader.decode() {
            Ok(img) => img,
            Err(e) => return Err(ImageError::Image(e))
        },
        Err(e) => return Err(ImageError::Io(e))
    };

    let rgb_image = img.to_rgb8();
    let pixels = rgb_image.pixels();
    let mut prev: u32 = 0;
    let mut index: u32 = 0;
    let mut morse = String::new();
    
    for pixel in pixels {
        // If it's a white pixel.
        if pixel[0] == 255 && pixel[1] == 255 && pixel[2] == 255 {
            let character: u8 = (index - prev).try_into().unwrap();
            let ascii = character as char;
            
            prev = index;
            morse.push(ascii);
        }

        index += 1;
    }

    Ok(morse)
}
