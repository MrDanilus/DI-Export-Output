use std::path::PathBuf;

use freya::prelude::*;

use crate::{
    assets::{cross_icon, folder_look_icon}, 
    core::exif::{self, civitai_request},
    ui::{app::{DnDStatus, Exif}, 
        theme::button_transparent, 
        THEME
    }
};

pub fn file_list(
    selected_file: Signal<PathBuf>,
    metadata:      Signal<Exif>,
    files:         Signal<Vec<PathBuf>>,

    civitai_get:       Signal<bool>,
    file_hover:        Signal<DnDStatus>
) -> Element{
    let theme_value = *THEME.read();

    rsx!(rect{
        height: "fill",
        width: "fill",
        main_align: "center",
        cross_align: "center",

        border: {match *file_hover.read(){
            DnDStatus::Ok => "2 inner rgb(50, 150, 50)",
            DnDStatus::Wrong => "2 inner rgb(175, 50, 50)",
            DnDStatus::Exists => "2 inner rgb(175, 150, 50)",
            _ => if theme_value == 1 {"2 inner #59554c"} else {"2 inner #d4c9b4"}
        }},
        corner_radius: "16",

        if files.read().is_empty(){
            label { 
                font_size: "16",
                text_align: "center",
                color: {match *file_hover.read(){
                    DnDStatus::Ok => "rgb(50, 150, 50)",
                    DnDStatus::Wrong => "rgb(175, 50, 50)",
                    DnDStatus::Exists => "rgb(175, 150, 50)",
                    _ => if theme_value == 1 {"#cccccc"} else {"#323232"}
                }},
                "Переместите изображения/директорию\nв окно"
            }
        } else{{
            let files_read = files.read();
            let files_arr = files_read.clone().into_iter();
    
            let hovered = use_signal(|| PathBuf::new());
            rsx!(VirtualScrollView {
                direction: "vertical",
                padding: "8",

                length: files_arr.len(),
                item_size: 33.0,

                builder: move |i, _args: &Option<()>| {
                    let mut files_arr = files_arr.clone();
                    let file = files_arr.nth(i).unwrap();
                    let file_name = file.file_name().unwrap().to_str().unwrap();
                    rsx!(
                        rect{
                            key: "{i}",
                            direction: "vertical",
                            spacing: "4",

                            {file_element(
                                selected_file, metadata, files, hovered, civitai_get,
                                file.clone(), file_name.to_string()
                            )}
                        }
                    )
                }
            })
        }}
    })
}

fn file_element(
    mut selected_file: Signal<PathBuf>,
    mut metadata:      Signal<Exif>,
    mut files:         Signal<Vec<PathBuf>>,
    mut hovered:       Signal<PathBuf>,
    civitai_get: Signal<bool>,

    file: PathBuf, 
    file_name: String
) -> Element{
    let civitai_value = *civitai_get.read();
    let theme_value = *THEME.read();

    let file_clone = file.clone();
    let file_show = file.clone();
    let file_hover = file.clone();
    rsx!(rect{
        direction: "horizontal",
        cross_align: "center",
        width: "fill",
        height: "32",

        onmouseenter: move |_| *hovered.write() = file_hover.clone(),
        onmouseleave: move |_| *hovered.write() = PathBuf::new(),

        if *hovered.read() == file_hover{
            Button{
                onpress: move |_| {
                    let mut selected_file = selected_file.write();
                    if file_clone == selected_file.clone() {
                        *selected_file = PathBuf::new();
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
                onpress: move |_| {
                    let _ = open::that(&file_show);
                },
                svg {
                    color: if theme_value == 1 {"#cccccc"} else {"#323232"},
                    width: "24",
                    height: "24",
                    svg_data: static_bytes(folder_look_icon())
                }
            }
        },

        Button{
            theme: Some(button_transparent(
                if theme_value == 1 {"#3b3938"} else {"#e4e0d9"}
            )),
            onpress: move |_| {
                let file = file.clone();
                if selected_file.read().clone() == file{
                    return;
                }

                spawn(async move {
                    let mut selected_file = selected_file.clone();
                    metadata.set(Exif::Loading);

                    selected_file.set(file.clone());
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
                margin: "0 0 0 4",
                max_lines: "1",

                color: if theme_value == 1 {"#cccccc"} else {"#323232"},
                font_size: "16",
                font_weight: "bold",
                text_overflow: "ellipsis",

                decoration: if selected_file.read().clone() == file_hover{
                    "underline"
                },
                decoration_style: "dotted",
                { file_name }
            }
        }
    })
}