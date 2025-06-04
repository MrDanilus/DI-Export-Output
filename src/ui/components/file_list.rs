use std::path::PathBuf;

use freya::prelude::*;

use crate::{assets::cross_icon, core::exif::{self, civitai_request}, ui::{app::{DnDStatus, Exif}, theme::button_transparent, THEME}};

pub fn file_list(
    mut selected_file: Signal<Option<PathBuf>>,
    mut metadata:      Signal<Exif>,
    mut files:         Signal<Vec<PathBuf>>,

    civitai_get:       Signal<bool>,
    file_hover:        Signal<DnDStatus>
) -> Element{
    let civitai_value = *civitai_get.read();
    let theme_value = *THEME.read();

    rsx!(rect{
        height: "fill",
        width: "fill",
        main_align: "center",
        cross_align: "center",

        border: {match *file_hover.read(){
            DnDStatus::Ok => "2 inner rgb(50, 150, 50)",
            DnDStatus::Wrong => "2 inner rgb(150, 50, 50)",
            DnDStatus::Exists => "2 inner rgb(150, 150, 50)",
            _ => if theme_value == 1 {"2 inner #59554c"} else {"2 inner #d4c9b4"}
        }},
        corner_radius: "16",

        if files.read().is_empty(){
            label { 
                font_size: "16",
                text_align: "center",
                color: {match *file_hover.read(){
                    DnDStatus::Ok => "rgb(50, 150, 50)",
                    DnDStatus::Wrong => "rgb(150, 50, 50)",
                    DnDStatus::Exists => "rgb(150, 150, 50)",
                    _ => if theme_value == 1 {"#cccccc"} else {"#323232"}
                }},
                "Переместите изображения/директорию\nв окно"
            }
        } else{
            ScrollView {
                padding: "8",
                direction: "vertical",
                rect{
                    direction: "vertical",
                    spacing: "4",

                    for file in files.read().clone(){{
                        let file_clone = file.clone();
                        let file_name = file_clone.file_name().unwrap().to_str().unwrap();
                        let file_clone = file.clone();
                        rsx!(rect{
                            direction: "horizontal",
                            cross_align: "center",
                            width: "fill",

                            Button{
                                onpress: move |_| {
                                    let mut selected_file = selected_file.write();
                                    if selected_file.is_some() && 
                                    file_clone == selected_file.clone().unwrap() {
                                        *selected_file = None;
                                        metadata.set(Exif::None);
                                    }
                                    files.write().retain(|f| *f != file_clone);
                                },
                                svg {
                                    color: if theme_value == 1 {"#cccccc"} else {"#323232"},
                                    width: "28",
                                    height: "28",
                                    svg_data: static_bytes(cross_icon())
                                }
                            },
                            Button{
                                theme: Some(button_transparent(
                                    if theme_value == 1 {"#3b3938"} else {"#e4e0d9"}
                                )),
                                onpress: move |_| {
                                    let file = file.clone();
                                    if selected_file.read().is_some() &&
                                    selected_file.read().clone().unwrap() == file{
                                        return;
                                    }

                                    spawn(async move {
                                        let mut selected_file = selected_file.clone();
                                        metadata.set(Exif::Loading);

                                        selected_file.set(Some(file.clone()));
                                        metadata.set(match exif::parse_image(&file){
                                            Ok(res) => Exif::Ok(if civitai_value{
                                                civitai_request(res).await.to_string()
                                            } else {res.to_string()}),
                                            Err(err) => Exif::Err(err),
                                        });
                                    });
                                },
                                label {
                                    width: "fill",
                                    max_lines: "1",

                                    color: if theme_value == 1 {"#cccccc"} else {"#323232"},
                                    font_size: "16",
                                    font_weight: "bold",
                                    text_overflow: "ellipsis",
                                    { file_name }
                                }
                            }
                        })
                    }}
                }
            }
        }
    })
}