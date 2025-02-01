mod fluent_icon;
mod font;
mod gallery;
mod page;
mod style;
mod theme;
mod widget;

use gallery::Gallery;

use iced::window::{icon, Settings};

fn main() -> iced::Result {
    let icon = icon::from_file_data(include_bytes!("../assets/images/logo.png"), None);

    iced::application("Fluent Iced Gallery", Gallery::update, Gallery::view)
        .subscription(Gallery::subscription)
        .theme(Gallery::theme)
        .window(Settings {
            min_size: Some((500.0, 500.0).into()),
            icon: icon.ok(),
            ..Settings::default()
        })
        .run()
}
