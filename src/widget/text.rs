use crate::{font, theme::Theme, widget::Text};

use iced::{advanced::text::LineHeight, widget::text, Pixels};

impl iced::widget::text::Catalog for Theme {
    type Class<'a> = Box<dyn Fn(&Theme) -> text::Style + 'a>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|_theme| text::Style::default())
    }

    fn style(&self, class: &Self::Class<'_>) -> text::Style {
        class(self)
    }
}

// Guidelines and naming from:
// https://fluent2.microsoft.design/typography

// Size and line heights from
// https://learn.microsoft.com/en-gb/windows/apps/design/signature-experiences/typography#type-ramp

pub fn caption1<'a>(fragment: impl text::IntoFragment<'a>) -> Text<'a> {
    Text::new(fragment)
        .font(font::SEGOE)
        .size(12)
        .line_height(LineHeight::Absolute(Pixels(16.0)))
}

pub fn body1<'a>(fragment: impl text::IntoFragment<'a>) -> Text<'a> {
    Text::new(fragment)
        .font(font::SEGOE)
        .size(14)
        .line_height(LineHeight::Absolute(Pixels(20.0)))
}

pub fn bold<'a>(fragment: impl text::IntoFragment<'a>) -> Text<'a> {
    body1(fragment).font(font::SEGOE_BOLD)
}

pub fn subtitle1<'a>(fragment: impl text::IntoFragment<'a>) -> Text<'a> {
    Text::new(fragment)
        .font(font::SEGOE_SEMIBOLD)
        .size(20)
        .line_height(LineHeight::Absolute(Pixels(26.0)))
}

pub fn title2<'a>(fragment: impl text::IntoFragment<'a>) -> Text<'a> {
    Text::new(fragment)
        .font(font::SEGOE_SEMIBOLD)
        .size(28)
        .line_height(LineHeight::Absolute(Pixels(36.0)))
}

pub fn icon<'a>(codepoint: char) -> Text<'a> {
    text(codepoint)
        .font(font::SEGOE_FLUENT_ICONS)
        .shaping(iced::widget::text::Shaping::Advanced)
}
