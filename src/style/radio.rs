use crate::theme::Theme;

use iced::widget::radio::{Catalog, Status, Style};

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
    let base = Style {
        background: palette.control_alt_fill_color_secondary.into(),
        dot_color: palette.text_on_accent_fill_color_primary,
        border_width: 1.0,
        border_color: palette.control_strong_stroke_color_default,
        text_color: Some(palette.text_fill_color_primary),
    };

    match status {
        Status::Active { is_selected } => {
            if is_selected {
                Style {
                    background: palette.accent_fill_color_default.into(),
                    border_color: palette.accent_fill_color_default,
                    ..base
                }
            } else {
                base
            }
        }
        Status::Hovered { is_selected } => {
            if is_selected {
                Style {
                    background: palette.accent_fill_color_secondary.into(),
                    border_color: palette.accent_fill_color_secondary,
                    ..base
                }
            } else {
                Style {
                    background: palette.control_alt_fill_color_tertiary.into(),
                    ..base
                }
            }
        }
    }
}
