use crate::{
    page::{page, widget_example},
    widget::{toggler, Element},
};

#[derive(Clone, Debug, Default)]
pub struct Toggler {
    is_toggled: bool,
}

#[derive(Clone, Debug)]
pub enum Message {
    Toggled(bool),
}

impl Toggler {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Toggled(toggled) => self.is_toggled = toggled,
        }
    }

    pub fn view(&self) -> Element<Message> {
        page(
            "Toggler",
            [widget_example(
                "A simple Toggler.",
                toggler::standard(self.is_toggled)
                    .label(if self.is_toggled { "On" } else { "Off" })
                    .on_toggle(Message::Toggled),
                None::<Element<Message>>,
                None::<Element<Message>>,
            )],
        )
    }
}
