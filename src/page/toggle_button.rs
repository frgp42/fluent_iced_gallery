use crate::{
    page::{page, widget_example},
    style,
    widget::{self, checkbox, text, Element},
};

#[derive(Clone, Debug)]
pub struct ToggleButton {
    button1_on: bool,
    button1_enabled: bool,
}

impl Default for ToggleButton {
    fn default() -> Self {
        Self {
            button1_on: false,
            button1_enabled: true,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    Button1Toggled,
    Button1EnabledToggled(bool),
}

impl ToggleButton {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Button1Toggled => self.button1_on = !self.button1_on,
            Message::Button1EnabledToggled(checked) => self.button1_enabled = !checked,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let style = if self.button1_on {
            style::button::primary
        } else {
            style::button::secondary
        };

        page(
            "Toggle Button",
            [widget_example(
                "A simple Toggle Button with text content.",
                widget::Button::new(text::body1("Toggle Button"))
                    .height(32)
                    .style(style)
                    .on_press_maybe(self.button1_enabled.then_some(Message::Button1Toggled)),
                Some(text::body1(format![
                    "Output: {}",
                    if self.button1_on { "\nOn" } else { "\nOff" }
                ])),
                Some(
                    checkbox::two_state("Disable Toggle Button", !self.button1_enabled)
                        .on_toggle(Message::Button1EnabledToggled),
                ),
            )],
        )
    }
}
