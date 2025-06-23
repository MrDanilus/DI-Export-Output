use std::path::PathBuf;

use freya::prelude::*;

use crate::ui::{app::Exif, theme::button_transparent, THEME};

pub fn preview(
    mut selected_file: Signal<PathBuf>,
    mut metadata:      Signal<Exif>,
    preview_image:     Signal<Vec<u8>>
) -> Element{
    let theme_value = *THEME.read();
    rsx!({
        match preview_image.len() > 0{
            true => {
                rsx!(
                    Button{
                        theme: Some(button_transparent(
                            if theme_value == 1 {"#3b3938"} else {"#e4e0d9"}
                        )),
                        onpress: move |_| {
                            selected_file.set(PathBuf::new());
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
                                    image_data: dynamic_bytes(preview_image.read().clone())
                                }
                            }
                        }
                    }
                )
            },
            false => {
                rsx!( label { 
                    text_align: "center",
                    "Не удалось загрузить изображение"
                } )
            }
        }
    })
}