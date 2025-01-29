use crate::{
    page::{page, widget_example},
    widget::{
        self,
        number_input::{self, NumberInput},
        text, Element,
    },
};

use iced::{
    alignment,
    widget::{column, row},
};

#[derive(Clone, Debug)]
pub struct Slider {
    simple_value: u32,
    min: u32,
    max: u32,
    value: u32,
    step: u32,
}

impl Default for Slider {
    fn default() -> Self {
        Self {
            simple_value: 0,
            min: 500,
            max: 1000,
            value: 800,
            step: 10,
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Debug)]
pub enum Message {
    SimpleValueChanged(u32),
    MinChanged(u32),
    MaxChanged(u32),
    StepChanged(u32),
    ValueChanged(u32),
}

impl Slider {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SimpleValueChanged(value) => self.simple_value = value,
            Message::MinChanged(min) => {
                self.min = min;

                if min > self.value {
                    self.value = min
                }
            }
            Message::MaxChanged(max) => {
                self.max = max;

                if max < self.value {
                    self.value = max
                }
            }
            Message::StepChanged(step) => self.step = step,
            Message::ValueChanged(value) => self.value = value,
        }
    }

    pub fn view(&self) -> Element<Message> {
        page(
            "Slider",
            [
                widget_example(
                    "A simple Slider.",
                    widget::slider::standard(
                        0..=100,
                        self.simple_value,
                        Message::SimpleValueChanged,
                    ),
                    Some(text::body1(format!["Output: \n{}", self.simple_value])),
                    None::<Element<Message>>,
                ),
                widget_example(
                    "A Slider with range and steps specified.",
                    widget::slider::standard(
                        self.min..=self.max,
                        self.value,
                        Message::ValueChanged,
                    )
                    .step(self.step),
                    Some(text::body1(format!["Output: \n{}", self.value])),
                    Some(
                        column![
                            row![
                                text::body1("Minimum:"),
                                number_input::underline(NumberInput::new(
                                    self.min,
                                    ..=self.max,
                                    Message::MinChanged
                                ))
                            ]
                            .align_y(alignment::Vertical::Center)
                            .spacing(16),
                            row![
                                text::body1("Maximum:"),
                                number_input::underline(NumberInput::new(
                                    self.max,
                                    self.min..,
                                    Message::MaxChanged
                                ))
                            ]
                            .align_y(alignment::Vertical::Center)
                            .spacing(14),
                            row![
                                text::body1("Step:"),
                                number_input::underline(NumberInput::new(
                                    self.step,
                                    0..=(self.max - self.min),
                                    Message::StepChanged
                                ))
                            ]
                            .align_y(alignment::Vertical::Center)
                            .spacing(48),
                        ]
                        .spacing(8),
                    ),
                ),
            ],
        )
    }
}
