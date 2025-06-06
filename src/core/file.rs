use std::path::PathBuf;
use freya::{events::FileData, prelude::{Event, Readable, Signal, Writable}};
use image::ImageReader;
use walkdir::WalkDir;

use crate::ui::app::DnDStatus;

pub fn check_file(files: Signal<Vec<PathBuf>>, file: Event<FileData>) -> DnDStatus{
    let path = match &file.file_path{
        Some(path) => path,
        None => return DnDStatus::Wrong
    };
    if path.is_dir(){
        let mut status = DnDStatus::Wrong;
        for entry in WalkDir::new(path)
        .into_iter().filter_map(|e| e.ok()){
            let path = entry.path().to_path_buf();
            match check_image(&path){
                true => {
                    if files.read().contains(&path){
                        status = DnDStatus::Exists;
                    } else{
                        status = DnDStatus::Ok;
                        break;
                    }
                },
                false => continue
            }
        }

        return status
    } else{
        if files.read().contains(path){
            return DnDStatus::Exists;
        }
        match check_image(path){
            true => DnDStatus::Ok,
            false => DnDStatus::Wrong
        }
    }
}

fn check_image(path: &PathBuf) -> bool{
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

pub fn write(
    files: &mut Signal<Vec<PathBuf>>, file_hover: &mut Signal<DnDStatus>,
    file: Event<FileData>
) {
    let file_clone = file.clone();
    let path = match &file_clone.file_path{
        Some(path) => path,
        None => {
            file_hover.set(DnDStatus::Wrong);
            return;
        }
    };
    match check_file(*files, file){
        DnDStatus::Ok => {
            file_hover.set(DnDStatus::None);
            if path.is_dir(){
                for entry in WalkDir::new(path)
                .into_iter().filter_map(|e| e.ok()){
                    let path = entry.path().to_path_buf();
                    match check_image(&path){
                        true => {
                            if !files.read().contains(&path){
                                files.write().push(path.clone());
                            }
                        },
                        false => continue
                    }
                }
            } else{
                files.write().push(path.clone());
            }
        },
        DnDStatus::Exists => file_hover.set(DnDStatus::Exists),
        status => file_hover.set(status)
    }
}