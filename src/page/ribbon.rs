use crate::{
    fluent_icon::FluentIcon,
    page::{page, widget_example},
    style,
    widget::{button, ribbon, text, Element},
};

use iced::widget::{column, row, Container};

#[derive(Clone, Debug, Default)]
pub struct Ribbon {
    mail_button_open: bool,
    last_button: Option<&'static str>,
}

#[derive(Clone, Debug)]
pub enum Message {
    MailSplitButtonOpened,
    MailSplitButtonClosed,
    MailButtonPressed,
    EventButtonPressed,
    GroupButtonPressed,
    DeleteButtonPressed,
    CutButtonPressed,
    CopyButtonPressed,
    PasteButtonPressed,
    ReplyButtonPressed,
    ForwardButtonPressed,
}

impl Message {
    fn button_pressed(&self) -> Option<&'static str> {
        match self {
            Message::MailSplitButtonOpened => None,
            Message::MailSplitButtonClosed => None,
            Message::MailButtonPressed => Some("Mail"),
            Message::EventButtonPressed => Some("Event"),
            Message::GroupButtonPressed => Some("Group"),
            Message::DeleteButtonPressed => Some("Delete"),
            Message::CutButtonPressed => Some("Cut"),
            Message::CopyButtonPressed => Some("Copy"),
            Message::PasteButtonPressed => Some("Paste"),
            Message::ReplyButtonPressed => Some("Reply"),
            Message::ForwardButtonPressed => Some("Forward"),
        }
    }
}

impl Ribbon {
    pub fn update(&mut self, message: Message) {
        if let Some(button) = message.button_pressed() {
            self.last_button = Some(button);
        }
        let mut close_mail = || self.mail_button_open = false;

        match message {
            Message::MailButtonPressed => close_mail(),
            Message::MailSplitButtonOpened => self.mail_button_open = true,
            Message::MailSplitButtonClosed => close_mail(),
            Message::EventButtonPressed => close_mail(),
            Message::GroupButtonPressed => close_mail(),
            _ => (),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let mail_button_flyout = Container::new(column![
            button::menu_icon("Mail", FluentIcon::Mail).on_press(Message::MailButtonPressed),
            button::menu_icon("Event", FluentIcon::Calendar).on_press(Message::EventButtonPressed),
            button::menu_icon("Group", FluentIcon::Group).on_press(Message::GroupButtonPressed),
        ])
        .style(style::container::overlay);

        page(
            "Ribbon",
            [widget_example(
                "A sample of Ribbon buttons.",
                row![
                    ribbon::split_button::large(
                        "New",
                        FluentIcon::Mail,
                        mail_button_flyout,
                        Message::MailButtonPressed,
                        Message::MailSplitButtonOpened,
                        Message::MailSplitButtonClosed,
                        self.mail_button_open
                    )
                    .width(100),
                    ribbon::separator(),
                    ribbon::button::large("Delete", FluentIcon::Delete)
                        .width(48)
                        .on_press(Message::DeleteButtonPressed),
                    column![
                        ribbon::button::medium("Cut", FluentIcon::Cut)
                            .on_press(Message::CutButtonPressed),
                        ribbon::button::medium("Copy", FluentIcon::Copy)
                            .on_press(Message::CopyButtonPressed),
                        ribbon::button::medium("Paste", FluentIcon::Paste)
                            .on_press(Message::PasteButtonPressed),
                    ],
                    ribbon::separator(),
                    ribbon::button::large("Reply", FluentIcon::Reply)
                        .on_press(Message::ReplyButtonPressed)
                        .width(44),
                    ribbon::button::large("Forward", FluentIcon::Forward)
                        .width(56)
                        .on_press(Message::ForwardButtonPressed),
                ]
                .height(80)
                .spacing(4),
                Some(text::body1(if let Some(button) = self.last_button {
                    format!("You clicked: {button}")
                } else {
                    String::new()
                })),
                None::<Element<Message>>,
            )],
        )
    }
}
