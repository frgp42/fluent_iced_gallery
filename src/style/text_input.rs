use crate::theme::Theme;

use iced::{
    border::Radius,
    widget::text_input::{Catalog, Status, Style},
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

fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let base = Style {
        background: palette.control_fill_color_default.into(),
        border: Border {
            color: palette.control_stroke_color_default,
            width: 1.0,
            radius: Radius::new(4),
        },
        icon: palette.text_fill_color_primary,
        placeholder: palette.text_fill_color_secondary,
        selection: palette.accent_fill_color_default,
        value: palette.text_fill_color_primary,
    };

    match status {
        Status::Active => base,
        Status::Hovered => Style {
            background: palette.control_fill_color_secondary.into(),
            ..base
        },
        Status::Focused => Style {
            background: palette.control_fill_color_input_active.into(),
            ..base
        },
        Status::Disabled => Style {
            background: palette.control_fill_color_disabled.into(),
            placeholder: palette.text_fill_color_disabled,
            value: Color::from_rgba8(254, 254, 254, 0.36),
            ..base
        },
    }
}
