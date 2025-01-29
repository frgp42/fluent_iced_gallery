use crate::theme::Theme;

use iced::{
    border::Radius,
    widget::checkbox::{Catalog, Status, Style},
    Border,
};

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();

    match status {
        Status::Active { is_checked } => Style {
            background: if is_checked {
                palette.accent_fill_color_default.into()
            } else {
                palette.control_alt_fill_color_secondary.into()
            },
            icon_color: palette.text_on_accent_fill_color_primary,
            border: Border {
                color: if is_checked {
                    palette.accent_fill_color_default
                } else {
                    palette.control_strong_stroke_color_default
                },
                radius: Radius::new(4),
                width: 1.0,
            },
            text_color: Some(palette.text_fill_color_primary),
        },
        Status::Hovered { is_checked } => Style {
            background: if is_checked {
                palette.accent_fill_color_secondary.into()
            } else {
                palette.control_alt_fill_color_tertiary.into()
            },
            icon_color: palette.text_on_accent_fill_color_primary,
            border: Border {
                color: if is_checked {
                    palette.accent_fill_color_secondary
                } else {
                    palette.control_strong_stroke_color_default
                },
                radius: Radius::new(2),
                width: 1.0,
            },
            text_color: Some(palette.text_fill_color_primary),
        },
        Status::Disabled { is_checked } => Style {
            background: if is_checked {
                palette.accent_fill_color_tertiary.into()
            } else {
                palette.control_alt_fill_color_disabled.into()
            },
            icon_color: palette.text_on_accent_fill_color_disabled,
            border: Border {
                color: palette.control_strong_stroke_color_disabled,
                radius: Radius::new(2),
                width: 1.0,
            },
            text_color: Some(palette.text_fill_color_disabled),
        },
    }
}
