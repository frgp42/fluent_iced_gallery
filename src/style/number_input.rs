use crate::theme::Theme;

use iced_aw::{
    number_input::{Catalog, Style},
    style::{number_input::ExtendedCatalog, Status, StyleFn},
};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self, Style>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

impl ExtendedCatalog for Theme {
    fn style(&self, class: &<Self as self::Catalog>::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let base = Style {
        button_background: Some(palette.control_fill_color_transparent.into()),
        icon_color: palette.text_fill_color_secondary,
    };

    match status {
        Status::Active => base,
        Status::Hovered => Style {
            button_background: Some(palette.subtle_fill_color_secondary.into()),
            ..base
        },
        Status::Pressed => Style {
            button_background: Some(palette.subtle_fill_color_tertiary.into()),
            ..base
        },
        Status::Disabled => base,
        Status::Focused => base,
        Status::Selected => base,
    }
}
