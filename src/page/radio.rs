use crate::{
    page::{page, widget_example},
    widget::{self, text, Element},
};

use iced::widget::column;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Choice {
    One,
    Two,
    Three,
}

#[derive(Debug, Clone, Default)]
pub struct Radio {
    choice: Option<Choice>,
}

#[derive(Clone, Debug)]
pub enum Message {
    ChoiceSelected(Choice),
}

impl Radio {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChoiceSelected(choice) => self.choice = Some(choice),
        }
    }

    pub fn view(&self) -> Element<Message> {
        page(
            "Radio",
            [widget_example(
                "A group of Radios.",
                column![
                    text::body1("Options:"),
                    widget::radio::standard(
                        "Option 1",
                        Choice::One,
                        self.choice,
                        Message::ChoiceSelected
                    ),
                    widget::radio::standard(
                        "Option 2",
                        Choice::Two,
                        self.choice,
                        Message::ChoiceSelected
                    ),
                    widget::radio::standard(
                        "Option 3",
                        Choice::Three,
                        self.choice,
                        Message::ChoiceSelected
                    ),
                ]
                .spacing(20),
                Some(text::body1(format![
                    "Output:\n {}",
                    match self.choice {
                        Some(choice) => match choice {
                            Choice::One => "You selected Option 1",
                            Choice::Two => "You selected Option 2",
                            Choice::Three => "You selected Option 3",
                        },
                        None => "Select an option.",
                    }
                ])),
                None::<Element<Message>>,
            )],
        )
    }
}
