use core::App;

mod core;
mod ui;

fn main() {
    let app = iced::application("SD Image 2 Params", App::update, App::view)
        .window_size(iced::Size::from((800.0, 700.0)))
        .subscription(App::subscription);
    app.run().unwrap();
}