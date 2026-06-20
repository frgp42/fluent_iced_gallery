mod fluent_icon;
mod font;
mod gallery;
mod page;
mod style;
mod theme;
mod widget;

use gallery::Gallery;

use iced::window::icon;

fn main() -> iced::Result {
    let icon = icon::from_file_data(include_bytes!("../assets/images/logo.png"), None);

    font::set();

    let settings = iced::Settings {
        fonts: font::load(),
        antialiasing: true,
        default_font: font::UI_REGULAR.clone().into(),
        ..Default::default()
    };

    let window_settings = iced::window::Settings {
        min_size: Some((500.0, 500.0).into()),
        icon: icon.ok(),
        ..Default::default()
    };

    iced::application("Fluent Iced Gallery", Gallery::update, Gallery::view)
        .subscription(Gallery::subscription)
        .theme(Gallery::theme)
        .settings(settings)
        .window(window_settings)
        .run()
}
