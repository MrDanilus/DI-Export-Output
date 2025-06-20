#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use freya::prelude::*;

mod core;
mod ui;
use ui::view;
mod assets;
use assets::JETBRAINS_REGULAR;

const ICON: &[u8] = include_bytes!("../icons/icon.png");

fn main() {
    launch_cfg(
        view,
        LaunchConfig::<()>::new()
            .with_size(800.0, 800.0)
            .with_title("SD Image 2 Params")
            .with_font("JetBrains Regular", JETBRAINS_REGULAR)
            .with_icon(LaunchConfig::load_icon(ICON))
    );
}