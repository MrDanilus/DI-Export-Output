use freya::prelude::*;

use crate::ui::app::Exif;

pub fn exif_view(
    metadata: Signal<Exif>
) -> Element{
    rsx!({match metadata.read().clone(){
        Exif::Ok(res) => rsx!(
            ScrollView {
                direction: "vertical",
                SelectableText {
                    value: res
                }
            }
        ),
        Exif::Err(err) => rsx!(rect{
            width: "fill",
            height: "fill",

            main_align: "center",
            cross_align: "center",
            label{
                {err}
            }
        }),
        Exif::Loading => rsx!(rect{
            width: "fill",
            height: "fill",

            main_align: "center",
            cross_align: "center",
            label{
                "Загрузка..."
            }
        }),
        Exif::None => rsx!()
    }})
}