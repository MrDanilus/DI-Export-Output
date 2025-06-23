use std::{fs, path::PathBuf};

use crate::core::parse::clean_text_bytes;

pub fn parse(format: &str, path: &PathBuf) -> Result<String, String>{
    let split = match format{
        "jpg" | "jpeg" => ("UNICODE", ""),
        "avif" => ("UNICODE", ""),
        "png" => ("parameters", "IDAT"),
        _ => ("UNICODE", "NO SECOND SPLIT CHARACTER !#@")
    };

    let file = fs::read(path).map_err(|e| format!("Ошибка чтения файла: {e}"))?;

    let file_str = String::from_utf8_lossy(&file);
    let raw_params = file_str.split(split.0).nth(1)
        .ok_or(format!("Не удалось найти данные"))?;
    let raw_params = raw_params.split(split.1).nth(0)
        .ok_or(format!("Не удалось найти данные"))?;
    let params = String::from_utf8(clean_text_bytes(raw_params.as_bytes()))
        .map_err(|e| format!("Ошибка чистки байтов: {e}"))?;

    Ok(params)
}