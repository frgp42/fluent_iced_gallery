use crate::{
    page::{page, widget_example},
    widget::{self, button, checkbox, text, Element},
};

use iced::{widget::image, Padding};

#[derive(Clone, Debug)]
pub struct Button {
    pub button1_pressed: bool,
    pub button1_enabled: bool,
    pub button2_pressed: bool,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            button1_pressed: false,
            button1_enabled: true,
            button2_pressed: false,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    Button1Pressed,
    Button1EnabledToggled(bool),
    Button2Pressed,
}

impl Button {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Button1Pressed => self.button1_pressed = true,
            Message::Button1EnabledToggled(checked) => self.button1_enabled = !checked,
            Message::Button2Pressed => self.button2_pressed = true,
        }
    }

    pub fn view(&self) -> Element<Message> {
        page(
            "Button",
            [
                widget_example(
                    "A simple Button with text content.",
                    button::standard(text::body1("Standard button"))
                        .on_press_maybe(self.button1_enabled.then_some(Message::Button1Pressed)),
                    Some(text::body1(format![
                        "Output: {}",
                        if self.button1_pressed {
                            "\nYou clicked: Button1"
                        } else {
                            ""
                        }
                    ])),
                    Some(
                        checkbox::two_state("Disable button", !self.button1_enabled)
                            .on_toggle(Message::Button1EnabledToggled),
                    ),
                ),
                widget_example(
                    "A Button with graphical content.",
                    widget::Button::new(image(format! {
                        "{}/assets/images/Slices.png", env!("CARGO_MANIFEST_DIR")
                    }))
                    .width(50)
                    .height(50)
                    .padding(Padding::from(12))
                    .on_press(Message::Button2Pressed),
                    Some(text::body1(format![
                        "Output: {}",
                        if self.button2_pressed {
                            "\nYou clicked: Button2"
                        } else {
                            ""
                        }
                    ])),
                    None::<Element<Message>>,
                ),
            ],
        )
    }
}
