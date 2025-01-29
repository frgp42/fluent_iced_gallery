use iced::{
    application, color,
    Color,
};

#[derive(Clone, Default, Debug, PartialEq)]
pub enum Theme {
    Light,
    #[default]
    Dark,
}

impl Theme {
    pub fn palette(&self) -> &Palette {
        match self {
            Theme::Dark => &Palette::DARK,
            Theme::Light => &Palette::LIGHT,
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{:?}", self)
    }
}

impl application::DefaultStyle for Theme {
    fn default_style(&self) -> application::Appearance {
        let palette = self.palette();

        application::Appearance {
            background_color: palette.solid_background_fill_color_base,
            text_color: palette.text_fill_color_primary,
        }
    }
}

macro_rules! from_argb {
    ($hex:expr) => {{
        let hex = $hex as u32;

        let a = ((hex & 0xff000000) >> 24) as f32 / 255.0;
        let r = (hex & 0x00ff0000) >> 16;
        let g = (hex & 0x0000ff00) >> 8;
        let b = (hex & 0x000000ff);

        color!(r, g, b, a)
    }};
}


// Requires "web-colors" feature to better match WinUI 3 colours
// https://github.com/iced-rs/iced/pull/1888
// There are still some slight colour variation but the biggest issue is text renderering
// https://github.com/iced-rs/iced/issues/2254

// Windows theme colour stored in HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Accent

pub struct Palette {
    // Accent colours are hard-coded as Iced does not provide system theme colours (yet).
    // https://github.com/microsoft/microsoft-ui-xaml/blob/63671e055eadfd74806f9800382b8bd6c9999b4a/dxaml/xcp/components/theminginterop/SystemThemingInterop.cpp#L226
    pub accent_fill_color_default: Color,
    pub accent_fill_color_secondary: Color,
    pub accent_fill_color_tertiary: Color,

    // Acrylic effects are not implemented so use fallback colours:
    // https://github.com/microsoft/microsoft-ui-xaml/blob/4c50e610e537aca92afc950c4be1ffb60c2f99d5/dev/Materials/Acrylic/AcrylicBrush_rs3_themeresources.xaml
    pub acrylic_in_app_fill_color_default_fallback: Color,

    // Colors from WinUI3 source code:
    // https://github.com/microsoft/microsoft-ui-xaml/blob/winui3/release/1.5-stable/controls/dev/CommonStyles/Common_themeresources_any.xaml

