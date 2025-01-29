use crate::theme::Theme;

use iced::{
    widget::pick_list::{Catalog, Status, Style},
    Border
};

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> StyleFn<'a, Self> {
        Box::new(default)
    }

    fn style(&self, class: &StyleFn<'_, Self>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();

    let base = Style {
        text_color: palette.text_fill_color_primary,
        background: palette.control_fill_color_default.into(),
        placeholder_color: palette.text_fill_color_secondary,
        handle_color: palette.text_fill_color_secondary,
        border: Border {
            radius: 4.0.into(),
            width: 1.0,
            color: palette.control_stroke_color_default,
        },
    };

    match status {
        Status::Active => base,
        Status::Hovered => Style {
            background: palette.control_fill_color_secondary.into(),
            ..base
        },
        Status::Opened => base,
    }
}
