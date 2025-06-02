#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use freya::prelude::*;

mod core;
mod ui;
use ui::view;
mod assets;

fn main() {
    launch_cfg(
        view,
        LaunchConfig::<()>::new()
            .with_size(800.0, 700.0)
            .with_title("SD Image 2 Params")
    );
}