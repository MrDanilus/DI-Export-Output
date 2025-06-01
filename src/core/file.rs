use std::path::PathBuf;

use image::ImageReader;

pub fn check_image(path: &PathBuf) -> bool{
    return match ImageReader::open(path){
        Ok(img) => match img.with_guessed_format(){
            Ok(img) => {
                if img.format().is_none(){
                    false
                } else {
                    true
                }
            },
            Err(_) => false
        },
        Err(_) => false
    }
}