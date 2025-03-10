use crate::{
    fluent_icon::FluentIcon,
    font, style, theme,
    widget::underline::{ElementType, Underline},
};

use std::{
    fmt::Display,
    ops::{Bound, RangeBounds},
    str::FromStr,
};

use iced::{
    advanced::{
        layout::{Limits, Node},
        renderer,
        widget::{
            tree::{State, Tag},
            Operation, Tree,
        },
        Clipboard, Layout, Shell, Widget,
    },
    alignment::{Horizontal, Vertical},
    event, keyboard,
    mouse::{self, Cursor},
    widget::{
        text::{LineHeight, Wrapping},
        text_input::{self as iced_text_input, cursor},
        Column, Container, Row, Text,
    },
    Alignment, Background, Border, Color, Element, Event, Length, Padding, Pixels, Point,
    Rectangle, Shadow, Size,
};

use iced_aw::{
    style::{
        number_input::{Catalog, ExtendedCatalog},
        Status,
    },
    widget::number_input,
    TypedInput,
};

use num_traits::{Bounded, Num, NumAssignOps};

const DEFAULT_PADDING: Padding = Padding::new(5.0);

pub struct NumberInput<'a, T, Message, Theme = theme::Theme, Renderer = iced::Renderer>
where
    Theme: Catalog + ExtendedCatalog,
    Renderer: iced::advanced::text::Renderer<Font = iced::Font>,
{
    value: T,
    step: T,
    min: T,
    max: T,
    padding: iced::Padding,
    size: Option<iced::Pixels>,
    content: TypedInput<'a, T, Message, Theme, Renderer>,
    on_change: Box<dyn Fn(T) -> Message>,
    class: <Theme as number_input::Catalog>::Class<'a>,
    font: Renderer::Font,
    width: Length,
    ignore_scroll_events: bool,
    ignore_buttons: bool,
}

