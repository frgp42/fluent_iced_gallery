use crate::theme::Theme;

use iced::{
    widget::toggler::{Catalog, Status, Style},
    Color,
};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

pub fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let base = Style {
        background: palette.control_alt_fill_color_secondary,
        background_border_width: 1.0,
        background_border_color: palette.control_strong_stroke_color_default,
        foreground: palette.text_fill_color_secondary,
        foreground_border_width: 0.0,
        foreground_border_color: Color::TRANSPARENT,
    };

    match status {
        Status::Active { is_toggled } => {
            if is_toggled {
                Style {
                    background: palette.accent_fill_color_default,
                    background_border_color: palette.accent_fill_color_default,
                    foreground: palette.text_on_accent_fill_color_primary,
                    ..base
                }
            } else {
                base
            }
        }
        Status::Hovered { is_toggled } => {
            if is_toggled {
                Style {
                    background: palette.accent_fill_color_secondary,
                    background_border_color: palette.accent_fill_color_secondary,
                    foreground: palette.text_on_accent_fill_color_secondary,
                    ..base
                }
            } else {
                Style {
                    background: palette.control_alt_fill_color_tertiary,
                    ..base
                }
            }
        }
        Status::Disabled => Style {
            background: palette.control_alt_fill_color_disabled,
            background_border_color: palette.control_strong_stroke_color_disabled,
            foreground: palette.text_fill_color_disabled,
            ..base
        },
    }
}
