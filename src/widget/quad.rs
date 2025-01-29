// A Quad widget exists in iced_aw but it can't be styled according to the theme
// and this one can wrap around another widget
use crate::{
    style::{
        self,
        quad::{Catalog, Status, Style, StyleFn},
    },
    theme,
};

use iced::{
    advanced::{
        self,
        layout::{self, Limits, Node},
        renderer,
        widget::{tree, Tree},
        Clipboard, Layout, Widget,
    },
    event,
    mouse::{self, Cursor},
    Color, Element, Event, Length, Rectangle, Size,
};

pub struct Quad<'a, Message, Theme = theme::Theme, Renderer = iced::Renderer>
where
    Theme: Catalog,
{
    content: Option<Element<'a, Message, Theme, Renderer>>,
    width: Length,
    height: Length,
    class: Theme::Class<'a>,
}

impl<'a, Message, Theme, Renderer> Quad<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
{
    pub fn new(content: Option<impl Into<Element<'a, Message, Theme, Renderer>>>) -> Self {
        Self {
            content: content.map(|element| element.into()),
            width: Length::Fixed(50.0),
            height: Length::Fixed(50.0),
            class: Theme::default(),
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }
}

impl<'a, Message, Theme, Renderer> Default for Quad<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
{
    fn default() -> Self {
        Self::new(None::<Element<Message, Theme, Renderer>>)
    }
}

fn content_layout(layout: Layout) -> Layout {
    layout
        .children()
        .next()
        .expect("failed to get content layout")
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Quad<'a, Message, Theme, Renderer>
where
    Renderer: advanced::Renderer,
    Theme: 'a + Catalog,
{
    fn size(&self) -> iced::Size<Length> {
        Size::new(self.width, self.height)
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits.width(self.width).height(self.height);

        match &self.content {
            Some(element) => layout::contained(&limits, self.width, self.height, |limits| {
                element
                    .as_widget()
                    .layout(&mut tree.children[0], renderer, limits)
            }),
            None => Node::new(limits.max()),
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &iced::Rectangle,
    ) {
        let bounds = layout.bounds();
        let status = if cursor.is_over(bounds) {
            Status::Hovered
        } else {
            Status::Active
        };
        let quad_style = style::quad::Catalog::style(theme, &self.class, status);

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: quad_style.border,
                shadow: quad_style.shadow,
            },
            quad_style.background.unwrap_or(Color::TRANSPARENT.into()),
        );

        if let Some(element) = &self.content {
            element.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                content_layout(layout),
                cursor,
                viewport,
            );
        }
    }

    fn tag(&self) -> tree::Tag {
        match &self.content {
            Some(element) => element.as_widget().tag(),
            None => tree::Tag::stateless(),
        }
    }

    fn children(&self) -> Vec<Tree> {
        match &self.content {
            Some(element) => vec![Tree::new(element)],
            None => vec![],
        }
    }

    fn diff(&self, tree: &mut Tree) {
        if let Some(element) = &self.content {
            tree.diff_children(std::slice::from_ref(element));
        }
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn advanced::widget::Operation,
    ) {
        if let Some(element) = &self.content {
            element.as_widget().operate(
                &mut tree.children[0],
                content_layout(layout),
                renderer,
                operation,
            );
        }
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) -> event::Status {
        match &mut self.content {
            Some(element) => element.as_widget_mut().on_event(
                &mut tree.children[0],
                event,
                content_layout(layout),
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            ),
            None => event::Status::Ignored,
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        match &self.content {
            Some(element) => element.as_widget().mouse_interaction(
                &tree.children[0],
                content_layout(layout),
                cursor,
                viewport,
                renderer,
            ),
            None => mouse::Interaction::None,
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: iced::Vector,
    ) -> Option<advanced::overlay::Element<'b, Message, Theme, Renderer>> {
        self.content.as_mut().and_then(|element| {
            element.as_widget_mut().overlay(
                &mut tree.children[0],
                content_layout(layout),
                renderer,
                translation,
            )
        })
    }
}

impl<'a, Message, Theme, Renderer> From<Quad<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a + Catalog,
    Renderer: 'a + advanced::Renderer,
{
    fn from(quad: Quad<'a, Message, Theme, Renderer>) -> Self {
        Element::new(quad)
    }
}
