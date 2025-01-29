use crate::{
    page::{page, widget_example},
    widget::{button, checkbox, dialog, text, Element},
};

use iced::widget::column;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Dialog {
    is_checked: bool,
    action: Action,
    dialog_open: bool,
}

impl Dialog {
    pub fn is_dialog_open(&self) -> bool {
        self.dialog_open
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    Opened,
    Toggled(bool),
    Action(Action),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Action {
    #[default]
    None,
    Saved,
    NotSaved,
    Cancelled,
}

impl Dialog {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Opened => self.dialog_open = true,
            Message::Toggled(toggled) => self.is_checked = toggled,
            Message::Action(action) => {
                self.action = action;
                self.dialog_open = false;
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        page(
            "Dialog",
            [widget_example(
                "A basic dialog with content.",
                button::standard(text::body1("Show dialog")).on_press(Message::Opened),
                Some(text::body1(match self.action {
                    Action::None => "",
                    Action::Saved => "User saved their work",
                    Action::NotSaved => "User did not save their work",
                    Action::Cancelled => "User cancelled the dialog",
                })),
                None::<Element<Message>>,
            )],
        )
    }

    pub fn dialog(&self) -> Element<'static, Message>
    where
        Message: Clone,
    {
        dialog::content(
            Some("Save your work?"),
            column![
                text::body1("Lorem ipsum dolor sit amet, adipisicing elit."),
                checkbox::two_state("Upload your content to the cloud", self.is_checked)
                    .on_toggle(Message::Toggled)
            ]
            .spacing(4),
            Some(dialog::Button {
                text: String::from("Save"),
                on_press: Message::Action(Action::Saved),
            }),
            Some(dialog::Button {
                text: String::from("Don't Save"),
                on_press: Message::Action(Action::NotSaved),
            }),
            dialog::Button {
                text: String::from("Cancel"),
                on_press: Message::Action(Action::Cancelled),
            },
        )
        .into()
    }
}