impl<'a, T, Message, Theme, Renderer> NumberInput<'a, T, Message, Theme, Renderer>
where
    T: Num + NumAssignOps + PartialOrd + Display + FromStr + Copy + Bounded,
    Message: 'a + Clone,
    Theme: Catalog + ExtendedCatalog,
    Renderer: iced::advanced::text::Renderer<Font = iced::Font>,
{
    pub fn new<F>(value: T, bounds: impl RangeBounds<T>, on_change: F) -> Self
    where
        F: 'static + Fn(T) -> Message + Copy,
        T: 'static,
    {
        let padding = DEFAULT_PADDING;

        Self {
            value,
            step: T::one(),
            min: Self::set_min(bounds.start_bound()),
            max: Self::set_max(bounds.end_bound()),
            padding,
            size: Some(Pixels(14.0)),
            content: TypedInput::new("", &value)
                .size(14.0)
                .line_height(LineHeight::Absolute(Pixels(20.0)))
                .padding(padding)
                .width(Length::Fixed(60.0))
                .class(Theme::default_input())
                .on_input(on_change),
            on_change: Box::new(on_change),
            class: <Theme as number_input::Catalog>::default(),
            font: Renderer::Font::default(),
            width: Length::Shrink,
            ignore_scroll_events: false,
            ignore_buttons: false,
        }
    }

    #[must_use]
    pub fn bounds(mut self, bounds: impl RangeBounds<T>) -> Self {
        self.min = Self::set_min(bounds.start_bound());
        self.max = Self::set_max(bounds.end_bound());

        self
    }

    #[must_use]
    pub fn content_width(mut self, width: impl Into<Length>) -> Self {
        self.content = self.content.width(width);
        self
    }

    #[allow(clippy::needless_pass_by_value)]
    #[must_use]
    pub fn font(mut self, font: Renderer::Font) -> Self {
        self.font = font;
        self.content = self.content.font(font);
        self
    }

    #[must_use]
    pub fn ignore_buttons(mut self, ignore: bool) -> Self {
        self.ignore_buttons = ignore;
        self
    }

    #[must_use]
    pub fn ignore_scroll(mut self, ignore: bool) -> Self {
        self.ignore_scroll_events = ignore;
        self
    }

    #[must_use]
    pub fn on_submit(mut self, message: Message) -> Self {
        self.content = self.content.on_submit(move |_| message.clone());
        self
    }

    #[must_use]
    pub fn padding(mut self, padding: impl Into<iced::Padding>) -> Self {
        let padding = padding.into();
        self.padding = padding;
        self.content = self.content.padding(padding);
        self
    }

    #[must_use]
    pub fn size(mut self, size: impl Into<iced::Pixels>) -> Self {
        let size = size.into();
        self.size = Some(size);
        self.content = self.content.size(size);
        self
    }

    #[must_use]
    pub fn step(mut self, step: T) -> Self {
        self.step = step;
        self
    }

    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> number_input::Style + 'a) -> Self
    where
        <Theme as number_input::Catalog>::Class<'a>:
            From<number_input::StyleFn<'a, Theme, number_input::Style>>,
    {
        self.class =
            (Box::new(style) as number_input::StyleFn<'a, Theme, number_input::Style>).into();
        self
    }

    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    fn decrease_value(&mut self, shell: &mut Shell<Message>) {
        if self.value < self.min + self.step {
            self.value = self.min;
        } else {
            self.value -= self.step;
        }

        shell.publish((self.on_change)(self.value));
    }

    fn increase_value(&mut self, shell: &mut Shell<Message>) {
        if self.value > self.max - self.step {
            self.value = self.max;
        } else {
            self.value += self.step;
        }
        shell.publish((self.on_change)(self.value));
    }

    fn set_min(min: Bound<&T>) -> T {
        match min {
            Bound::Included(n) | Bound::Excluded(n) => *n,
            Bound::Unbounded => T::min_value(),
        }
    }

    fn set_max(max: Bound<&T>) -> T {
        match max {
            Bound::Included(n) => *n,
            Bound::Excluded(n) => *n - T::one(),
            Bound::Unbounded => T::max_value(),
        }
    }

    #[must_use]
    pub fn input_style(
        mut self,
        style: impl Fn(&Theme, iced_text_input::Status) -> iced_text_input::Style + 'a + Clone,
    ) -> Self
    where
        <Theme as iced_text_input::Catalog>::Class<'a>: From<iced_text_input::StyleFn<'a, Theme>>,
    {
        self.content = self.content.style(style);
        self
    }

    #[must_use]
    pub fn class(mut self, class: impl Into<<Theme as number_input::Catalog>::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<'a, T, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for NumberInput<'a, T, Message, Theme, Renderer>
where
    T: Num + NumAssignOps + PartialOrd + Display + FromStr + ToString + Copy + Bounded,
    Message: 'a + Clone,
    Theme: 'a + ExtendedCatalog + Catalog,
    Renderer: 'a + iced::advanced::text::Renderer<Font = iced::Font>,
{
    fn tag(&self) -> Tag {
        Tag::of::<ModifierState>()
    }
    fn state(&self) -> State {
        State::new(ModifierState::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree {
            tag: self.content.tag(),
            state: self.content.state(),
            children: self.content.children(),
        }]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children_custom(
            &[&self.content],
            |state, content| content.diff(state),
            |&content| Tree {
                tag: content.tag(),
                state: content.state(),
                children: content.children(),
            },
        );
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.width, Length::Shrink)
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let num_size = self.size();
        let limits = limits.width(num_size.width).height(Length::Shrink);
        let content = self
            .content
            .layout(&mut tree.children[0], renderer, &limits);
        let limits2 = Limits::new(Size::new(0.0, 0.0), content.size());
        let txt_size = self.size.unwrap_or_else(|| renderer.default_size());

        let icon_size = txt_size * 2.5 / 4.0;
        let btn_mod = |c| {
            Container::<Message, Theme, Renderer>::new(Text::new(format!(" {c} ")).size(icon_size))
                .center_y(Length::Shrink)
                .center_x(Length::Shrink)
        };

        let default_padding = DEFAULT_PADDING;

        let element = if self.padding.top < default_padding.top
            || self.padding.bottom < default_padding.bottom
            || self.padding.right < default_padding.right
        {
            Element::new(
                Row::<Message, Theme, Renderer>::new()
                    .spacing(1)
                    .width(Length::Shrink)
                    .push(btn_mod('+'))
                    .push(btn_mod('-')),
            )
        } else {
            Element::new(
                Column::<Message, Theme, Renderer>::new()
                    .spacing(1)
                    .width(Length::Shrink)
                    .push(btn_mod('▲'))
                    .push(btn_mod('▼')),
            )
        };

        let input_tree = if let Some(child_tree) = tree.children.get_mut(1) {
            child_tree.diff(element.as_widget());
            child_tree
        } else {
            let child_tree = Tree::new(element.as_widget());
            tree.children.insert(1, child_tree);
            &mut tree.children[1]
        };

        let mut modifier = element
            .as_widget()
            .layout(input_tree, renderer, &limits2.loose());
        let intrinsic = Size::new(
            content.size().width - 1.0,
            content.size().height.max(modifier.size().height),
        );
        modifier = modifier.align(Alignment::End, Alignment::Center, intrinsic);

        let size = limits.resolve(num_size.width, Length::Shrink, intrinsic);
        Node::with_children(size, vec![content, modifier])
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.content.operate(
                &mut tree.children[0],
                layout
                    .children()
                    .next()
                    .expect("NumberInput inner child Textbox was not created."),
                renderer,
                operation,
            );
        });
    }

    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        let mut children = layout.children();
        let content = children.next().expect("failed to get content layout");
        let mut mod_children = children
            .next()
            .expect("failed to get modifiers layout")
            .children();
        let inc_bounds = mod_children
            .next()
            .expect("failed to get increase mod layout")
            .bounds();
        let dec_bounds = mod_children
            .next()
            .expect("failed to get decrease mod layout")
            .bounds();

        if self.min == self.max {
            return event::Status::Ignored;
        }

        let cursor_position = cursor.position().unwrap_or_default();
        let mouse_over_widget = layout.bounds().contains(cursor_position);
        let mouse_over_inc = inc_bounds.contains(cursor_position);
        let mouse_over_dec = dec_bounds.contains(cursor_position);
        let mouse_over_button = mouse_over_inc || mouse_over_dec;

        let child = state.children.get_mut(0).expect("failed to get child");
        let text_input = child
            .state
            .downcast_mut::<iced_text_input::State<Renderer::Paragraph>>();
        let modifiers = state.state.downcast_mut::<ModifierState>();

        let current_text = self.content.text().to_owned();

        let mut forward_to_text = |event, shell, child, clipboard| {
            self.content.on_event(
                child, event, content, cursor, renderer, clipboard, shell, viewport,
            )
        };

        match &event {
            Event::Keyboard(ke) => {
                if !text_input.is_focused() {
                    return event::Status::Ignored;
                }
                let (key, text) = match ke {
                    keyboard::Event::KeyPressed { key, text, .. } => (key, text),
                    keyboard::Event::ModifiersChanged(_) => {
                        return forward_to_text(event, shell, child, clipboard)
                    }
                    keyboard::Event::KeyReleased { .. } => return event::Status::Ignored,
                };
                match text {
                    Some(text) => {
                        if text == "\u{1}" || text == "\u{3}" {
                            // CTRL + a and CTRL + c
                            forward_to_text(event, shell, child, clipboard)
                        } else if text == "\u{8}" {
                            // Backspace
                            if current_text == T::zero().to_string() {
                                return event::Status::Ignored;
                            }
                            let mut new_val = current_text;
                            match text_input
                                .cursor()
                                .state(&iced_text_input::Value::new(&new_val))
                            {
                                cursor::State::Index(idx) if idx >= 1 && idx <= new_val.len() => {
                                    _ = new_val.remove(idx - 1);
                                }
                                cursor::State::Selection { start, end }
                                    if start <= new_val.len() && end <= new_val.len() =>
                                {
                                    new_val.replace_range(start.min(end)..start.max(end), "");
                                }
                                _ => return event::Status::Ignored,
                            }

                            if new_val.is_empty() {
                                new_val = T::zero().to_string();
                            }

                            match T::from_str(&new_val) {
                                Ok(val)
                                    if val >= self.min && val <= self.max && val != self.value =>
                                {
                                    self.value = val;
                                    forward_to_text(event, shell, child, clipboard)
                                }
                                Ok(val) if val >= self.min && val <= self.max => {
                                    forward_to_text(event, shell, child, clipboard)
                                }
                                Ok(_) => event::Status::Captured,
                                _ => event::Status::Ignored,
                            }
                        } else {
                            let input = if text == "\u{16}" {
                                // CTRL + v
                                match clipboard.read(iced::advanced::clipboard::Kind::Standard) {
                                    Some(paste) => paste,
                                    None => return event::Status::Ignored,
                                }
                            } else if text.parse::<i64>().is_err() && text != "-" && text != "." {
                                return event::Status::Ignored;
                            } else {
                                text.to_string()
                            };

                            let input = input.trim();

                            let mut new_val = current_text;
                            match text_input
                                .cursor()
                                .state(&iced_text_input::Value::new(&new_val))
                            {
                                cursor::State::Index(idx) if idx <= new_val.len() => {
                                    new_val.insert_str(idx, input);
                                }
                                cursor::State::Selection { start, end }
                                    if start <= new_val.len() && end <= new_val.len() =>
                                {
                                    new_val.replace_range(start.min(end)..end.max(start), input);
                                }
                                _ => return event::Status::Ignored,
                            }

                            match T::from_str(&new_val) {
                                Ok(val)
                                    if val >= self.min && val <= self.max && val != self.value =>
                                {
                                    self.value = val;
                                    forward_to_text(event, shell, child, clipboard)
                                }
                                Ok(val) if val >= self.min && val <= self.max => {
                                    forward_to_text(event, shell, child, clipboard)
                                }
                                Ok(_) => event::Status::Captured,
                                _ => event::Status::Ignored,
                            }
                        }
                    }
                    None => match key {
                        keyboard::Key::Named(keyboard::key::Named::ArrowDown) => {
                            self.decrease_value(shell);
                            event::Status::Captured
                        }
                        keyboard::Key::Named(keyboard::key::Named::ArrowUp) => {
                            self.increase_value(shell);
                            event::Status::Captured
                        }
                        keyboard::Key::Named(
                            keyboard::key::Named::ArrowLeft
                            | keyboard::key::Named::ArrowRight
                            | keyboard::key::Named::Home
                            | keyboard::key::Named::End,
                        ) => forward_to_text(event, shell, child, clipboard),
                        _ => event::Status::Ignored,
                    },
                }
            }
            Event::Mouse(mouse::Event::WheelScrolled { delta })
                if mouse_over_widget && !self.ignore_scroll_events =>
            {
                match delta {
                    mouse::ScrollDelta::Lines { y, .. } | mouse::ScrollDelta::Pixels { y, .. } => {
                        if y.is_sign_positive() {
                            self.increase_value(shell);
                        } else {
                            self.decrease_value(shell);
                        }
                    }
                }
                event::Status::Captured
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                if mouse_over_button && !self.ignore_buttons =>
            {
                if mouse_over_dec {
                    modifiers.decrease_pressed = true;
                    self.decrease_value(shell);
                } else {
                    modifiers.increase_pressed = true;
                    self.increase_value(shell);
                }
                event::Status::Captured
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                if mouse_over_button =>
            {
                if mouse_over_dec {
                    modifiers.decrease_pressed = false;
                } else {
                    modifiers.increase_pressed = false;
                }
                event::Status::Captured
            }
            _ => forward_to_text(event, shell, child, clipboard),
        }
    }

    fn mouse_interaction(
        &self,
        _state: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let bounds = layout.bounds();
        let mut children = layout.children();
        let _content_layout = children.next().expect("failed to get content layout");
        let mut mod_children = children
            .next()
            .expect("failed to get modifiers layout")
            .children();
        let inc_bounds = mod_children
            .next()
            .expect("failed to get increase mod layout")
            .bounds();
        let dec_bounds = mod_children
            .next()
            .expect("failed to get decrease mod layout")
            .bounds();
        let is_mouse_over = bounds.contains(cursor.position().unwrap_or_default());
        let is_decrease_disabled = self.value <= self.min || self.min == self.max;
        let is_increase_disabled = self.value >= self.max || self.min == self.max;
        let mouse_over_decrease = dec_bounds.contains(cursor.position().unwrap_or_default());
        let mouse_over_increase = inc_bounds.contains(cursor.position().unwrap_or_default());

        if ((mouse_over_decrease && !is_decrease_disabled)
            || (mouse_over_increase && !is_increase_disabled))
            && !self.ignore_buttons
        {
            mouse::Interaction::Pointer
        } else if is_mouse_over {
            mouse::Interaction::Text
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let mut children = layout.children();
        let content_layout = children.next().expect("failed to get content layout");
        let mut mod_children = children
            .next()
            .expect("failed to get modifiers layout")
            .children();
        let inc_bounds = mod_children
            .next()
            .expect("failed to get increase mod layout")
            .bounds();
        let dec_bounds = mod_children
            .next()
            .expect("failed to get decrease mod layout")
            .bounds();
        self.content.draw(
            &state.children[0],
            renderer,
            theme,
            style,
            content_layout,
            cursor,
            viewport,
        );
        let is_decrease_disabled = self.value <= self.min || self.min == self.max;
        let is_increase_disabled = self.value >= self.max || self.min == self.max;

        let decrease_btn_style = if is_decrease_disabled {
            number_input::Catalog::style(theme, &self.class, Status::Disabled)
        } else if state.state.downcast_ref::<ModifierState>().decrease_pressed {
            number_input::Catalog::style(theme, &self.class, Status::Pressed)
        } else {
            number_input::Catalog::style(theme, &self.class, Status::Active)
        };

        let increase_btn_style = if is_increase_disabled {
            number_input::Catalog::style(theme, &self.class, Status::Disabled)
        } else if state.state.downcast_ref::<ModifierState>().increase_pressed {
            number_input::Catalog::style(theme, &self.class, Status::Pressed)
        } else {
            number_input::Catalog::style(theme, &self.class, Status::Active)
        };

        let txt_size = self.size.unwrap_or_else(|| renderer.default_size());

        let icon_size = txt_size * 2.5 / 4.0;

        if self.ignore_buttons {
            return;
        }
        // decrease button section
        if dec_bounds.intersects(viewport) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: dec_bounds,
                    border: Border {
                        radius: (3.0).into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    shadow: Shadow::default(),
                },
                decrease_btn_style
                    .button_background
                    .unwrap_or(Background::Color(Color::TRANSPARENT)),
            );
        }

        renderer.fill_text(
            iced::advanced::text::Text {
                content: FluentIcon::ChevronDown.codepoint().to_string(),
                bounds: Size::new(dec_bounds.width, dec_bounds.height),
                size: icon_size,
                font: font::SEGOE_FLUENT_ICONS,
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                line_height: LineHeight::Relative(1.3),
                shaping: iced::advanced::text::Shaping::Advanced,
                wrapping: Wrapping::default(),
            },
            Point::new(dec_bounds.center_x(), dec_bounds.center_y()),
            decrease_btn_style.icon_color,
            dec_bounds,
        );

        // increase button section
        if inc_bounds.intersects(viewport) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: inc_bounds,
                    border: Border {
                        radius: (3.0).into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    shadow: Shadow::default(),
                },
                increase_btn_style
                    .button_background
                    .unwrap_or(Background::Color(Color::TRANSPARENT)),
            );
        }

        renderer.fill_text(
            iced::advanced::text::Text {
                content: FluentIcon::ChevronUp.codepoint().to_string(),
                bounds: Size::new(inc_bounds.width, inc_bounds.height),
                size: icon_size,
                font: font::SEGOE_FLUENT_ICONS,
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                line_height: LineHeight::Relative(1.3),
                shaping: iced::advanced::text::Shaping::Advanced,
                wrapping: Wrapping::default(),
            },
            Point::new(inc_bounds.center_x(), inc_bounds.center_y()),
            increase_btn_style.icon_color,
            inc_bounds,
        );
    }
}

