use std::path::PathBuf;
use freya::prelude::*;

use crate::{
    core::file::{check_file, write}, 
    ui::components::{
        exif_view::exif_view, file_list::file_list, header::header, preview::preview
    }
};

mod components;
mod theme;

pub enum DnDStatus{
    Ok,
    Wrong,
    Exists,
    None
}
#[derive(Clone)]
pub enum Exif{
    Ok(String),
    Err(String),
    None
}

pub fn view() -> Element{
    let mut file_hover = use_signal(|| DnDStatus::None);
    let mut files: Signal<Vec<PathBuf>> = use_signal(|| Vec::new());

    let selected_file = use_signal(|| None);
    let civitai_request = use_signal(|| false);
    let metadata = use_signal(|| Exif::None);

    rsx!(rect{
        direction: "vertical",
        height: "fill",
        width: "fill",

        background: "black",

        padding: "8",

        onglobalfilehover: move |file| 
            file_hover.set(check_file(files, file)),
        onglobalfilehovercancelled: move |_| 
            file_hover.set(DnDStatus::None),
        onfiledrop: move |file| 
            write(&mut files, &mut file_hover, file),

        rect { 
            direction: "horizontal",
            height: "65%",
            width: "fill",

            rect {
                direction: "vertical",
                height: "fill",
                width: "50%",
                main_align: "center",
                cross_align: "center",
                
                {header(civitai_request, selected_file, metadata, files)},
                
                {file_list(selected_file, metadata, files, civitai_request, file_hover)}
            },
            rect {
                border: "1 inner #ffffff",
                height: "fill",
                width: "50%",
                main_align: "center",
                cross_align: "center",

                padding: "8",

                if selected_file.read().is_some(){
                    {preview(selected_file, metadata)}
                } else{ label { "Выберите изображение" } }
            }
        },
        rect { 
            direction: "horizontal",
            height: "35%",
            width: "fill",
            padding: "8",
            
            color: "white",
            font_size: "20",

            border: "1 inner #ffffff",

            {exif_view(metadata)}
        }
    })
}