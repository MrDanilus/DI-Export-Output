use freya::hooks::{cow_borrowed, ButtonThemeWith, FontThemeWith};

pub fn button_transparent(hover_color: &'static str) -> ButtonThemeWith{
    ButtonThemeWith{
        font_theme: Some(FontThemeWith{
            color: Some(cow_borrowed!("white"))
        }),
        background: Some(cow_borrowed!("transparent")),
        hover_background: Some(cow_borrowed!(hover_color)),
        border_fill: Some(cow_borrowed!("transparent")),
        focus_border_fill: Some(cow_borrowed!("transparent")),
        corner_radius: Some(cow_borrowed!("0")),
        ..Default::default()
    }
}

pub fn button_background(background: &'static str) -> ButtonThemeWith{
    ButtonThemeWith{
        font_theme: Some(FontThemeWith{
            color: Some(cow_borrowed!("white"))
        }),
        background: Some(cow_borrowed!(background)),
        hover_background: Some(cow_borrowed!(background)),
        border_fill: Some(cow_borrowed!("transparent")),
        focus_border_fill: Some(cow_borrowed!("transparent")),
        corner_radius: Some(cow_borrowed!("0")),
        ..Default::default()
    }
}