use std::{fs, path::PathBuf};

use freya::prelude::*;

use crate::{ui::{theme::button_transparent, Exif}};

pub fn preview(
    mut selected_file: Signal<Option<PathBuf>>,
    mut metadata:      Signal<Exif>
) -> Element{
    rsx!({
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
    })
}