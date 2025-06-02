use std::{fs, path::PathBuf};
use freya::prelude::*;
use copypasta::{ClipboardContext, ClipboardProvider};

use crate::{core::{exif, file::{check_file, write}}, ui::theme::button_transparent};

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

    let mut selected_file = use_signal(|| None);
    let mut metadata = use_signal(|| Exif::None);
    
    let mut show_popup = use_signal(|| false);

    rsx!(
    rect{
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
                
                rect {  
                    direction: "horizontal",
                    width: "fill",
                    content: "flex",
                    padding: "4",
                    spacing: "4",

                    Button{
                        theme: Some(button_transparent("#6e6e6e")),
                        onpress: move |_| {
                            selected_file.set(None);
                            metadata.set(Exif::None);
                            files.write().clear()
                        },
                        rect{
                            direction: "horizontal",
                            main_align: "center",
                            width: "50%",
                            height: "fill-min",
                            label{
                                "🗑️"
                            }
                        }
                    },
                    Button{
                        theme: Some(button_transparent("#6e6e6e")),
                        onpress: move |_| {
                            let mut params = Vec::new();
                            for file in &files.read().clone(){
                                match exif::parse_image(&file){
                                    Ok(res) => params.push(
                                        format!("[{file:?}]\n{}", res)
                                    ),
                                    Err(_) => {},
                                }
                            }
                            let res = if params.is_empty(){
                                "Нет параметров".to_string()
                            } else{
                                params.join("\n---\n")
                            };
                            let mut ctx = ClipboardContext::new().unwrap();
                            ctx.set_contents(res).unwrap();
                            show_popup.set(true);
                        },
                        rect{
                            direction: "horizontal",
                            main_align: "center",
                            width: "fill",
                            height: "fill-min",
                            label{
                                "📲"
                            }
                        }
                    }
                },
                
                rect{
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
                                            metadata.set(match exif::parse_image(&file){
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
                }
            },
            rect {
                border: "1 inner #ffffff",
                height: "fill",
                width: "50%",
                main_align: "center",
                cross_align: "center",

                padding: "8",

                if selected_file.read().is_some(){
                    {
                        let path = selected_file.read().clone().unwrap();
                        match fs::read(path){
                            Ok(file) => {
                                rsx!(
                                    Button{
                                        theme: Some(button_transparent("#6e6e6e")),
                                        onpress: move |_| {
                                            selected_file.set(None);
                                            metadata.set(Exif::None);
                                        },
                                        image { image_data: dynamic_bytes(file) }
                                    }
                                )
                            },
                            Err(err) => {
                                rsx!( label { 
                                    text_align: "center",
                                    color: "white",
                                    {format!("Не удалось загрузить изображение:\n{err}")}
                                } )
                            }
                        }
                    }
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

            {match metadata.read().clone(){
                Exif::Ok(res) => rsx!(
                    ScrollView {
                        direction: "vertical",
                        SelectableText {
                            value: res
                        }
                    }
                ),
                Exif::Err(err) => rsx!(label{
                    color: "red",
                    {err}
                }),
                Exif::None => rsx!()
            }}
        },
        if *show_popup.read() {
            Popup {
                oncloserequest: move |_| {
                    show_popup.set(false)
                },
                PopupTitle {
                    label {
                        "Параметры сохранены в буфер обмена ✅"
                    }
                }
            }
        }
    }
    )
}