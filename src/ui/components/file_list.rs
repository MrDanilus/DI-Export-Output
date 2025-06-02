use std::path::PathBuf;

use freya::prelude::*;

use crate::{core::exif, ui::{theme::button_transparent, DnDStatus, Exif}};

pub fn file_list(
    mut selected_file: Signal<Option<PathBuf>>,
    mut metadata:      Signal<Exif>,
    mut files:         Signal<Vec<PathBuf>>,

    civitai_request:   Signal<bool>,
    file_hover:        Signal<DnDStatus>
) -> Element{
    let civitai_value = *civitai_request.read();
    rsx!(rect{
        border: "1 inner white",
        height: "fill",
        width: "fill",
        main_align: "center",
        cross_align: "center",

        background: {match *file_hover.read(){
            DnDStatus::Ok => "#022e00",
            DnDStatus::Wrong => "#2e0000",
            DnDStatus::Exists => "#2e2d00",
            _ => "black"
        }},

        if files.read().is_empty(){
            label { 
                font_size: "16",
                text_align: "center",
                color: {match *file_hover.read(){
                    DnDStatus::Ok => "green",
                    DnDStatus::Wrong => "red",
                    DnDStatus::Exists => "yellow",
                    _ => "white"
                }},
                "Переместите изображения/директорию\nв окно"
            }
        } else{
            ScrollView {
                padding: "8",
                direction: "vertical",
                for file in files.read().clone(){{
                    let file_clone = file.clone();
                    let file_name = file_clone.file_name().unwrap().to_str().unwrap();
                    let file_clone = file.clone();
                    rsx!(rect{
                        direction: "horizontal",
                        width: "fill",

                        Button{
                            theme: Some(button_transparent("#6e6e6e")),
                            onpress: move |_| {
                                let mut selected_file = selected_file.write();
                                if selected_file.is_some() && 
                                file_clone == selected_file.clone().unwrap() {
                                    *selected_file = None;
                                    metadata.set(Exif::None);
                                }
                                files.write().retain(|f| *f != file_clone);
                            },
                            label {
                                "❌"
                            }
                        },
                        Button{
                            theme: Some(button_transparent("#6e6e6e")),
                            onpress: move |_| {
                                selected_file.set(Some(file.clone()));
                                metadata.set(match exif::parse_image(&file, civitai_value){
                                    Ok(res) => Exif::Ok(res),
                                    Err(err) => Exif::Err(err),
                                });
                            },
                            label {
                                width: "fill",
                                { file_name }
                            }
                        }
                    })
                }}
            }
        }
    })
}