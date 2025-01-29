use crate::theme::Theme;

use iced::{
    widget::slider::{Catalog, Handle, HandleShape, Rail, Status, Style},
    Border, Color,
};

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();

    Style {
        rail: Rail {
            backgrounds: (
                match status {
                    Status::Active => palette.accent_fill_color_default.into(),
                    Status::Hovered => palette.accent_fill_color_secondary.into(),
                    Status::Dragged => palette.accent_fill_color_tertiary.into(),
                },
                palette.control_strong_fill_color_default.into(),
            ),
            width: 4.0,
            border: Border {
                radius: 2.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
        },
        handle: Handle {
            shape: HandleShape::Circle { radius: 11.0 },
            background: palette.accent_fill_color_default.into(),
            border_color: palette.control_solid_fill_color_default,
            border_width: match status {
                Status::Active => 5.0,
                Status::Hovered => 4.0,
                Status::Dragged => 6.0,
            },
        },
    }
}
