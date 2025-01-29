use crate::{font, widget::Toggler};

use iced::{widget::text::LineHeight, Pixels};

pub fn standard<'a, Message>(is_toggled: bool) -> Toggler<'a, Message> {
    Toggler::new(is_toggled)
        .size(20)
        .font(font::SEGOE)
        .text_size(14)
        .text_line_height(LineHeight::Absolute(Pixels(20.0)))
        .spacing(12)
}
