use crate::{
    page::{page, widget_example},
    widget::{canvas::Rectangle, radio, text, Canvas, Column, Element, Row},
};

use iced::{widget::column, Color, Length};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Orientation {
    Row,
    #[default]
    Column,
}

#[derive(Clone, Debug, Default)]
pub struct RowColumn {
    orientation: Orientation,
}

#[derive(Clone, Debug)]
pub enum Message {
    OrientationSelected(Orientation),
}

impl RowColumn {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::OrientationSelected(orientation) => self.orientation = orientation,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let squares = vec![
            Color::from_rgb(1.0, 0.0, 0.0),
            Color::from_rgb(0.0, 0.0, 1.0),
            Color::from_rgb(0.0, 0.5, 0.0),
            Color::from_rgb(1.0, 1.0, 0.0),
        ]
        .into_iter()
        .map(|colour| {
            Canvas::new(Rectangle::new(40.0, 40.0).colour(colour))
                .width(40)
                .height(40)
                .into()
        });

        page(
            "Row & Column",
            [widget_example(
                "Row and Column widgets.",
                match self.orientation {
                    Orientation::Row => Element::new(
                        Row::with_children(squares)
                            .height(180)
                            .spacing(0)
                            .padding([10, 0]),
                    ),
                    Orientation::Column => {
                        Element::new(Column::with_children(squares).padding([10, 0]))
                    }
                },
                None::<Element<Message>>,
                Some(
                    column![
                        text::body1("Orientation"),
                        radio::standard(
                            "Horizontal",
                            Orientation::Row,
                            Some(self.orientation),
                            Message::OrientationSelected,
                        ),
                        radio::standard(
                            "Vertical",
                            Orientation::Column,
                            Some(self.orientation),
                            Message::OrientationSelected,
                        ),
                    ]
                    .height(Length::Fill)
                    .spacing(16),
                ),
            )],
        )
    }
}
