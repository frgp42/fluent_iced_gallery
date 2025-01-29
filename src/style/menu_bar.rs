use crate::theme::Theme;

use iced::{border::Radius, Border, Color, Padding, Shadow, Vector};

use iced_aw::{
    menu::{Catalog, Style},
    style::Status,
};

type StyleFn<'a> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a>;

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
        bar_background: palette.subtle_fill_color_transparent.into(),
        bar_border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: Radius::new(4),
        },
        bar_shadow: Shadow::default(),
        bar_background_expand: Padding::default(),
        menu_background: palette.acrylic_in_app_fill_color_default_fallback.into(),
        menu_border: Border {
            color: palette.surface_stroke_color_flyout,
            width: 1.0,
            radius: Radius::new(8),
        },
        menu_shadow: Shadow {
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.28,
            },
            offset: Vector { x: 0.0, y: 4.0 },
            blur_radius: 8.0,
        },
        menu_background_expand: Padding::default(),
        path: Color::TRANSPARENT.into(),
        path_border: Border::default(),
    };

    let focussed = Style {
        bar_border: Border {
            color: palette.focus_stroke_color_outer,
            width: 2.0,
            radius: Radius::new(4),
        },
        ..base
    };

    match status {
        Status::Active => base,
        Status::Hovered => Style {
            bar_background: palette.subtle_fill_color_secondary.into(),
            ..base
        },
        Status::Pressed => Style {
            bar_background: palette.subtle_fill_color_tertiary.into(),
            ..base
        },
        Status::Disabled => Style {
            bar_background: palette.subtle_fill_color_disabled.into(),
            ..base
        },
        Status::Focused => focussed,
        Status::Selected => focussed,
    }
}