    pub text_fill_color_primary: Color,
    pub text_fill_color_secondary: Color,
    // pub text_fill_color_tertiary: Color,
    pub text_fill_color_disabled: Color,
    // pub text_fill_color_inverse: Color,
    // pub accent_text_fill_color_disabled: Color,
    // pub text_on_accent_fill_color_selected_text: Color,
    pub text_on_accent_fill_color_primary: Color,
    pub text_on_accent_fill_color_secondary: Color,
    pub text_on_accent_fill_color_disabled: Color,
    pub control_fill_color_default: Color,
    pub control_fill_color_secondary: Color,
    pub control_fill_color_tertiary: Color,
    pub control_fill_color_disabled: Color,
    pub control_fill_color_transparent: Color,
    pub control_fill_color_input_active: Color,
    pub control_strong_fill_color_default: Color,
    pub control_strong_fill_color_disabled: Color,
    pub control_solid_fill_color_default: Color,
    pub subtle_fill_color_transparent: Color,
    pub subtle_fill_color_secondary: Color,
    pub subtle_fill_color_tertiary: Color,
    pub subtle_fill_color_disabled: Color,
    // pub control_alt_fill_color_transparent: Color,
    pub control_alt_fill_color_secondary: Color,
    pub control_alt_fill_color_tertiary: Color,
    // pub control_alt_fill_color_quarternary: Color,
    pub control_alt_fill_color_disabled: Color,
    // pub control_on_image_fill_color_default: Color,
    // pub control_on_image_fill_color_secondary: Color,
    // pub control_on_image_fill_color_tertiary: Color,
    // pub control_on_image_fill_color_disabled: Color,
    pub accent_fill_color_disabled: Color,
    pub control_stroke_color_default: Color,
    pub control_stroke_color_secondary: Color,
    // pub control_stroke_color_on_accent_default: Color,
    pub control_stroke_color_on_accent_secondary: Color,
    // pub control_stroke_color_on_accent_tertiary: Color,
    // pub control_stroke_color_on_accent_disabled: Color,
    // pub control_stroke_color_for_strong_fill_when_on_image: Color,
    pub card_stroke_color_default: Color,
    // pub card_stroke_color_default_solid: Color,
    pub control_strong_stroke_color_default: Color,
    pub control_strong_stroke_color_disabled: Color,
    pub surface_stroke_color_default: Color,
    pub surface_stroke_color_flyout: Color,
    // pub surface_stroke_color_inverse: Color,
    pub divider_stroke_color_default: Color,
    pub focus_stroke_color_outer: Color,
    // pub focus_stroke_color_inner: Color,
    pub card_background_fill_color_default: Color,
    // pub card_background_fill_color_secondary: Color,
    pub smoke_fill_color_default: Color,
    // pub layer_fill_color_default: Color,
    pub layer_fill_color_alt: Color,
    // pub layer_on_acrylic_fill_color_default: Color,
    // pub layer_on_accent_acrylic_fill_color_default: Color,
    // pub layer_on_mica_base_alt_fill_color_default: Color,
    // pub layer_on_mica_base_alt_fill_color_secondary: Color,
    // pub layer_on_mica_base_alt_fill_color_tertiary: Color,
    // pub layer_on_mica_base_alt_fill_color_transparent: Color,
    pub solid_background_fill_color_base: Color,
    // pub solid_background_fill_color_secondary: Color,
    pub solid_background_fill_color_tertiary: Color,
    pub solid_background_fill_color_quarternary: Color,
    // pub solid_background_fill_color_transparent: Color,
    // pub solid_background_fill_color_base_alt: Color,
    // pub system_fill_color_success: Color,
    // pub system_fill_color_caution: Color,
    // pub system_fill_color_critical: Color,
    // pub system_fill_color_neutral: Color,
    // pub system_fill_color_solid_neutral: Color,
    // pub system_fill_color_attention_background: Color,
    // pub system_fill_color_success_background: Color,
    // pub system_fill_color_caution_background: Color,
    // pub system_fill_color_critical_background: Color,
    // pub system_fill_color_neutral_background: Color,
    // pub system_fill_color_solid_attention_background: Color,
    // pub system_fill_color_solid_neutral_background: Color,
}

impl Palette {
    pub const DARK: Self = Self {
        accent_fill_color_default: from_argb!(0xFF76B9ED),
        accent_fill_color_secondary: from_argb!(0xE676B9ED),
        accent_fill_color_tertiary: from_argb!(0xCC76B9ED),

        acrylic_in_app_fill_color_default_fallback: color!(0x2C2C2C),

        text_fill_color_primary: color!(0xFFFFFF),
        text_fill_color_secondary: from_argb!(0xC5FFFFFF),
        // text_fill_color_tertiary: from_argb!(0x87FFFFFF),
        text_fill_color_disabled: from_argb!(0x5DFFFFFF),
        // text_fill_color_inverse: from_argb!(0xE4000000),
        // accent_text_fill_color_disabled: from_argb!(0x5DFFFFFF),
        // text_on_accent_fill_color_selected_text: color!(0xFFFFFF),
        text_on_accent_fill_color_primary: color!(0x000000),
        text_on_accent_fill_color_secondary: from_argb!(0x80000000),
        text_on_accent_fill_color_disabled: from_argb!(0x87FFFFFF),
        control_fill_color_default: from_argb!(0x0FFFFFFF),
        control_fill_color_secondary: from_argb!(0x15FFFFFF),
        control_fill_color_tertiary: from_argb!(0x0BFFFFFF),
        control_fill_color_disabled: from_argb!(0x0BFFFFFF),
        control_fill_color_transparent: from_argb!(0x00FFFFFF),
        control_fill_color_input_active: from_argb!(0xB31E1E1E),
        control_strong_fill_color_default: from_argb!(0x8BFFFFFF),
        control_strong_fill_color_disabled: from_argb!(0x3FFFFFFF),
        control_solid_fill_color_default: color!(0x454545),
        subtle_fill_color_transparent: from_argb!(0x00FFFFFF),
        subtle_fill_color_secondary: from_argb!(0x0FFFFFFF),
        subtle_fill_color_tertiary: from_argb!(0x0AFFFFFF),
        subtle_fill_color_disabled: from_argb!(0x00FFFFFF),
        // control_alt_fill_color_transparent: from_argb!(0x00FFFFFF),
        control_alt_fill_color_secondary: from_argb!(0x19000000),
        control_alt_fill_color_tertiary: from_argb!(0x0BFFFFFF),
        // control_alt_fill_color_quarternary: from_argb!(0x12FFFFFF),
        control_alt_fill_color_disabled: from_argb!(0x00FFFFFF),
        // control_on_image_fill_color_default: from_argb!(0xB31C1C1C),
        // control_on_image_fill_color_secondary: color!(0x1A1A1A),
        // control_on_image_fill_color_tertiary: color!(0x131313),
        // control_on_image_fill_color_disabled: color!(0x1E1E1E),
        accent_fill_color_disabled: from_argb!(0x28FFFFFF),
        control_stroke_color_default: from_argb!(0x12FFFFFF),
        control_stroke_color_secondary: from_argb!(0x18FFFFFF),
        // control_stroke_color_on_accent_default: from_argb!(0x14FFFFFF),
        control_stroke_color_on_accent_secondary: from_argb!(0x23000000),
        // control_stroke_color_on_accent_tertiary: from_argb!(0x37000000),
        // control_stroke_color_on_accent_disabled: from_argb!(0x33000000),
        // control_stroke_color_for_strong_fill_when_on_image: from_argb!(0x6B000000),
        card_stroke_color_default: from_argb!(0x19000000),
        // card_stroke_color_default_solid: color!(0x1C1C1C),
        control_strong_stroke_color_default: from_argb!(0x8BFFFFFF),
        control_strong_stroke_color_disabled: from_argb!(0x28FFFFFF),
        surface_stroke_color_default: from_argb!(0x66757575),
        surface_stroke_color_flyout: from_argb!(0x33000000),
        // surface_stroke_color_inverse: from_argb!(0x0F000000),
        divider_stroke_color_default: from_argb!(0x15FFFFFF),
        focus_stroke_color_outer: color!(0xFFFFFF),
        // focus_stroke_color_inner: from_argb!(0xB3000000),
        card_background_fill_color_default: from_argb!(0x0DFFFFFF),
        // card_background_fill_color_secondary: from_argb!(0x08FFFFFF),
        smoke_fill_color_default: from_argb!(0x4D000000),
        // layer_fill_color_default: from_argb!(0x4C3A3A3A),
        layer_fill_color_alt: from_argb!(0x0DFFFFFF),
        // layer_on_acrylic_fill_color_default: from_argb!(0x09FFFFFF),
        // layer_on_accent_acrylic_fill_color_default: from_argb!(0x09FFFFFF),
        // layer_on_mica_base_alt_fill_color_default: from_argb!(0x733A3A3A),
        // layer_on_mica_base_alt_fill_color_secondary: from_argb!(0x0FFFFFFF),
        // layer_on_mica_base_alt_fill_color_tertiary: color!(0x2C2C2C),
        // layer_on_mica_base_alt_fill_color_transparent: from_argb!(0x00FFFFFF),
        solid_background_fill_color_base: color!(0x202020),
        // solid_background_fill_color_secondary: color!(0x1C1C1C),
        solid_background_fill_color_tertiary: color!(0x282828),
        solid_background_fill_color_quarternary: color!(0x2C2C2C),
        // solid_background_fill_color_transparent: from_argb!(0x00202020),
        // solid_background_fill_color_base_alt: color!(0x0A0A0A),
        // system_fill_color_success: color!(0x6CCB5F),
        // system_fill_color_caution: color!(0xFCE100),
        // system_fill_color_critical: color!(0xFF99A4),
        // system_fill_color_neutral: from_argb!(0x8BFFFFFF),
        // system_fill_color_solid_neutral: color!(0x9D9D9D),
        // system_fill_color_attention_background: from_argb!(0x08FFFFFF),
        // system_fill_color_success_background: color!(0x393D1B),
        // system_fill_color_caution_background: color!(0x433519),
        // system_fill_color_critical_background: color!(0x442726),
        // system_fill_color_neutral_background: from_argb!(0x08FFFFFF),
        // system_fill_color_solid_attention_background: color!(0x2E2E2E),
        // system_fill_color_solid_neutral_background: color!(0x2E2E2E),
    };

