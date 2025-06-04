#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use freya::prelude::*;

mod core;
use core::config::init_theme;
mod ui;
use ui::view;
mod assets;
use assets::JETBRAINS_REGULAR;

fn main() {
    init_theme();
    
    launch_cfg(
        view,
        LaunchConfig::<()>::new()
            .with_size(800.0, 800.0)
            .with_title("SD Image 2 Params")
            .with_font("JetBrains Regular", JETBRAINS_REGULAR)
    );
}