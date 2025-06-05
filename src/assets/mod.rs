pub fn civitai_logo() -> &'static [u8] {
    include_bytes!("icons/civitai-color.svg")
}

// Icons
pub fn cross_icon() -> &'static [u8] {
    include_bytes!("icons/cross.svg")
}
pub fn folder_look_icon() -> &'static [u8]{
    include_bytes!("icons/folder-look.svg")
}

pub fn loading_icon() -> &'static [u8]{
    include_bytes!("icons/loading.svg")
}

pub fn moon_icon() -> &'static [u8] {
    include_bytes!("icons/moon.svg")
}
pub fn sun_icon() -> &'static [u8] {
    include_bytes!("icons/sun.svg")
}

pub fn export_icon() -> &'static [u8]{
    include_bytes!("icons/export.svg")
}
pub fn trash_icon() -> &'static [u8]{
    include_bytes!("icons/trash.svg")
}

// Fonts
pub static JETBRAINS_REGULAR: &[u8] = include_bytes!("fonts/JetBrainsMono-Regular.ttf");