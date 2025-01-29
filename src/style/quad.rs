use crate::theme;

use iced::{Background, Border, Shadow};

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

pub enum Status {
    Active,
    Hovered,
}

#[derive(Default)]
pub struct Style {
    pub background: Option<Background>,
    pub border: Border,
    pub shadow: Shadow,
}

pub trait Catalog {
    type Class<'a>;

    fn default<'a>() -> Self::Class<'a>;

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

impl Catalog for theme::Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|_, _| Style::default())
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn separator(theme: &theme::Theme, _status: Status) -> Style {
    let palette = theme.palette();

    Style {
        background: Some(palette.divider_stroke_color_default.into()),
        border: Border::default(),
        shadow: Shadow::default(),
    }
}
