use crate::{
    style,
    widget::{Element, Scrollable},
};

use iced::widget::scrollable::Direction;

// The Iced Scrollable is unable to be expanded (at least easily) when hovered over
// like the WinUI ScrollView so this feature is ignored for the moment
pub fn standard<'a, Message>(content: impl Into<Element<'a, Message>>) -> Scrollable<'a, Message> {
    Scrollable::with_direction(content, Direction::Vertical(scrollbar::standard()))
        .style(style::scrollable::default)
}

pub mod scrollbar {
    use iced::widget::scrollable::Scrollbar;

    pub fn standard() -> Scrollbar {
        Scrollbar::new().width(12).scroller_width(6)
    }
}
