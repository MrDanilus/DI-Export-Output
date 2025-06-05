use std::path::PathBuf;
use freya::prelude::*;

use crate::{
    core::file::{check_file, write}, 
    ui::{components::{
        exif_view::exif_view, file_list::file_list, header::header, preview::preview
    }, THEME}
};

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
    Loading,
    None
}

pub fn app() -> Element{
    let mut file_hover = use_signal(|| DnDStatus::None);
    let mut files: Signal<Vec<PathBuf>> = use_signal(|| Vec::new());

    let selected_file = use_signal(|| PathBuf::new());
    let civitai_request = use_signal(|| false);
    let metadata = use_signal(|| Exif::None);

    let theme_value = *THEME.read();

    rsx!(rect{
        direction: "vertical",
        height: "fill",
        width: "fill",
        
        background: if theme_value == 1 {"#292827"} else {"#f0ece5"},
        padding: "16",
        spacing: "8",

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

            spacing: "8",

            rect {
                direction: "vertical",
                height: "fill",
                width: "50%",
                main_align: "center",
                cross_align: "center",

                spacing: "8",
                
                {header(civitai_request, selected_file, metadata, files)},
                
                {file_list(selected_file, metadata, files, civitai_request, file_hover)}
            },
            rect {
                height: "fill",
                width: "fill",
                main_align: "center",
                cross_align: "center",

                padding: "8",

                border: if theme_value == 1 {"2 inner #59554c"} else {"2 inner #d4c9b4"},
                corner_radius: "16",
                
                color: if theme_value == 1 {"#cccccc"} else {"#323232"},
                font_size: "20",

                if selected_file.read().is_file(){
                    {preview(selected_file, metadata)}
                } else if !files.read().is_empty(){ 
                    label { "Выберите изображение" } 
                }
            }
        },
        rect { 
            direction: "horizontal",
            height: "35%",
            width: "fill",
            padding: "8",
            
            border: if theme_value == 1 {"2 inner #59554c"} else {"2 inner #d4c9b4"},
            corner_radius: "16",
            
            color: if theme_value == 1 {"#cccccc"} else {"#323232"},
            font_size: "15",

            {exif_view(metadata)}
        }
    })
}