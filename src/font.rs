use iced::{
    font::{Family, Weight},
    Font,
};

pub const SEGOE: Font = Font::with_name("en Segoe UI");

pub const SEGOE_SEMIBOLD: Font = Font {
    family: Family::Name("en Segoe UI Semibold"),
    weight: Weight::Semibold,
    ..Font::DEFAULT
};

pub const SEGOE_BOLD: Font = Font {
    family: Family::Name("en Segoe UI Bold"),
    weight: Weight::Bold,
    ..Font::DEFAULT
};

pub const SEGOE_FLUENT_ICONS: Font = Font::with_name("en Segoe Fluent Icons");
