use std::path::PathBuf;

use freya::prelude::*;

use crate::{
    assets::civitai_logo, core::params::save_to_clipboard, ui::{
        theme::{button_background, button_transparent}, Exif
    }
};

pub fn header(
    mut civitai_request: Signal<bool>,
    mut selected_file:   Signal<Option<PathBuf>>,
    mut metadata:        Signal<Exif>,
    mut files:           Signal<Vec<PathBuf>>
) -> Element{
    let civitai_value = *civitai_request.clone().read();
    let mut show_popup = use_signal(|| false);

    rsx!(rect {  
        direction: "horizontal",
        width: "fill",
        content: "flex",
        padding: "4",
        spacing: "4",

        font_size: "27",

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
                width: "33%",
                height: "fill-min",
                label{
                    "🗑️"
                }
            }
        },
        Button{
            theme: Some(button_transparent("#6e6e6e")),
            onpress: move |_| {
                let mut req_files = Vec::new();
                for file in &files.read().clone(){
                    req_files.push(file.clone());
                }
                save_to_clipboard(req_files, civitai_value);
                show_popup.set(true);
            },
            rect{
                direction: "horizontal",
                main_align: "center",
                width: "50%",
                height: "fill-min",
                label{
                    "📲"
                }
            }
        },
        Button{
            theme: Some(button_background(match *civitai_request.read(){
                true => "#00a600",
                false => "transparent"
            })),
            onpress: move |_| 
                civitai_request.set(!(civitai_value.clone())),
            rect{
                direction: "horizontal",
                main_align: "center",
                width: "fill",
                height: "fill-min",
                image{
                    image_data: static_bytes(civitai_logo()),
                    width: "31",
                    height: "31"
                }
            }
        }
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
    })
}