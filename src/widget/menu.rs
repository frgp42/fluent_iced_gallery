use crate::widget::{Menu, MenuItem};

pub fn standard<Message>(items: Vec<MenuItem<Message>>) -> Menu<Message> {
    Menu::new(items)
        .width(100)
        // Negative spacing to counter padding in menu items
        // Ideally, Menu should implement Padding
        .spacing(-4.0)
}

pub mod bar {
    use crate::{
        style,
        widget::{MenuBar, MenuItem},
    };

    pub fn standard<Message>(roots: Vec<MenuItem<Message>>) -> MenuBar<Message> {
        MenuBar::new(roots).style(style::menu_bar::default)
    }
}

pub mod item {
    use crate::{
        fluent_icon::FluentIcon,
        style,
        widget::{text, Button, Element, Menu, MenuItem, Quad, Row},
    };

    use iced::{
        alignment::{Horizontal, Vertical},
        widget::{center, horizontal_space},
        Length,
    };

    fn contents<'a, Message>(
        content: impl Into<Element<'a, Message>>,
        on_press: Option<Message>,
    ) -> Element<'a, Message>
    where
        Message: 'a + Clone,
    {
        // Container (center) is for padding as Menu does not provide any
        center(
            Button::new(content)
                .width(Length::Fill)
                .height(28)
                .padding([3, 10])
                .style(style::button::menu_item)
                .on_press_maybe(on_press),
        )
        .width(Length::Fill)
        .height(Length::Shrink)
        .padding(5)
        .into()
    }

    // This should be improved once keyboard navigation is implemented in Iced
    pub fn labelled<'a, Message>(
        label: &'a str,
        indented: bool,
        accelerator: Option<&'a str>,
        on_press: Message,
    ) -> MenuItem<'a, Message>
    where
        Message: 'a + Clone,
    {
        let content = Row::new()
            .push_maybe(indented.then_some(horizontal_space().width(30)))
            .push(text::body1(label))
            .push(horizontal_space())
            .push_maybe(accelerator.map(|a| text::body1(a).size(11)));

        MenuItem::new(contents(content, Some(on_press)))
    }

    pub fn radio<'a, V, Message>(
        label: &'a str,
        value: V,
        selected: Option<V>,
        on_selected: Message,
    ) -> MenuItem<'a, Message>
    where
        Message: 'a + Clone,
        V: Copy + Eq,
    {
        let bullet = if Some(value) == selected {
            FluentIcon::RadioBullet.codepoint()
        } else {
            '\0'
        };

        let content = Row::new()
            .push(
                text::icon(bullet)
                    .width(14)
                    .height(Length::Fill)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .size(12),
            )
            .push(horizontal_space().width(16))
            .push(
                text::body1(label)
                    .height(Length::Fill)
                    .align_y(Vertical::Center),
            );

        MenuItem::new(contents(content, Some(on_selected)))
    }

    pub fn submenu<'a, Message>(
        label: &'a str,
        indented: bool,
        menu: Menu<'a, Message>,
    ) -> MenuItem<'a, Message>
    where
        Message: 'a + Clone,
    {
        let content = Row::new()
            .push_maybe(indented.then_some(horizontal_space().width(50)))
            .push(text::body1(label))
            .push(horizontal_space())
            .push(
                text::icon(FluentIcon::ChevronRightMed.codepoint())
                    .size(8)
                    .height(Length::Fill)
                    .align_y(Vertical::Center),
            );

        MenuItem::with_menu(contents(content, None), menu)
    }

    pub fn separator<'a, Message>() -> MenuItem<'a, Message>
    where
        Message: 'a,
    {
        MenuItem::new(
            Quad::new(None::<Element<Message>>)
                .width(Length::Fill)
                .height(1)
                .style(style::quad::separator),
        )
    }
}
