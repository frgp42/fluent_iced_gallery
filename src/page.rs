pub mod app_bar_button;
pub mod button;
pub mod checkbox;
pub mod combo_box;
pub mod dialog;
pub mod drop_down_button;
pub mod image;
pub mod menu_bar;
pub mod radio;
pub mod ribbon;
pub mod row_column;
pub mod slider;
pub mod split_button;
pub mod svg;
pub mod text_input;
pub mod toggle_button;
pub mod toggler;

use crate::widget::{
    text, {Container, Element, Row},
};

use iced::{
    alignment::Vertical,
    border::Radius,
    widget::{column, container::Style, horizontal_space},
    Border, Length, Padding,
};

pub fn page<'a, Message: 'a>(
    title: &'a str,
    examples: impl IntoIterator<Item = Element<'a, Message>>,
) -> Element<'a, Message> {
    column![text::title2(title),]
        .spacing(32)
        .extend(examples)
        .into()
}

// Based on https://github.com/microsoft/WinUI-Gallery/blob/main/WinUIGallery/Controls/ControlExample.xaml
pub fn widget_example<'a, Message: 'a + Clone>(
    title: &'a str,
    example: impl Into<Element<'a, Message>>,
    output: Option<impl Into<Element<'a, Message>>>,
    options: Option<impl Into<Element<'a, Message>>>,
) -> Element<'a, Message> {
    let mut content = vec![example.into()];

    if output.is_some() || options.is_some() {
        content.push(horizontal_space().into());
    }

    if let Some(element) = output {
        content.push(element.into());
    }

    let padding = if options.is_some() { 2.0 } else { 24.0 };

    if let Some(element) = options {
        content.push(
            Container::new(element)
                .style(|theme| {
                    let palette = theme.palette();
                    Style::default()
                        .background(palette.card_background_fill_color_default)
                        .border(Border {
                            color: palette.divider_stroke_color_default,
                            radius: Radius::new(4).left(0),
                            width: 1.0,
                        })
                })
                .padding([24, 12])
                .into(),
        );
    }

    let presenter = Container::new(
        Row::from_vec(content)
            .width(Length::Fill)
            .spacing(24)
            .align_y(Vertical::Center),
    )
    .style(|theme| {
        let palette = theme.palette();
        Style::default()
            .background(palette.solid_background_fill_color_base)
            .border(Border {
                color: palette.card_stroke_color_default,
                radius: Radius::new(8.0),
                width: 1.0,
            })
    })
    .padding(Padding {
        left: 12.0,
        top: padding,
        right: padding,
        bottom: padding,
    })
    .width(Length::Fill);

    column![text::bold(title), presenter,].spacing(12).into()
}
