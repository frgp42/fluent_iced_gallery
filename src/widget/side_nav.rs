use crate::{
    fluent_icon::FluentIcon,
    style,
    widget::{scrollable, text, Button, Column, Container, DropDown, Element, Row, Text},
};

use iced::{
    alignment::Vertical,
    widget::{
        horizontal_space, row,
        text::{Fragment, IntoFragment},
    },
    Length,
};

use iced_aw::drop_down::Alignment;

#[derive(Clone, Debug, PartialEq)]
pub enum DisplayMode {
    // Nav button and NavItem icons
    Compact,
    // Nav button and NavItems with icons and labels
    Full,
}

pub struct SideNav<'a, Message>
where
    Message: 'a + Clone,
{
    full_width: Length,
    height: Length,
    display_mode: DisplayMode,
    groups: Vec<Group<'a, Message>>,
    footer_groups: Vec<Group<'a, Message>>,
    on_nav_button_pressed: Message,
}

impl<'a, Message> SideNav<'a, Message>
where
    Message: 'a + Clone,
{
    pub fn new(display_mode: DisplayMode, on_nav_button_pressed: Message) -> Self {
        Self {
            full_width: Length::Fixed(300.0),
            height: Length::Fill,
            display_mode,
            groups: Vec::new(),
            footer_groups: Vec::new(),
            on_nav_button_pressed,
        }
    }

    pub fn with_groups(mut self, items: impl IntoIterator<Item = Group<'a, Message>>) -> Self {
        self.groups.extend(items);
        self
    }

    pub fn with_footer_groups(
        mut self,
        items: impl IntoIterator<Item = Group<'a, Message>>,
    ) -> Self {
        self.footer_groups.extend(items);
        self
    }

    fn nav_button(&self) -> Button<'a, Message> {
        button_base(compact_view_contents(
            FluentIcon::GlobalNavButton.codepoint(),
        ))
        .on_press(self.on_nav_button_pressed.clone())
    }

    fn compact_view(self) -> Element<'a, Message> {
        let mut contents = vec![self.nav_button().width(Length::Shrink).into()];

        let group_buttons = scrollable::standard(Column::with_children(
            self.groups.into_iter().map(|group| group.compact_view()),
        ))
        .height(Length::Fill)
        .into();

        let footer_buttons = self
            .footer_groups
            .into_iter()
            .map(|group| group.compact_view());

        contents.push(group_buttons);
        contents.extend(footer_buttons);

        Column::with_children(contents).height(self.height).into()
    }

    fn full_view(self) -> Element<'a, Message> {
        let mut contents = vec![self.nav_button().into()];

        let group_items = scrollable::standard(Column::with_children(
            self.groups.into_iter().map(|group| group.full_view()),
        ))
        .height(Length::Fill)
        .into();

        let footer_group_items = self
            .footer_groups
            .into_iter()
            .map(|group| group.full_view());

        contents.push(group_items);
        contents.extend(footer_group_items);

        Column::with_children(contents)
            .width(self.full_width)
            .height(self.height)
            .into()
    }
}

impl<'a, Message> From<SideNav<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(side_nav: SideNav<'a, Message>) -> Self {
        match side_nav.display_mode {
            DisplayMode::Compact => side_nav.compact_view(),
            DisplayMode::Full => side_nav.full_view(),
        }
    }
}

pub struct Group<'a, Message>
where
    Message: 'a + Clone,
{
    icon: char,
    label: Fragment<'a>,
    overlay_width: Length,
    items: Vec<Item<'a, Message>>,
    expanded: bool,
    on_press: Message,
    on_overlay_dismiss: Option<Message>,
}

impl<'a, Message> Group<'a, Message>
where
    Message: 'a + Clone,
{
    pub fn new(icon: char, label: impl IntoFragment<'a>, on_press: Message) -> Self {
        Self {
            icon,
            label: label.into_fragment(),
            overlay_width: 300.into(),
            items: Vec::new(),
            expanded: false,
            on_press,
            on_overlay_dismiss: None,
        }
    }

    pub fn overlay_width(mut self, width: impl Into<Length>) -> Self {
        self.overlay_width = width.into();
        self
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    pub fn with_items(mut self, items: impl IntoIterator<Item = Item<'a, Message>>) -> Self {
        self.items.extend(items);
        self
    }

    pub fn on_overlay_dismiss(mut self, message: Message) -> Self {
        self.on_overlay_dismiss = Some(message);
        self
    }

    fn full_view(self) -> Element<'a, Message> {
        let indicator = if self.items.is_empty() {
            None
        } else if self.expanded {
            Some(FluentIcon::ChevronUp)
        } else {
            Some(FluentIcon::ChevronDown)
        };

        let header = button_base(
            full_view_contents(Some(self.icon), self.label)
                .push_maybe(indicator.map(|chevron| text::icon(chevron.codepoint()).size(8))),
        )
        .on_press(self.on_press);

        if self.expanded {
            let mut contents = vec![header.into()];
            contents.extend(self.items.into_iter().map(|item| {
                button_base(full_view_contents(item.icon, item.label))
                    .on_press(item.on_press)
                    .into()
            }));

            Column::with_children(contents).into()
        } else {
            header.into()
        }
    }

    fn compact_view(self) -> Element<'a, Message> {
        let underlay = button_base(compact_view_contents(self.icon))
            .width(Length::Shrink)
            .on_press(self.on_press);

        let overlay = Container::new(Column::with_children(self.items.into_iter().map(|item| {
            button_base(
                text::body1(item.label)
                    .height(Length::Fill)
                    .align_y(Vertical::Center),
            )
            .on_press(item.on_press)
            .into()
        })))
        .width(self.overlay_width)
        .style(style::container::overlay);

        let button = DropDown::new(underlay, overlay, self.expanded)
            .width(Length::Shrink)
            .alignment(Alignment::BottomEnd);

        if let Some(message) = self.on_overlay_dismiss {
            button.on_dismiss(message).into()
        } else {
            button.into()
        }
    }
}

pub struct Item<'a, Message>
where
    Message: 'a + Clone,
{
    icon: Option<char>,
    label: Fragment<'a>,
    on_press: Message,
}

impl<'a, Message> Item<'a, Message>
where
    Message: 'a + Clone,
{
    pub fn new(icon: Option<char>, label: impl IntoFragment<'a>, on_press: Message) -> Self {
        Self {
            icon,
            label: label.into_fragment(),
            on_press,
        }
    }
}

fn button_base<'a, Message>(content: impl Into<Element<'a, Message>>) -> Button<'a, Message> {
    Button::new(content)
        .width(Length::Fill)
        .height(36.0)
        .padding([0.0, 12.0])
        .style(style::button::nav_item)
}

fn full_view_contents<'a, Message: 'a>(
    icon: Option<char>,
    label: impl IntoFragment<'a>,
) -> Row<'a, Message> {
    let icon: Element<'a, Message> = icon
        .map_or(horizontal_space().width(16).into(), |codepoint| {
            text::icon(codepoint).into()
        });

    row![icon, text::body1(label), horizontal_space(),]
        .height(Length::Fill)
        .spacing(18)
        .align_y(Vertical::Center)
}

fn compact_view_contents<'a>(icon: char) -> Text<'a> {
    text::icon(icon)
        .height(Length::Fill)
        .align_y(Vertical::Center)
}