#[derive(Default, Clone, Debug)]
pub struct ModifierState {
    pub decrease_pressed: bool,
    pub increase_pressed: bool,
}

impl<'a, T, Message, Theme, Renderer> From<NumberInput<'a, T, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    T: 'a + Num + NumAssignOps + PartialOrd + Display + FromStr + Copy + Bounded,
    Message: 'a + Clone,
    Theme: 'a + ExtendedCatalog + Catalog,
    Renderer: 'a + iced::advanced::text::Renderer<Font = iced::Font>,
{
    fn from(num_input: NumberInput<'a, T, Message, Theme, Renderer>) -> Self {
        Element::new(num_input)
    }
}

pub fn underline<'a, T, Message, Theme, Renderer>(
    number_input: NumberInput<'a, T, Message, Theme, Renderer>,
) -> Element<'a, Message, Theme, Renderer>
where
    T: 'a + Num + NumAssignOps + PartialOrd + Display + FromStr + Copy + Bounded,
    Message: 'a + Clone,
    Theme: 'a + Catalog + ExtendedCatalog + style::underline::Catalog,
    Renderer: 'a + iced::advanced::text::Renderer<Font = iced::Font>,
{
    let element = iced::Element::new(number_input);
    let underline_type = ElementType::NumberInput(element);
    Underline::new(underline_type).into()
}
