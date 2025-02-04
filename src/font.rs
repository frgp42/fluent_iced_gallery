use iced::font;
use std::borrow::Cow;
use std::sync::OnceLock;

// Define font candidates with their system paths and fallback assets
const FONT_CANDIDATES: &[(&str, &[(&str, &[&str])])] = &[
    (
        "UI",
        &[
            (
                "Segoe UI",
                &[
                    "C:\\Windows\\Fonts\\segoeui.ttf",
                    "C:\\Windows\\Fonts\\segoeuib.ttf",
                    "C:\\Windows\\Fonts\\segoeuisb.ttf",
                ],
            ),
            (
                "Selawik",
                &[
                    concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/selawk.ttf"),
                    concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/selawkb.ttf"),
                    concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/selawksb.ttf"),
                ],
            ),
        ],
    ),
    (
        "Icons",
        &[
            (
                "Segoe Fluent Icons",
                &["C:\\Windows\\Fonts\\SegoeFluentIcons.ttf"],
            ),
            (
                "Fluent System Icons",
                &[concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/assets/fonts/FluentSystemIcons-Regular.ttf"
                )],
            ),
        ],
    ),
];

pub static UI_REGULAR: Font = Font::new(Weight::Regular);
pub static UI_BOLD: Font = Font::new(Weight::Bold);
pub static UI_SEMIBOLD: Font = Font::new(Weight::Semibold);
pub static FLUENT_ICONS: Font = Font::new(Weight::Regular);

#[derive(Debug, Clone)]
pub struct Font {
    weight: Weight,
    inner: OnceLock<iced::Font>,
}

#[derive(Debug, Clone, Copy)]
pub enum Weight {
    Bold,
    Semibold,
    Regular,
}

impl Weight {
    fn to_iced(self) -> font::Weight {
        match self {
            Weight::Bold => font::Weight::Bold,
            Weight::Semibold => font::Weight::Semibold,
            Weight::Regular => font::Weight::Normal,
        }
    }
}

impl Font {
    const fn new(weight: Weight) -> Self {
        Self {
            weight,
            inner: OnceLock::new(),
        }
    }

    fn set(&self, name: String) {
        let name = Box::leak(name.into_boxed_str());

        let _ = self.inner.set(iced::Font {
            family: font::Family::Name(name),
            weight: self.weight.to_iced(),
            ..iced::Font::DEFAULT
        });
    }
}

impl From<Font> for iced::Font {
    fn from(value: Font) -> Self {
        value.inner.get().copied().expect("font is set on startup")
    }
}

fn load_font(path: &str) -> Option<Vec<u8>> {
    std::fs::read(path).ok()
}

// First step: Register font names before the application starts
pub fn set() {
    // Try to find and register the first available font for each type
    for &(font_type, candidates) in FONT_CANDIDATES {
        for &(name, paths) in candidates {
            // Check if at least the regular weight is available
            if let Some(_) = load_font(paths[0]) {
                match font_type {
                    "UI" => {
                        UI_REGULAR.set(name.to_string());
                        UI_BOLD.set(name.to_string());
                        UI_SEMIBOLD.set(name.to_string());
                    }
                    "Icons" => FLUENT_ICONS.set(name.to_string()),
                    _ => {}
                }
                break;
            }
        }
    }
}

// Second step: Provide the font data to iced in `iced::Settings`
pub fn load() -> Vec<Cow<'static, [u8]>> {
    vec![
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/fonts/selawk.ttf"
        ))
        .as_slice()
        .into(),
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/fonts/selawksb.ttf"
        ))
        .as_slice()
        .into(),
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/fonts/selawkb.ttf"
        ))
        .as_slice()
        .into(),
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/fonts/FluentSystemIcons-Regular.ttf"
        ))
        .as_slice()
        .into(),
    ]
}
