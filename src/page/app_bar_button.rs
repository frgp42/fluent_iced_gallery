use crate::{
    fluent_icon::FluentIcon,
    page::{page, widget_example},
    widget::{button, text, Element},
};

#[derive(Clone, Debug, Default)]
pub struct AppBarButton {
    button1_pressed: bool,
}

#[derive(Clone, Debug)]
pub enum Message {
    Button1Pressed,
}

impl AppBarButton {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Button1Pressed => self.button1_pressed = true,
        }
    }

    pub fn view(&self) -> Element<Message> {
        page(
            "AppBar Button",
            [widget_example(
                "An AppBar button with a symbol icon.",
                button::app_bar("Icon", FluentIcon::Like).on_press(Message::Button1Pressed),
                Some(text::body1(if self.button1_pressed {
                    "You clicked: Button1"
                } else {
                    ""
                })),
                None::<Element<Message>>,
            )],
        )
    }
}
