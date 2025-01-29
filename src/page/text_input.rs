use crate::{
    page::{page, widget_example},
    theme::Theme,
    widget::{text, text_input, Element},
};

use iced::{
    color,
    font::{Family, Style as FontStyle},
    widget::{
        column,
        text::LineHeight,
        text_input::{Catalog, Status, Style},
    },
    Font, Pixels,
};

#[derive(Clone, Debug, Default)]
pub struct TextInput {
    text_input1_content: String,
    text_input2_content: String,
}

#[derive(Clone, Debug)]
pub enum Message {
    TextInput1ContentChanged(String),
    TextInput2ContentChanged(String),
    NoOp,
}

impl TextInput {
    const EXAMPLE_FONT: Font = Font {
        family: Family::Name("en Arial"),
        style: FontStyle::Italic,
        ..Font::DEFAULT
    };

    pub fn update(&mut self, message: Message) {
        match message {
            Message::TextInput1ContentChanged(content) => self.text_input1_content = content,
            Message::TextInput2ContentChanged(content) => self.text_input2_content = content,
            Message::NoOp => (),
        }
    }

    pub fn view(&self) -> Element<Message> {
        page(
            "Text Input",
            [
                widget_example(
                    "A simple TextInput.",
                    text_input::underline(
                        text_input::standard("", &self.text_input1_content)
                            .width(64)
                            .on_input(Message::TextInput1ContentChanged),
                    ),
                    None::<Element<Message>>,
                    None::<Element<Message>>,
                ),
                widget_example(
                    "A TextInput with a header and placeholder text.",
                    column![
                        text::body1("Enter your name:"),
                        text_input::underline(
                            text_input::standard("Name", &self.text_input2_content)
                                .width(104)
                                .on_input(Message::TextInput2ContentChanged)
                        ),
                    ]
                    .spacing(8),
                    None::<Element<Message>>,
                    None::<Element<Message>>,
                ),
                widget_example(
                    "A read-only TextInput with a different style set.",
                    text_input::underline(
                        text_input::standard("", "I am super excited to be here!")
                            .font(Self::EXAMPLE_FONT)
                            .line_height(LineHeight::Absolute(Pixels(32.0)))
                            .size(24)
                            .width(328)
                            .on_input(|_| Message::NoOp)
                            .style(|theme: &Theme, status| {
                                let style = (<Theme as Catalog>::default())(theme, status);

                                match status {
                                    Status::Active => Style {
                                        value: color!(0x5178BE),
                                        ..style
                                    },
                                    Status::Hovered => style,
                                    Status::Focused => style,
                                    Status::Disabled => style,
                                }
                            }),
                    ),
                    None::<Element<Message>>,
                    None::<Element<Message>>,
                ),
            ],
        )
    }
}