    pub const LIGHT: Self = Self {
        accent_fill_color_default: from_argb!(0xFF005A9E),
        accent_fill_color_secondary: from_argb!(0xE6005A9E),
        accent_fill_color_tertiary: from_argb!(0xCC005A9E),

        acrylic_in_app_fill_color_default_fallback: color!(0xF9F9F9),

        text_fill_color_primary: from_argb!(0xE4000000),
        text_fill_color_secondary: from_argb!(0x9E000000),
        // text_fill_color_tertiary: from_argb!(0x72000000),
        text_fill_color_disabled: from_argb!(0x5C000000),
        // text_fill_color_inverse: color!(0xFFFFFF),
        // accent_text_fill_color_disabled: from_argb!(0x5C000000),
        // text_on_accent_fill_color_selected_text: color!(0xFFFFFF),
        text_on_accent_fill_color_primary: color!(0xFFFFFF),
        text_on_accent_fill_color_secondary: from_argb!(0xB3FFFFFF),
        text_on_accent_fill_color_disabled: color!(0xFFFFFF),
        control_fill_color_default: from_argb!(0xB3FFFFFF),
        control_fill_color_secondary: from_argb!(0x80F9F9F9),
        control_fill_color_tertiary: from_argb!(0x4DF9F9F9),
        control_fill_color_disabled: from_argb!(0x4DF9F9F9),
        control_fill_color_transparent: from_argb!(0x00FFFFFF),
        control_fill_color_input_active: from_argb!(0xFFFFFF),
        control_strong_fill_color_default: from_argb!(0x72000000),
        control_strong_fill_color_disabled: from_argb!(0x51000000),
        control_solid_fill_color_default: color!(0xFFFFFF),
        subtle_fill_color_transparent: from_argb!(0x00FFFFFF),
        subtle_fill_color_secondary: from_argb!(0x09000000),
        subtle_fill_color_tertiary: from_argb!(0x06000000),
        subtle_fill_color_disabled: from_argb!(0x00FFFFFF),
        // control_alt_fill_color_transparent: from_argb!(0x00FFFFFF),
        control_alt_fill_color_secondary: from_argb!(0x06000000),
        control_alt_fill_color_tertiary: from_argb!(0x0F000000),
        // control_alt_fill_color_quarternary: from_argb!(0x18000000),
        control_alt_fill_color_disabled: from_argb!(0x00FFFFFF),
        // control_on_image_fill_color_default: from_argb!(0xC9FFFFFF),
        // control_on_image_fill_color_secondary: color!(0xF3F3F3),
        // control_on_image_fill_color_tertiary: color!(0xEBEBEB),
        // control_on_image_fill_color_disabled: from_argb!(0x00FFFFFF),
        accent_fill_color_disabled: from_argb!(0x37000000),
        control_stroke_color_default: from_argb!(0x0F000000),
        control_stroke_color_secondary: from_argb!(0x29000000),
        // control_stroke_color_on_accent_default: from_argb!(0x14FFFFFF),
        control_stroke_color_on_accent_secondary: from_argb!(0x66000000),
        // control_stroke_color_on_accent_tertiary: from_argb!(0x37000000),
        // control_stroke_color_on_accent_disabled: from_argb!(0x0F000000),
        // control_stroke_color_for_strong_fill_when_on_image: from_argb!(0x59FFFFFF),
        card_stroke_color_default: from_argb!(0x0F000000),
        // card_stroke_color_default_solid: color!(0xEBEBEB),
        control_strong_stroke_color_default: from_argb!(0x72000000),
        control_strong_stroke_color_disabled: from_argb!(0x37000000),
        surface_stroke_color_default: from_argb!(0x66757575),
        surface_stroke_color_flyout: from_argb!(0x0F000000),
        // surface_stroke_color_inverse: from_argb!(0x15FFFFFF),
        divider_stroke_color_default: from_argb!(0x0F000000),
        focus_stroke_color_outer: from_argb!(0xE4000000),
        // focus_stroke_color_inner: from_argb!(0xB3FFFFFF),
        card_background_fill_color_default: from_argb!(0xB3FFFFFF),
        // card_background_fill_color_secondary: from_argb!(0x80F6F6F6),
        smoke_fill_color_default: from_argb!(0x4D000000),
        // layer_fill_color_default: from_argb!(0x80FFFFFF),
        layer_fill_color_alt: color!(0xFFFFFF),
        // layer_on_acrylic_fill_color_default: from_argb!(0x40FFFFFF),
        // layer_on_accent_acrylic_fill_color_default: from_argb!(0x40FFFFFF),
        // layer_on_mica_base_alt_fill_color_default: from_argb!(0xB3FFFFFF),
        // layer_on_mica_base_alt_fill_color_secondary: from_argb!(0x0A000000),
        // layer_on_mica_base_alt_fill_color_tertiary: color!(0xF9F9F9),
        // layer_on_mica_base_alt_fill_color_transparent: from_argb!(0x00000000),
        solid_background_fill_color_base: color!(0xF3F3F3),
        // solid_background_fill_color_secondary: color!(0xEEEEEE),
        solid_background_fill_color_tertiary: color!(0xF9F9F9),
        solid_background_fill_color_quarternary: color!(0xFFFFFF),
        // solid_background_fill_color_transparent: from_argb!(0x00F3F3F3),
        // solid_background_fill_color_base_alt: color!(0xDADADA),
        // system_fill_color_success: color!(0x0F7B0F),
        // system_fill_color_caution: color!(0x9D5D00),
        // system_fill_color_critical: color!(0xC42B1C),
        // system_fill_color_neutral: from_argb!(0x72000000),
        // system_fill_color_solid_neutral: color!(0x8A8A8A),
        // system_fill_color_attention_background: from_argb!(0x80F6F6F6),
        // system_fill_color_success_background: color!(0xDFF6DD),
        // system_fill_color_caution_background: color!(0xFFF4CE),
        // system_fill_color_critical_background: color!(0xFDE7E9),
        // system_fill_color_neutral_background: from_argb!(0x06000000),
        // system_fill_color_solid_attention_background: color!(0xF7F7F7),
        // system_fill_color_solid_neutral_background: color!(0xF3F3F3),
    };
}
