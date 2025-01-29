use crate::theme::Theme;

use iced::widget::button::{self, Status, Style, StyleFn};
use iced::Color;
use iced::{border::Radius, Border, Shadow};

impl button::Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(secondary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let base = Style {
        background: Some(palette.accent_fill_color_default.into()),
        text_color: palette.text_on_accent_fill_color_primary,
        border: Border {
            color: palette.control_stroke_color_on_accent_secondary,
            radius: Radius::new(4),
            width: 1.0,
        },
        shadow: Shadow::default(),
    };

    match status {
        Status::Active => base,
        Status::Hovered => base.with_background(palette.accent_fill_color_secondary),
        Status::Pressed => Style {
            background: Some(palette.accent_fill_color_tertiary.into()),
            text_color: palette.text_on_accent_fill_color_secondary,
            border: base.border.color(palette.control_fill_color_transparent),
            ..base
        },
        Status::Disabled => Style {
            background: Some(palette.accent_fill_color_disabled.into()),
            text_color: palette.text_on_accent_fill_color_disabled,
            border: base.border.color(palette.control_fill_color_transparent),
            ..base
        },
    }
}

pub fn secondary(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let base = Style {
        background: Some(palette.control_fill_color_default.into()),
        text_color: palette.text_fill_color_primary,
        border: Border {
            color: palette.control_stroke_color_default,
            radius: Radius::new(4),
            width: 1.0,
        },
        shadow: Shadow::default(),
    };

    match status {
        Status::Active => base,
        Status::Hovered => base.with_background(palette.control_fill_color_secondary),
        Status::Pressed => Style {
            background: Some(palette.control_fill_color_tertiary.into()),
            text_color: palette.text_fill_color_secondary,
            ..base
        },
        Status::Disabled => Style {
            background: Some(palette.control_fill_color_disabled.into()),
            text_color: palette.text_fill_color_disabled,
            ..base
        },
    }
}

pub fn split_content(theme: &Theme, status: Status) -> Style {
    let base = secondary(theme, status);

    Style {
        border: Border {
            radius: base.border.radius.right(0),
            ..base.border
        },
        ..base
    }
}

pub fn split_indicator(theme: &Theme, status: Status) -> Style {
    let base = secondary(theme, status);

    Style {
        border: Border {
            radius: base.border.radius.left(0),
            ..base.border
        },
        ..base
    }
}

pub fn transparent(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let base = Style {
        background: Some(palette.subtle_fill_color_transparent.into()),
        text_color: palette.text_fill_color_primary,
        border: Border {
            color: Color::TRANSPARENT,
            radius: Radius::new(4),
            width: 0.0,
        },
        shadow: Shadow::default(),
    };

    match status {
        Status::Active => base,
        Status::Hovered => Style {
            background: Some(palette.subtle_fill_color_secondary.into()),
            ..base
        },
        Status::Pressed => Style {
            background: Some(palette.subtle_fill_color_tertiary.into()),
            ..base
        },
        Status::Disabled => Style {
            background: Some(palette.subtle_fill_color_disabled.into()),
            text_color: palette.text_fill_color_disabled,
            ..base
        },
    }
}

pub fn flyout(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let base = secondary(theme, status);

    match status {
        Status::Active => Style {
            background: Some(palette.control_fill_color_transparent.into()),
            text_color: palette.text_fill_color_primary,
            ..Style::default()
        },
        _ => base,
    }
}

pub fn menu_item(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let base = Style {
        background: Some(palette.subtle_fill_color_transparent.into()),
        text_color: palette.text_fill_color_primary,
        border: Border {
            color: Color::TRANSPARENT,
            radius: Radius::new(4),
            width: 0.0,
        },
        shadow: Shadow::default(),
    };

    match status {
        Status::Active => base,
        Status::Hovered => Style {
            background: Some(palette.subtle_fill_color_secondary.into()),
            ..base
        },
        Status::Pressed => Style {
            background: Some(palette.subtle_fill_color_tertiary.into()),
            ..base
        },
        Status::Disabled => Style {
            background: Some(palette.subtle_fill_color_disabled.into()),
            ..base
        },
    }
}

pub fn nav_item(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();

    let background = match status {
        Status::Active => None,
        Status::Hovered => Some(palette.subtle_fill_color_secondary.into()),
        Status::Pressed => Some(palette.subtle_fill_color_tertiary.into()),
        Status::Disabled => None,
    };

    let text_color = match status {
        Status::Active => palette.text_fill_color_primary,
        Status::Hovered => palette.text_fill_color_primary,
        Status::Pressed => palette.text_fill_color_secondary,
        Status::Disabled => palette.text_fill_color_disabled,
    };

    Style {
        background,
        text_color,
        border: Border::default().rounded(4),
        shadow: Shadow::default(),
    }
}
