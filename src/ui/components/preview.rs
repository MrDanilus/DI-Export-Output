use std::{fs, path::PathBuf};

use freya::prelude::*;

use crate::ui::{app::Exif, theme::button_transparent, THEME};

pub fn preview(
    mut selected_file: Signal<Option<PathBuf>>,
    mut metadata:      Signal<Exif>
) -> Element{
    let theme_value = *THEME.read();
    rsx!({
        let path = selected_file.read().clone().unwrap();
        match fs::read(path){
            Ok(file) => {
                rsx!(
                    Button{
                        theme: Some(button_transparent(
                            if theme_value == 1 {"#3b3938"} else {"#e4e0d9"}
                        )),
                        onpress: move |_| {
                            selected_file.set(None);
                            metadata.set(Exif::None);
                        },
                        rect{
                            width: "fill",
                            height: "fill",
                            main_align: "center",
                            cross_align: "center",

                            padding: "12",
                            corner_radius: "12",
                            rect{
                                shadow: if theme_value == 1 {"0 0 16 4 #403c36"} else {"0 0 16 4 #c4b9a6"},
                                image { 
                                    sampling: "mitchell",
                                    image_data: dynamic_bytes(file) 
                                }
                            }
                        }
                    }
                )
            },
            Err(err) => {
                rsx!( label { 
                    text_align: "center",
                    {format!("Не удалось загрузить изображение:\n{err}")}
                } )
            }
        }
    })
}