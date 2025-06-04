use freya::prelude::*;

use crate::core::config::get_theme;

mod components;
mod theme;
use theme::CUSTOM_THEME;
pub mod app;

static THEME: GlobalSignal<u8> = Signal::global(|| 0);

pub fn view() -> Element{
    use_init_default_theme();
    *THEME.write() = if get_theme() == "dark" {1} else {0};

    rsx!(rect{
        direction: "vertical",
        height: "fill",
        width: "fill",

        font_family: "JetBrains Regular",
        ThemeProvider{
            theme: CUSTOM_THEME,
            {app::app()}
        }
    })
}