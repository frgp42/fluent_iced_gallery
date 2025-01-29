use iced::{
    advanced::{self, mouse},
    border::Radius,
    widget::canvas::{Frame, Geometry, Path, Program},
    Color, Point, Size,
};

pub struct Rectangle {
    width: f32,
    height: f32,
    radius: Radius,
    colour: Color,
}

impl Rectangle {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            radius: Radius::default(),
            colour: Color::default(),
        }
    }

    // pub fn radius(mut self, radius: impl Into<Radius>) -> Self {
    //     self.radius = radius.into();
    //     self
    // }

    pub fn colour(mut self, colour: impl Into<Color>) -> Self {
        self.colour = colour.into();
        self
    }
}

impl<Message, Theme, Renderer> Program<Message, Theme, Renderer> for Rectangle
where
    Renderer: advanced::graphics::geometry::Renderer,
{
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let mut frame = Frame::new(renderer, bounds.size());
        let rectangle = Path::rounded_rectangle(
            Point::ORIGIN,
            Size::new(self.width, self.height),
            self.radius,
        );
        frame.fill(&rectangle, self.colour);
        vec![frame.into_geometry()]
    }
}
