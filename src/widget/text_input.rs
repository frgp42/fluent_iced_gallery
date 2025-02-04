use crate::{
    font,
    widget::{
        underline::{ElementType, Underline},
        Element, TextInput,
    },
};

use iced::{widget::text::LineHeight, Padding, Pixels};

pub fn standard<'a, Message>(placeholder: &str, value: &str) -> TextInput<'a, Message>
where
    Message: 'a + Clone,
{
    TextInput::new(placeholder, value)
        .font(font::UI_REGULAR.clone().into())
        .line_height(LineHeight::Absolute(Pixels(20.0)))
        .size(14)
        .padding(Padding {
            left: 10.0,
            top: 6.0,
            right: 10.0,
            bottom: 6.0,
        })
}

pub fn underline<'a, Message>(text_input: TextInput<'a, Message>) -> Element<'a, Message>
where
    Message: 'a + Clone,
{
    let element = Element::new(text_input);
    let underline_type = ElementType::TextInput(element);
    Underline::new(underline_type).into()
}
