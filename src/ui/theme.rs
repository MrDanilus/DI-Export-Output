use freya::prelude::*;

pub const CUSTOM_THEME: Theme = Theme{
    button: ButtonTheme{
        background: cow_borrowed!("transparent"),
        hover_background: cow_borrowed!("transparent"),
        border_fill: cow_borrowed!("transparent"),
        focus_border_fill: cow_borrowed!("transparent"),
        corner_radius: cow_borrowed!("0"),
        margin: cow_borrowed!("0"),
        padding: cow_borrowed!("0"),
        ..LIGHT_THEME.button
    },
    ..LIGHT_THEME
};

pub fn button_transparent(hover_color: &'static str) -> ButtonThemeWith{
    ButtonThemeWith{
        background: Some(cow_borrowed!("transparent")),
        hover_background: Some(cow_borrowed!(hover_color)),
        border_fill: Some(cow_borrowed!("transparent")),
        focus_border_fill: Some(cow_borrowed!("transparent")),
        corner_radius: Some(cow_borrowed!("12")),
        padding: Some(cow_borrowed!("2")),
        ..Default::default()
    }
}