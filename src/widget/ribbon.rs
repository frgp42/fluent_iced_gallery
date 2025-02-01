use crate::{
    style,
    widget::{Container, Element, Quad},
};

use iced::Length;

pub fn separator<'a, Message: 'a>() -> Element<'a, Message> {
    // Container to add padding
    Container::new(
        Quad::new(None::<Element<Message>>)
            .width(1.0)
            .height(Length::Fill)
            .style(|theme, _status| {
                let palette = theme.palette();
                style::quad::Style {
                    background: Some(palette.divider_stroke_color_default.into()),
                    ..style::quad::Style::default()
                }
            }),
    )
    .padding([0, 4])
    .into()
}

pub mod button {

    use crate::{
        fluent_icon::FluentIcon,
        style,
        widget::{text, Button, Column, Row},
    };

    use iced::{
        alignment::{Horizontal, Vertical},
        widget::center,
        Length, Padding,
    };

    pub fn large<'a, Message>(label: &'a str, icon: FluentIcon) -> Button<'a, Message>
    where
        Message: 'a + Clone,
    {
        Button::new(center(
            Column::new()
                .push(
                    text::icon(icon.codepoint())
                        .height(Length::FillPortion(1))
                        .size(28),
                )
                .push(text::caption1(label).height(Length::FillPortion(1)))
                .align_x(Horizontal::Center),
        ))
        .width(60)
        .height(72)
        .padding(0)
        .style(style::button::transparent)
    }

    pub fn medium<'a, Message>(label: &'a str, icon: FluentIcon) -> Button<'a, Message>
    where
        Message: 'a + Clone,
    {
        Button::new(
            Row::new()
                .push(text::icon(icon.codepoint()))
                .push(text::caption1(label))
                .height(Length::Fill)
                .align_y(Vertical::Center)
                .spacing(4),
        )
        .width(Length::Shrink)
        .height(24)
        .padding(Padding::new(0.0).left(4).right(8))
        .style(style::button::transparent)
    }

    // pub fn small<'a, Message>(icon: FluentIcon) -> Button<'a, Message>
    // where
    //     Message: 'a + Clone,
    // {
    //     Button::new(center(text::icon(icon.codepoint())))
    //         .width(24)
    //         .height(24)
    //         .padding(4)
    //         .style(style::button::transparent)
    // }
}

pub mod split_button {

    use crate::{
        fluent_icon::FluentIcon,
        theme,
        widget::{text, Button, Column, DropDown, Element, Quad},
    };

    use iced::{
        alignment::{Horizontal, Vertical},
        border::Radius,
        widget::center,
        Border, Length, Shadow,
    };

    pub fn large<'a, Message>(
        label: &'a str,
        icon: FluentIcon,
        overlay: impl Into<Element<'a, Message>>,
        on_press: Message,
        on_open: Message,
        on_dismiss: Message,
        expanded: bool,
    ) -> DropDown<'a, Message>
    where
        Message: 'a + Clone,
    {
        type ButtonStatus = iced::widget::button::Status;
        type ButtonStyle = iced::widget::button::Style;
        type QuadStatus = crate::style::quad::Status;
        type QuadStyle = crate::style::quad::Style;

        fn base_style(theme: &theme::Theme, status: ButtonStatus) -> ButtonStyle {
            let palette = theme.palette();
            let base = ButtonStyle {
                background: Some(palette.subtle_fill_color_transparent.into()),
                text_color: palette.text_fill_color_primary,
                border: Border {
                    color: palette.control_stroke_color_default,
                    radius: Radius::new(4),
                    width: 1.0,
                },
                shadow: Shadow::default(),
            };

            match status {
                ButtonStatus::Active => ButtonStyle {
                    border: Border::default(),
                    ..base
                },
                ButtonStatus::Hovered => ButtonStyle {
                    background: Some(palette.subtle_fill_color_secondary.into()),
                    ..base
                },
                ButtonStatus::Pressed => ButtonStyle {
                    background: Some(palette.subtle_fill_color_tertiary.into()),
                    ..base
                },
                ButtonStatus::Disabled => ButtonStyle {
                    background: Some(palette.subtle_fill_color_disabled.into()),
                    text_color: palette.text_fill_color_disabled,
                    ..base
                },
            }
        }

        fn icon_button_style(theme: &theme::Theme, status: ButtonStatus) -> ButtonStyle {
            let base = base_style(theme, status);

            ButtonStyle {
                border: Border {
                    radius: base.border.radius.bottom(0),
                    ..base.border
                },
                ..base
            }
        }

        fn indicator_button_style(theme: &theme::Theme, status: ButtonStatus) -> ButtonStyle {
            let base = base_style(theme, status);

            ButtonStyle {
                border: Border {
                    radius: base.border.radius.top(0),
                    ..base.border
                },
                ..base
            }
        }

        fn quad_style(theme: &theme::Theme, status: QuadStatus) -> QuadStyle {
            let palette = theme.palette();
            let base = QuadStyle {
                background: None,
                border: Border::default(),
                shadow: Shadow::default(),
            };

            match status {
                QuadStatus::Active => base,
                QuadStatus::Hovered => QuadStyle {
                    border: Border {
                        color: palette.control_stroke_color_default,
                        radius: Radius::new(4),
                        width: 1.0,
                    },
                    ..base
                },
            }
        }

        let icon_button = Button::new(center(text::icon(icon.codepoint()).size(28)))
            .width(Length::Fill)
            .height(Length::FillPortion(1))
            .padding(0)
            .on_press(on_press)
            .style(icon_button_style);

        let indicator_button = Button::new(
            Column::new()
                .push(text::caption1(label))
                .push(
                    text::icon(FluentIcon::ChevronDown.codepoint())
                        .size(10)
                        .align_y(Vertical::Center),
                )
                .width(Length::Fill)
                .align_x(Horizontal::Center),
        )
        .width(Length::Fill)
        .height(Length::FillPortion(1))
        .padding(0)
        .on_press(on_open)
        .style(indicator_button_style);

        let underlay = Quad::new(Some(
            Column::new()
                .push(icon_button)
                .push(indicator_button)
                .width(Length::Fill)
                .height(Length::Fill),
        ))
        .width(40)
        .height(72)
        .style(quad_style);

        DropDown::new(underlay, overlay, expanded).on_dismiss(on_dismiss)
    }
}
