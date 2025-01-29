use crate::{
    style,
    widget::{self, button, text, Column, Container, Element, Row},
};

use iced::{
    alignment::Horizontal,
    border::Radius,
    widget::{container, vertical_space},
    Border, Length,
};

pub fn content<'a, Message>(
    title: Option<&'a str>,
    content: impl Into<Element<'a, Message>>,
    primary_button: Option<Button<Message>>,
    secondary_button: Option<Button<Message>>,
    close_button: Button<Message>,
) -> Container<'a, Message>
where
    Message: 'a + Clone,
{
    let contents = Container::new(
        Column::new()
            .push_maybe(title.map(text::subtitle1))
            .push(vertical_space().height(12))
            .push(content),
    )
    .style(|theme| container::Style {
        background: Some(theme.palette().layer_fill_color_alt.into()),
        border: Border {
            radius: Radius::new(12).bottom(0),
            ..Border::default()
        },
        ..container::Style::default()
    })
    .width(Length::Fill)
    .padding(24);

    let buttons = Container::new(
        Row::new()
            .push_maybe(
                primary_button.map(move |button| button.view().style(style::button::primary)),
            )
            .push_maybe(secondary_button.map(Button::view))
            .push(close_button.view())
            .spacing(8),
    )
    .height(80)
    .padding(24);

    Container::new(
        Column::new()
            .push(contents)
            .push(vertical_space())
            .push(buttons),
    )
    .width(328)
    .height(220)
    .style(style::container::dialog)
    // Clipping not working
    // https://github.com/iced-rs/iced/issues/2093
    .clip(true)
}

pub struct Button<Message>
where
    Message: Clone,
{
    pub text: String,
    pub on_press: Message,
}

impl<'a, Message> Button<Message>
where
    Message: Clone,
{
    fn view(self) -> widget::Button<'a, Message> {
        button::standard(text::body1(self.text).align_x(Horizontal::Center))
            .on_press(self.on_press.clone())
            .width(Length::Fill)
    }
}
