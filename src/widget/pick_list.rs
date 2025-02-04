use crate::{fluent_icon::FluentIcon, font, widget::PickList};

use iced::{
    widget::{
        pick_list::{Handle, Icon},
        text::LineHeight,
    },
    Pixels,
};

use std::borrow::Borrow;

pub fn standard<'a, T, L, V, Message>(
    options: L,
    selected: Option<V>,
    on_select: impl Fn(T) -> Message + 'a,
) -> PickList<'a, T, L, V, Message>
where
    T: ToString + PartialEq + Clone,
    L: 'a + Borrow<[T]>,
    V: 'a + Borrow<T>,
    Message: Clone,
{
    PickList::new(options, selected, on_select)
        .font(font::UI_REGULAR.clone())
        .text_size(14)
        .text_line_height(LineHeight::Absolute(Pixels(20.0)))
        .handle(Handle::Static(Icon {
            font: iced::Font::from(font::FLUENT_ICONS.clone()),
            code_point: FluentIcon::ChevronDown.codepoint(),
            size: Some(10.into()),
            line_height: LineHeight::default(),
            shaping: iced::widget::text::Shaping::Advanced,
        }))
}
