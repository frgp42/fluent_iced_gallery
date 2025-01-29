use crate::theme::Theme;

use iced::{widget::text_input::Status, Color};

pub struct Style {
    pub colour: Color,
}

pub trait Catalog {
    type Class<'a>;

    fn default<'a>() -> Self::Class<'a>;

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

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
        colour: palette.control_strong_stroke_color_default,
    };

    match status {
        Status::Active => base,
        Status::Hovered => base,
        Status::Focused => Style {
            colour: palette.accent_fill_color_default,
        },
        Status::Disabled => base,
    }
}
