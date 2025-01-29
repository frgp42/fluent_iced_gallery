use crate::{
    page::{page, widget_example},
    style,
    widget::{button, Button, Container, Element, Wrap},
};

use iced::{
    border::Radius,
    widget::{button::Style, vertical_space},
    Border, Color, Padding,
};

#[derive(Clone, Debug)]
pub struct SplitButton {
    open_splitbutton: OpenSplitButton,
    colour1: Color,
    colour2: Color,
}

impl Default for SplitButton {
    fn default() -> Self {
        Self {
            open_splitbutton: OpenSplitButton::None,
            colour1: COLOURS[3],
            colour2: COLOURS[3],
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum OpenSplitButton {
    #[default]
    None,
    One,
    Two,
}

#[derive(Clone, Debug)]
pub enum Message {
    SplitButton1Pressed,
    SplitButton2Pressed,
    SplitButton1Closed,
    SplitButton2Closed,
    Colour1Selected(Color),
    Colour2Selected(Color),
    NoOp,
}

impl SplitButton {
    pub fn update(&mut self, message: Message) {
        let mut close_all = || self.open_splitbutton = OpenSplitButton::None;

        match message {
            Message::SplitButton1Pressed => {
                if self.open_splitbutton == OpenSplitButton::None {
                    self.open_splitbutton = OpenSplitButton::One
                } else {
                    self.open_splitbutton = OpenSplitButton::None
                }
            }
            Message::SplitButton2Pressed => {
                if self.open_splitbutton == OpenSplitButton::None {
                    self.open_splitbutton = OpenSplitButton::Two
                } else {
                    self.open_splitbutton = OpenSplitButton::None
                }
            }
            Message::SplitButton1Closed => close_all(),
            Message::SplitButton2Closed => close_all(),
            Message::Colour1Selected(colour) => {
                self.colour1 = colour;
                close_all();
            }
            Message::Colour2Selected(colour) => {
                self.colour2 = colour;
                close_all();
            }
            Message::NoOp => (),
        }
    }

    pub fn view(&self) -> Element<Message> {
        page(
            "Split Button",
            [
                widget_example(
                    "A Split Button.",
                    button::split(
                        Button::new(vertical_space()).width(32).height(32).style(
                            move |theme, status| {
                                let base = style::button::secondary(theme, status);

                                Style {
                                    background: Some(self.colour1.into()),
                                    border: Border {
                                        radius: Radius::new(4).right(0),
                                        ..base.border
                                    },
                                    ..base
                                }
                            },
                        ),
                        colour_grid(&COLOURS[..8], Message::Colour1Selected),
                        Message::SplitButton1Pressed,
                        Message::SplitButton1Closed,
                        matches!(self.open_splitbutton, OpenSplitButton::One),
                    ),
                    None::<Element<Message>>,
                    None::<Element<Message>>,
                ),
                widget_example(
                    "A Split Button with text.",
                    button::split(
                        button::standard("Choose color")
                            .style(style::button::split_content)
                            .on_press(Message::NoOp),
                        colour_grid(&COLOURS[..], Message::Colour2Selected),
                        Message::SplitButton2Pressed,
                        Message::SplitButton2Closed,
                        matches!(self.open_splitbutton, OpenSplitButton::Two),
                    ),
                    None::<Element<Message>>,
                    None::<Element<Message>>,
                ),
            ],
        )
    }
}

const COLOURS: [Color; 9] = [
    Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    },
    Color {
        r: 1.0,
        g: 0.65,
        b: 0.0,
        a: 1.0,
    },
    Color {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    },
    Color {
        r: 0.0,
        g: 0.5,
        b: 0.0,
        a: 1.0,
    },
    Color {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    },
    Color {
        r: 0.29,
        g: 0.0,
        b: 0.51,
        a: 1.0,
    },
    Color {
        r: 0.93,
        g: 0.51,
        b: 0.93,
        a: 1.0,
    },
    Color {
        r: 0.5,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    },
    Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    },
];

fn colour_grid<'a, F>(colours: &'a [Color], on_press: F) -> Element<'a, Message>
where
    F: 'a + Fn(Color) -> Message + Clone,
{
    Container::new(
        colours
            .iter()
            .fold(Wrap::new(), |wrap, colour| {
                wrap.push(
                    Button::new(vertical_space())
                        .width(32)
                        .height(32)
                        .on_press(on_press(*colour))
                        .style(move |theme, status| {
                            style::button::secondary(theme, status).with_background(*colour)
                        }),
                )
            })
            .spacing(12)
            .line_spacing(12),
    )
    .width(168)
    .height(168)
    .padding(Padding::new(24.0).right(12))
    .style(style::container::overlay)
    .into()
}
