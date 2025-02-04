use crate::{fluent_icon::FluentIcon, font, widget::ComboBox};

use iced::{
    widget::{
        combo_box::State,
        text::LineHeight,
        text_input::{Icon, Side},
    },
    Pixels,
};

pub fn standard<'a, T, Message>(
    state: &'a State<T>,
    placeholder: &str,
    selection: Option<&T>,
    on_selected: impl Fn(T) -> Message + 'static,
) -> ComboBox<'a, T, Message>
where
    T: std::fmt::Display + Clone,
{
    ComboBox::new(state, placeholder, selection, on_selected)
        .font(font::UI_REGULAR.clone().into())
        .line_height(LineHeight::Absolute(Pixels(20.0)))
        .icon(Icon {
            font: iced::Font::from(font::FLUENT_ICONS.clone()),
            code_point: FluentIcon::ChevronDown.codepoint(),
            size: Some(10.into()),
            spacing: 4.0,
            side: Side::Right,
        })
}
