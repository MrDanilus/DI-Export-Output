use std::path::PathBuf;

use freya::prelude::*;

use crate::{
    assets::{civitai_logo, export_icon, loading_icon, moon_icon, sun_icon, trash_icon}, core::{clipboard::save_to_clipboard, config::write_theme}, ui::{
        app::Exif, theme::button_transparent, THEME
    }
};

pub fn header(
    mut civitai_request: Signal<bool>,
    mut selected_file:   Signal<PathBuf>,
    mut metadata:        Signal<Exif>,
    mut files:           Signal<Vec<PathBuf>>
) -> Element{
    let animation = use_animation(|conf| {
        conf.on_finish(OnFinish::Restart).auto_start(true);
        AnimNum::new(0., 360.).time(1000)
    });
    let rotate = animation.get().read().read();
    let mut exporting = use_signal(|| false);

    let civitai_value = *civitai_request.clone().read();
    
    let mut show_popup = use_signal(|| String::new());

    let theme_value = *THEME.read();

    rsx!(rect {  
        direction: "horizontal",
        cross_align: "center",
        width: "fill",
        height: "48",

        content: "flex",
        padding: "4",
        spacing: "4",

        font_size: "27",

        Button{
            theme: Some(button_transparent(
                if theme_value == 1 {"#3d3c3a"} else {"#e3dfd5"}
            )),
            onpress: move |_| {
                let res_theme = match theme_value{
                    0 => "dark",
                    _ => "light"
                };
                *THEME.write() = if theme_value == 0 { 1 } else { 0 };
                write_theme(res_theme.to_string());
            },
            rect{
                direction: "horizontal",
                main_align: "center",
                width: "25%",
                height: "fill-min",

                padding: "4",
                svg{
                    width: "31",
                    height: "31",
                    svg_data: static_bytes(
                        match theme_value{
                            1 => sun_icon(),
                            _ => moon_icon()
                        }
                    )
                }
            }
        },
        Button{
            theme: Some(button_transparent(
                if theme_value == 1 {"#3d3c3a"} else {"#e3dfd5"}
            )),
            onpress: move |_| {
                selected_file.set(PathBuf::new());
                metadata.set(Exif::None);
                files.write().clear()
            },
            rect{
                direction: "horizontal",
                main_align: "center",
                width: "33%",
                height: "fill-min",

                padding: "4",
                svg{
                    color: if theme_value == 1 {"#cccccc"} else {"#323232"},
                    svg_data: static_bytes(trash_icon()),
                    width: "32",
                    height: "32"
                }
            }
        },
        Button{
            theme: Some(button_transparent(
                if theme_value == 1 {"#3d3c3a"} else {"#e3dfd5"}
            )),
            onpress: move |_| {
                exporting.set(true);
                animation.start();

                spawn(async move {
                    let mut req_files = Vec::new();
                    for file in &files.read().clone(){
                        req_files.push(file.clone());
                    }
                    let res = save_to_clipboard(req_files, civitai_value).await;
                    show_popup.set(match res{
                        true => String::from("OK"),
                        false => String::from("NOT_FOUND")
                    });
                    animation.finish();
                    exporting.set(false);
                });
            },
            rect{
                direction: "horizontal",
                main_align: "center",
                width: "50%",
                height: "fill-min",

                padding: "4",
                svg{
                    width: "32",
                    height: "32",

                    color: if theme_value == 1 {"#cccccc"} else {"#323232"},
                    rotate: if *exporting.read() {"{rotate}deg"},
                    svg_data: static_bytes(
                        if *exporting.read() {loading_icon()}
                        else {export_icon()}
                    )
                }
            }
        },
        Button{
            onpress: move |_| 
                civitai_request.set(!(civitai_value.clone())),
            rect{
                direction: "horizontal",
                main_align: "center",
                width: "fill",
                height: "fill-min",

                background: match *civitai_request.read(){
                    true => if theme_value == 1 {"#829c65"} else {"#c0e595"},
                    false => "transparent"
                },
                corner_radius: "12",

                padding: "6",
                svg{
                    svg_data: static_bytes(civitai_logo()),
                    width: "32",
                    height: "32"
                }
            }
        }
    },
    if !show_popup.read().is_empty() {
        rect{
            position: "absolute",
            Popup {
                oncloserequest: move |_| {
                    show_popup.write().clear();
                },
                PopupTitle {
                    label {
                        if show_popup.read().as_str() == "OK"{
                            "Параметры сохранены в буфер обмена ✅"
                        } else if show_popup.read().as_str() == "NOT_FOUND"{
                            "Нечего сохранить ❌"
                        } else{
                            "Ошибка сохранения"
                        }
                    }
                }
            }
        }
    })
}