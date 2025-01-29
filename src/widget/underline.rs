use crate::style;

use iced::{
    advanced::{
        self,
        widget::{tree, Widget},
    },
    widget::text_input,
    Border, Element, Length, Rectangle, Shadow, Size,
};

use iced_aw::style::number_input;

pub struct Underline<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Theme: style::underline::Catalog + number_input::ExtendedCatalog,
    Renderer: advanced::text::Renderer,
{
    element: ElementType<'a, Message, Theme, Renderer>,
    class: <Theme as style::underline::Catalog>::Class<'a>,
}

impl<'a, Message, Theme, Renderer> Underline<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Theme: style::underline::Catalog + number_input::ExtendedCatalog,
    Renderer: advanced::text::Renderer,
{
    pub fn new(content: ElementType<'a, Message, Theme, Renderer>) -> Self {
        Self {
            element: content,
            class: <Theme as style::underline::Catalog>::default(),
        }
    }
}

pub enum ElementType<'a, Message, Theme, Renderer> {
    NumberInput(Element<'a, Message, Theme, Renderer>),
    TextInput(Element<'a, Message, Theme, Renderer>),
}

impl<'a, Message, Theme, Renderer> ElementType<'a, Message, Theme, Renderer> {
    fn as_widget(&self) -> &dyn Widget<Message, Theme, Renderer> {
        match self {
            ElementType::NumberInput(element) => element.as_widget(),
            ElementType::TextInput(element) => element.as_widget(),
        }
    }

    fn as_widget_mut(&mut self) -> &mut dyn Widget<Message, Theme, Renderer> {
        match self {
            ElementType::NumberInput(element) => element.as_widget_mut(),
            ElementType::TextInput(element) => element.as_widget_mut(),
        }
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Underline<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Theme: style::underline::Catalog + iced_aw::style::number_input::ExtendedCatalog,
    Renderer: advanced::Renderer + advanced::text::Renderer,
{
    fn size(&self) -> Size<Length> {
        self.element.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.element.as_widget().size_hint()
    }

    fn tag(&self) -> tree::Tag {
        self.element.as_widget().tag()
    }

    fn state(&self) -> tree::State {
        self.element.as_widget().state()
    }

    fn children(&self) -> Vec<tree::Tree> {
        self.element.as_widget().children()
    }

    fn diff(&self, tree: &mut tree::Tree) {
        self.element.as_widget().diff(tree)
    }

    fn layout(
        &self,
        tree: &mut tree::Tree,
        renderer: &Renderer,
        limits: &advanced::layout::Limits,
    ) -> advanced::layout::Node {
        self.element.as_widget().layout(tree, renderer, limits)
    }

    fn operate(
        &self,
        state: &mut tree::Tree,
        layout: advanced::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn advanced::widget::Operation,
    ) {
        self.element
            .as_widget()
            .operate(state, layout, renderer, operation)
    }

    fn on_event(
        &mut self,
        state: &mut tree::Tree,
        event: iced::Event,
        layout: advanced::Layout<'_>,
        cursor: advanced::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn advanced::Clipboard,
        shell: &mut advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) -> advanced::graphics::core::event::Status {
        self.element.as_widget_mut().on_event(
            state, event, layout, cursor, renderer, clipboard, shell, viewport,
        )
    }

    fn draw(
        &self,
        tree: &tree::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &advanced::renderer::Style,
        layout: advanced::Layout<'_>,
        cursor: advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        self.element
            .as_widget()
            .draw(tree, renderer, theme, style, layout, cursor, viewport);

        let content_bounds = match self.element {
            ElementType::NumberInput(_) => {
                let mut children = layout.children();
                children
                    .next()
                    .expect("failed to get content layout")
                    .bounds()
            }
            ElementType::TextInput(_) => layout.bounds(),
        };

        let underline_bounds = Rectangle {
            x: content_bounds.x,
            y: content_bounds.y + content_bounds.height - 1.0,
            width: content_bounds.width,
            height: 1.0,
        };

        let is_mouse_over = cursor.is_over(content_bounds);

        let input_tree = match self.element {
            ElementType::NumberInput(_) => &tree.children[0],
            ElementType::TextInput(_) => tree,
        };

        let state = input_tree
            .state
            .downcast_ref::<text_input::State<Renderer::Paragraph>>();

        let status = if state.is_focused() {
            text_input::Status::Focused
        } else if is_mouse_over {
            text_input::Status::Hovered
        } else {
            text_input::Status::Active
        };

        let style = style::underline::Catalog::style(theme, &self.class, status);

        renderer.fill_quad(
            advanced::renderer::Quad {
                bounds: underline_bounds,
                border: Border::default(),
                shadow: Shadow::default(),
            },
            style.colour,
        )
    }

    fn mouse_interaction(
        &self,
        state: &tree::Tree,
        layout: advanced::Layout<'_>,
        cursor: advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
        renderer: &Renderer,
    ) -> advanced::mouse::Interaction {
        self.element
            .as_widget()
            .mouse_interaction(state, layout, cursor, viewport, renderer)
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut tree::Tree,
        layout: advanced::Layout<'_>,
        renderer: &Renderer,
        translation: iced::Vector,
    ) -> Option<advanced::overlay::Element<'b, Message, Theme, Renderer>> {
        self.element
            .as_widget_mut()
            .overlay(tree, layout, renderer, translation)
    }
}

impl<'a, Message, Theme, Renderer> From<Underline<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + style::underline::Catalog + number_input::ExtendedCatalog,
    Renderer: 'a + advanced::text::Renderer,
{
    fn from(underline: Underline<'a, Message, Theme, Renderer>) -> Self {
        Self::new(underline)
    }
}
