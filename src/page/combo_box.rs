use crate::{
    page::{page, widget_example},
    widget::{canvas::Rectangle, combo_box, pick_list, text, Canvas, Container, Element},
};

use iced::{
    widget::{column, combo_box::State},
    Color, Length,
};

use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug)]
pub struct ComboBox {
    selected_colour: Option<Colour>,
    font_sizes: State<u16>,
    selected_font_size: u16,
}

impl Default for ComboBox {
    fn default() -> Self {
        Self {
            selected_colour: None,
            font_sizes: State::new(vec![8, 9, 10, 11, 12, 14, 16, 18, 20, 24, 28, 36, 48, 72]),
            selected_font_size: 10,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    ColourSelected(Colour),
    FontSizeSelected(u16),
}

impl ComboBox {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ColourSelected(colour) => self.selected_colour = Some(colour),
            Message::FontSizeSelected(size) => self.selected_font_size = size,
        }
    }

    pub fn view(&self) -> Element<Message> {
        page(
            "ComboBox",
            [
                widget_example(
                    "A PickList with items defined.",
                    column![
                        text::body1("Colours"),
                        pick_list::standard(
                            [Colour::Blue, Colour::Green, Colour::Red, Colour::Yellow],
                            self.selected_colour,
                            Message::ColourSelected,
                        )
                        .placeholder("Pick a colour")
                        .width(200),
                        Container::new(Canvas::new(
                            Rectangle::new(108.0, 32.0).colour(
                                self.selected_colour
                                    .map_or(Color::TRANSPARENT, |colour| colour.into())
                            )
                        ))
                        .center_x(Length::Fill)
                        .height(32)
                    ]
                    .width(200)
                    .spacing(8),
                    None::<Element<Message>>,
                    None::<Element<Message>>,
                ),
                widget_example(
                    "An editable ComboBox.",
                    column![
                        text::body1("Font Size"),
                        combo_box::standard(
                            &self.font_sizes,
                            "",
                            Some(&self.selected_font_size),
                            Message::FontSizeSelected
                        )
                        .width(200),
                        text::body1("You can set the font size used for this text.")
                            .line_height(1.3)
                            .width(Length::Fill)
                            .size(self.selected_font_size)
                    ]
                    .width(Length::Fill)
                    .spacing(8),
                    None::<Element<Message>>,
                    None::<Element<Message>>,
                ),
            ],
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Colour {
    Blue,
    Green,
    Red,
    Yellow,
}

impl From<Colour> for Color {
    fn from(value: Colour) -> Self {
        match value {
            Colour::Blue => Color::from_rgb(0.0, 0.0, 1.0),
            Colour::Green => Color::from_rgb(0.0, 0.5, 0.0),
            Colour::Red => Color::from_rgb(1.0, 0.0, 0.0),
            Colour::Yellow => Color::from_rgb(1.0, 1.0, 0.0),
        }
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            Colour::Blue => "Blue",
            Colour::Green => "Green",
            Colour::Red => "Red",
            Colour::Yellow => "Yellow",
        })
    }
}
