use crate::theme::Theme;

use iced::{
    border::Radius,
    widget::{
        container,
        container::{Style, StyleFn},
    },
    Border, Color, Shadow, Vector,
};

impl container::Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(transparent)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn transparent(_theme: &Theme) -> Style {
    Style::default()
}

pub fn card(theme: &Theme) -> Style {
    let palette = theme.palette();

    Style::default()
        .background(palette.card_background_fill_color_default)
        .border(Border {
            color: palette.card_stroke_color_default,
            radius: Radius::new(4.0),
            width: 1.0,
        })
}

pub fn overlay(theme: &Theme) -> Style {
    let palette = theme.palette();

    Style {
        text_color: Some(palette.text_fill_color_primary),
        background: Some(palette.solid_background_fill_color_quarternary.into()),
        border: Border {
            color: palette.surface_stroke_color_flyout,
            width: 1.0,
            radius: Radius::new(8),
        },
        shadow: Shadow {
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.28,
            },
            offset: Vector { x: 0.0, y: 4.0 },
            blur_radius: 8.0,
        },
    }
}

pub fn dialog(theme: &Theme) -> Style {
    let palette = theme.palette();

    Style {
        text_color: Some(palette.text_fill_color_primary),
        background: Some(palette.solid_background_fill_color_base.into()),
        border: Border {
            color: palette.surface_stroke_color_default,
            width: 1.0,
            radius: Radius::new(12),
        },
        shadow: Shadow {
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.28,
            },
            offset: Vector { x: 0.0, y: 32.0 },
            blur_radius: 64.0,
        },
    }
}
