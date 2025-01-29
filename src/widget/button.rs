use crate::{
    fluent_icon::FluentIcon,
    style,
    widget::{text, Button, Column, DropDown, Element, Row},
};

use iced::{
    alignment::{Horizontal, Vertical},
    widget::{center, row},
    Length,
};

pub fn standard<'a, Message>(content: impl Into<Element<'a, Message>>) -> Button<'a, Message> {
    Button::new(content)
        .height(32)
        .style(style::button::secondary)
}

pub fn dropdown<'a, Message>(content: impl Into<Element<'a, Message>>) -> Button<'a, Message>
where
    Message: 'a + Clone,
{
    Button::new(
        Row::new()
            .push(content.into())
            .push(text::icon(FluentIcon::ChevronDown.codepoint()).size(8))
            .height(Length::Fill)
            .spacing(8)
            .align_y(Vertical::Center),
    )
    .height(32.0)
}

pub fn split<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    overlay: impl Into<Element<'a, Message>>,
    on_open: Message,
    on_dismiss: Message,
    expanded: bool,
) -> Element<'a, Message>
where
    Message: 'a + Clone,
{
    let indicator = Button::new(
        text::icon(FluentIcon::ChevronDown.codepoint())
            .size(10)
            .align_y(Vertical::Center),
    )
    .height(32)
    .on_press(on_open)
    .style(style::button::split_indicator);
    let underlay = Row::new().push(content).push(indicator);

    DropDown::new(underlay, overlay, expanded)
        .width(Length::Shrink)
        .on_dismiss(on_dismiss)
        .into()
}

fn menu_base<'a, Message>(content: impl Into<Element<'a, Message>>) -> Button<'a, Message>
where
    Message: 'a + Clone,
{
    Button::new(content)
        .width(Length::Fill)
        .height(28.0)
        .padding([0, 8])
        .style(style::button::flyout)
}

pub fn menu_labelled<'a, Message>(label: &'a str) -> Button<'a, Message>
where
    Message: 'a + Clone,
{
    menu_base(
        text::body1(label)
            .height(Length::Fill)
            .align_y(Vertical::Center),
    )
}

pub fn menu_icon<'a, Message>(label: &'a str, icon: FluentIcon) -> Button<'a, Message>
where
    Message: 'a + Clone,
{
    menu_base(
        row![text::icon(icon.codepoint()), text::body1(label)]
            .height(Length::Fill)
            .align_y(Vertical::Center)
            .spacing(12),
    )
}

pub fn app_bar<'a, Message>(label: &'a str, icon: FluentIcon) -> Button<Message>
where
    Message: 'a + Clone,
{
    Button::new(
        center(
            Column::new()
                .push(text::icon(icon.codepoint()))
                .push(text::body1(label))
                .spacing(-4.0)
                .align_x(Horizontal::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .width(64)
    .height(52)
    .style(style::button::transparent)
}
