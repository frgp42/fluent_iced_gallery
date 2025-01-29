use crate::{
    fluent_icon::FluentIcon,
    page::{page, widget_example},
    style,
    widget::{button, text, Column, Container, DropDown, Element},
};

use iced::{widget::row, Length};

#[derive(Clone, Debug, PartialEq)]
pub enum OpenButton {
    Simple,
    Icon,
}

#[derive(Clone, Debug, Default)]
pub struct DropDownButton {
    current_open: Option<OpenButton>,
}

#[derive(Clone, Debug)]
pub enum Message {
    DropDownPressed(OpenButton),
    DropDownDismissed,
}

impl DropDownButton {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::DropDownPressed(button) => match self.current_open {
                None => self.current_open = Some(button),
                Some(_) => self.current_open = None,
            },
            Message::DropDownDismissed => self.current_open = None,
        }
    }

    pub fn view(&self) -> Element<'static, Message> {
        let simple_underlay = underlay(OpenButton::Simple, text::body1("Email"));
        let simple_overlay = overlay([
            simple_button("Send"),
            simple_button("Reply"),
            simple_button("Reply All"),
        ])
        .width(84);
        let simple_drop_down = DropDown::new(
            simple_underlay,
            simple_overlay,
            self.is_open(OpenButton::Simple),
        )
        .width(Length::Shrink)
        .on_dismiss(Message::DropDownDismissed);

        let icon_underlay = underlay(OpenButton::Icon, text::icon(FluentIcon::Mail.codepoint()));
        let icon_overlay = overlay([
            icon_button(FluentIcon::SendFluentIconll, "Send"),
            icon_button(FluentIcon::MailReply, "Reply"),
            icon_button(FluentIcon::MailReplyAll, "Reply All"),
        ])
        .width(112);
        let icon_drop_down =
            DropDown::new(icon_underlay, icon_overlay, self.is_open(OpenButton::Icon))
                .width(Length::Shrink)
                .on_dismiss(Message::DropDownDismissed);

        page(
            "Drop Down Button",
            [
                widget_example(
                    "A simple Drop Down Button.",
                    row![simple_drop_down],
                    None::<Element<Message>>,
                    None::<Element<Message>>,
                ),
                widget_example(
                    "Drop Down Button with Icons",
                    icon_drop_down,
                    None::<Element<Message>>,
                    None::<Element<Message>>,
                ),
            ],
        )
    }

    fn is_open(&self, button: OpenButton) -> bool {
        match &self.current_open {
            Some(open_button) => *open_button == button,
            None => false,
        }
    }
}

fn underlay<'a>(id: OpenButton, content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    button::dropdown(content)
        .on_press(Message::DropDownPressed(id))
        .into()
}

fn overlay<'a>(items: impl IntoIterator<Item = Element<'a, Message>>) -> Container<'a, Message> {
    Container::new(Column::with_children(items))
        .padding(6)
        .style(style::container::overlay)
}

fn simple_button(label: &str) -> Element<Message> {
    button::menu_labelled(label)
        .on_press(Message::DropDownDismissed)
        .into()
}

fn icon_button(icon: FluentIcon, label: &str) -> Element<Message> {
    button::menu_icon(label, icon)
        .on_press(Message::DropDownDismissed)
        .into()
}
