use crate::{font, style, widget::Radio};

use iced::{widget::text::LineHeight, Pixels};

pub fn standard<'a, F, V, Message>(
    label: impl Into<String>,
    value: V,
    selected: Option<V>,
    f: F,
) -> Radio<'a, Message>
where
    V: Eq + Copy,
    F: FnOnce(V) -> Message,
    Message: Clone,
{
    iced::widget::Radio::new(label, value, selected, f)
        .size(20)
        .font(font::UI_REGULAR.clone())
        .text_size(14)
        .text_line_height(LineHeight::Absolute(Pixels(20.0)))
        .style(style::radio::default)
}
