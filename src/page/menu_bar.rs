use crate::{
    page::{page, widget_example},
    style,
    widget::{button, menu, text, Element, Menu, MenuItem},
};

use std::fmt::{self, Display, Formatter};

use iced::{
    keyboard::{self, Key, Modifiers},
    Length, Subscription,
};

#[derive(Clone, Debug, Default)]
pub struct MenuBar {
    menu_bar1_action: Action,
    menu_bar2_action: Action,
    menu_bar3_action: Action,
    orientation: Orientation,
    icon_size: IconSize,
}

#[derive(Clone, Debug, Default)]
pub enum Action {
    #[default]
    None,
    New,
    Open,
    Save,
    Exit,
    PlainTextDocument,
    RichTextDocument,
    OtherFormats,
    Undo,
    Cut,
    Copy,
    Paste,
    Output,
    Landscape,
    Portait,
    SmallIcon,
    MediumIcon,
    LargeIcon,
    About,
}

impl Action {
    fn name(&self) -> &'static str {
        match self {
            Action::None => "",
            Action::New => "New",
            Action::Open => "Open",
            Action::Save => "Save",
            Action::Exit => "Exit",
            Action::PlainTextDocument => "Plain Text Document",
            Action::RichTextDocument => "Rich Text Document",
            Action::OtherFormats => "Other Formats",
            Action::Undo => "Undo",
            Action::Cut => "Cut",
            Action::Copy => "Copy",
            Action::Paste => "Paste",
            Action::Output => "Output",
            Action::Landscape => "Landscape",
            Action::Portait => "Portrait",
            Action::SmallIcon => "Small Icons",
            Action::MediumIcon => "Medium Icons",
            Action::LargeIcon => "Large Icons",
            Action::About => "About",
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Orientation {
    Landscape,
    #[default]
    Portait,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum IconSize {
    Small,
    #[default]
    Medium,
    Large,
}

#[derive(Clone, Debug)]
pub enum Message {
    NoOp,
    MenuBar1Selected(Action),
    MenuBar2Selected(Action),
    MenuBar3Selected(Action),
}

impl MenuBar {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::NoOp => (),
            Message::MenuBar1Selected(action) => self.menu_bar1_action = action,
            Message::MenuBar2Selected(action) => self.menu_bar2_action = action,
            Message::MenuBar3Selected(action) => {
                match action {
                    Action::Portait => self.orientation = Orientation::Portait,
                    Action::Landscape => self.orientation = Orientation::Landscape,
                    Action::SmallIcon => self.icon_size = IconSize::Small,
                    Action::MediumIcon => self.icon_size = IconSize::Medium,
                    Action::LargeIcon => self.icon_size = IconSize::Large,
                    _ => (),
                }

                self.menu_bar3_action = action
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        fn action_msg(a: Action) -> Option<Message> {
            Some(Message::MenuBar2Selected(a))
        }

        fn handle_keys(key: Key, modifiers: Modifiers) -> Option<Message> {
            if modifiers.control() {
                match key.as_ref() {
                    Key::Character("n") => action_msg(Action::New),
                    Key::Character("o") => action_msg(Action::Open),
                    Key::Character("s") => action_msg(Action::Save),
                    Key::Character("e") => action_msg(Action::Exit),
                    Key::Character("z") => action_msg(Action::Undo),
                    Key::Character("x") => action_msg(Action::Cut),
                    Key::Character("c") => action_msg(Action::Copy),
                    Key::Character("v") => action_msg(Action::Paste),
                    Key::Character("i") => action_msg(Action::About),
                    _ => None,
                }
            } else {
                None
            }
        }

        keyboard::on_key_press(handle_keys)
    }

    pub fn view(&self) -> Element<Message> {
        page(
            "MenuBar",
            [
                widget_example(
                    "A simple MenuBar.",
                    menu::bar::standard(vec![
                        menu_bar_item(
                            "File",
                            menu::standard(vec![
                                item1(Action::New),
                                item1(Action::Open),
                                item1(Action::Save),
                                item1(Action::Exit),
                            ]),
                        ),
                        menu_bar_item(
                            "Edit",
                            menu::standard(vec![
                                item1(Action::Undo),
                                item1(Action::Cut),
                                item1(Action::Copy),
                                item1(Action::Paste),
                            ]),
                        ),
                        menu_bar_item("Help", menu::standard(vec![item1(Action::About)])),
                    ])
                    .spacing(4.0),
                    Some(text::body1(match self.menu_bar1_action {
                        Action::None => String::from(""),
                        _ => format!("You clicked: {}", self.menu_bar1_action),
                    })),
                    None::<Element<Message>>,
                ),
                widget_example(
                    "A MenuBar with keyboard accelerators.",
                    menu::bar::standard(vec![
                        menu_bar_item(
                            "File",
                            menu::standard(vec![
                                item2(Action::New, "Ctrl+N"),
                                item2(Action::Open, "Ctrl+O"),
                                item2(Action::Save, "Ctrl+S"),
                                item2(Action::Exit, "Ctrl+E"),
                            ])
                            .width(120),
                        ),
                        menu_bar_item(
                            "Edit",
                            menu::standard(vec![
                                item2(Action::Undo, "Ctrl+Z"),
                                item2(Action::Cut, "Ctrl+X"),
                                item2(Action::Copy, "Ctrl+C"),
                                item2(Action::Paste, "Ctrl+V"),
                            ])
                            .width(120),
                        ),
                        menu_bar_item(
                            "Help",
                            menu::standard(vec![item2(Action::About, "Ctrl+I")]).width(120),
                        ),
                    ])
                    .spacing(4.0),
                    Some(text::body1(match self.menu_bar2_action {
                        Action::None => String::from(""),
                        _ => format!("You clicked: {}", self.menu_bar2_action),
                    })),
                    None::<Element<Message>>,
                ),
                widget_example(
                    "MenuBar with submenus, separators, and radio items.",
                    menu::bar::standard(vec![
                        menu_bar_item(
                            "File",
                            menu::standard(vec![
                                menu::item::submenu(
                                    "New",
                                    false,
                                    menu::standard(vec![
                                        item3(Action::PlainTextDocument),
                                        item3(Action::RichTextDocument),
                                        item3(Action::OtherFormats),
                                    ])
                                    .width(160)
                                    .offset(-2.0),
                                ),
                                item3(Action::Open),
                                item3(Action::Save),
                                menu::item::separator(),
                                item3(Action::Exit),
                            ]),
                        ),
                        menu_bar_item(
                            "Edit",
                            menu::standard(vec![
                                item3(Action::Undo),
                                item3(Action::Cut),
                                item3(Action::Copy),
                                item3(Action::Paste),
                            ])
                            .width(120),
                        ),
                        menu_bar_item(
                            "View",
                            menu::standard(vec![
                                menu::item::labelled(
                                    Action::Output.name(),
                                    true,
                                    None,
                                    Message::MenuBar3Selected(Action::Output),
                                ),
                                menu::item::separator(),
                                item3_radio(
                                    Action::Landscape,
                                    Orientation::Landscape,
                                    Some(self.orientation),
                                ),
                                item3_radio(
                                    Action::Portait,
                                    Orientation::Portait,
                                    Some(self.orientation),
                                ),
                                menu::item::separator(),
                                item3_radio(
                                    Action::SmallIcon,
                                    IconSize::Small,
                                    Some(self.icon_size),
                                ),
                                item3_radio(
                                    Action::MediumIcon,
                                    IconSize::Medium,
                                    Some(self.icon_size),
                                ),
                                item3_radio(
                                    Action::LargeIcon,
                                    IconSize::Large,
                                    Some(self.icon_size),
                                ),
                            ])
                            .width(150),
                        ),
                        menu_bar_item("Help", menu::standard(vec![item3(Action::About)])),
                    ])
                    .spacing(4.0),
                    Some(text::body1(match self.menu_bar3_action {
                        Action::None => String::from(""),
                        _ => format!("You clicked: {}", self.menu_bar3_action),
                    })),
                    None::<Element<Message>>,
                ),
            ],
        )
    }
}

fn menu_bar_item<'a>(label: &'a str, menu: Menu<'a, Message>) -> MenuItem<'a, Message> {
    MenuItem::with_menu(
        button::standard(text::body1(label))
            .width(Length::Shrink)
            .style(style::button::menu_item)
            // NoOp message required for button to look active
            .on_press(Message::NoOp),
        menu,
    )
}

fn item1<'a>(action: Action) -> MenuItem<'a, Message>
where
    Message: 'a + Clone,
{
    menu::item::labelled(
        action.name(),
        false,
        None,
        Message::MenuBar1Selected(action),
    )
}

fn item2<'a>(action: Action, accelerator: &'a str) -> MenuItem<'a, Message>
where
    Message: 'a + Clone,
{
    menu::item::labelled(
        action.name(),
        false,
        Some(accelerator),
        Message::MenuBar2Selected(action),
    )
}

fn item3<'a>(action: Action) -> MenuItem<'a, Message>
where
    Message: 'a + Clone,
{
    menu::item::labelled(
        action.name(),
        false,
        None,
        Message::MenuBar3Selected(action),
    )
}

fn item3_radio<'a, V>(action: Action, value: V, selected: Option<V>) -> MenuItem<'a, Message>
where
    Message: 'a + Clone,
    V: Copy + Eq,
{
    menu::item::radio(
        action.name(),
        value,
        selected,
        Message::MenuBar3Selected(action),
    )
}
