use crate::{style::container, theme::Theme};

use iced::{
    border,
    widget::scrollable::{Catalog, Rail, Scroller, Status, Style},
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

// ScrollBar colours specified here:
// https://github.com/microsoft/microsoft-ui-xaml/blob/4c50e610e537aca92afc950c4be1ffb60c2f99d5/dev/CommonStyles/ScrollBar_themeresources.xaml

pub fn default(theme: &Theme, _status: Status) -> Style {
    let palette = theme.palette();

    let scrollbar = Rail {
        background: Some(palette.acrylic_in_app_fill_color_default_fallback.into()),
        border: border::rounded(8),
        scroller: Scroller {
            color: palette.control_strong_fill_color_default,
            border: border::rounded(8),
        },
    };

    Style {
        container: container::transparent(theme),
        vertical_rail: scrollbar,
        horizontal_rail: scrollbar,
        gap: None,
    }
}
