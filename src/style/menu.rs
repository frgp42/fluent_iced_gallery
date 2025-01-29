use crate::theme::Theme;

use iced::{
    widget::overlay::menu::{Catalog, Style},
    Border
};

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> StyleFn<'a, Self> {
        Box::new(default)
    }

    fn style(&self, class: &StyleFn<'_, Self>) -> Style {
        class(self)
    }
}

pub fn default(theme: &Theme) -> Style {
    let palette = theme.palette();
    
    Style {
        background: palette.acrylic_in_app_fill_color_default_fallback.into(),
        border: Border {
            color: palette.surface_stroke_color_flyout,
            width: 1.0,
            radius: 4.0.into(),
        },
        text_color: palette.text_fill_color_primary,
        selected_background: palette.subtle_fill_color_secondary.into(),
        selected_text_color: palette.text_fill_color_primary,
    }
}