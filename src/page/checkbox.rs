use crate::{
    page::{page, widget_example},
    widget::{
        checkbox::{self, ThreeState},
        text, Element,
    },
};

use iced::{widget::column, Padding};

#[derive(Clone, Debug)]
pub struct Checkbox {
    checkbox1_checked: Option<bool>,
    checkbox2_state: Option<checkbox::ThreeState>,
    group_parent_state: checkbox::ThreeState,
    group_option1_checked: bool,
    group_option2_checked: bool,
    group_option3_checked: bool,
}

impl Default for Checkbox {
    fn default() -> Self {
        Self {
            checkbox1_checked: None,
            checkbox2_state: None,
            group_parent_state: ThreeState::Indeterminate,
            group_option1_checked: false,
            group_option2_checked: true,
            group_option3_checked: false,
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Debug)]
pub enum Message {
    Checkbox1Toggled(bool),
    Checkbox2Toggled,
    CheckboxParentToggled,
    Option1Toggled(bool),
    Option2Toggled(bool),
    Option3Toggled(bool),
}

impl Checkbox {
    fn group_parent_update(&mut self) {
        if self.group_option1_checked && self.group_option2_checked && self.group_option3_checked {
            self.group_parent_state = ThreeState::Checked;
        } else if !self.group_option1_checked
            && !self.group_option2_checked
            && !self.group_option3_checked
        {
            self.group_parent_state = ThreeState::Unchecked;
        } else {
            self.group_parent_state = ThreeState::Indeterminate;
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Checkbox1Toggled(checked) => self.checkbox1_checked = Some(checked),
            Message::Checkbox2Toggled => match self.checkbox2_state {
                None => self.checkbox2_state = Some(ThreeState::Checked),
                Some(mut state) => self.checkbox2_state = Some(state.toggle()),
            },
            Message::CheckboxParentToggled => {
                let group_checked = match self.group_parent_state {
                    ThreeState::Checked | ThreeState::Indeterminate => false,
                    ThreeState::Unchecked => true,
                };

                self.group_parent_state = if group_checked {
                    ThreeState::Checked
                } else {
                    ThreeState::Unchecked
                };
                self.group_option1_checked = group_checked;
                self.group_option2_checked = group_checked;
                self.group_option3_checked = group_checked;
            }
            Message::Option1Toggled(checked) => {
                self.group_option1_checked = checked;
                self.group_parent_update();
            }
            Message::Option2Toggled(checked) => {
                self.group_option2_checked = checked;
                self.group_parent_update();
            }
            Message::Option3Toggled(checked) => {
                self.group_option3_checked = checked;
                self.group_parent_update();
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        page(
            "Checkbox",
            [
                widget_example(
                    "A 2-state Checkbox.",
                    checkbox::two_state(
                        "Two-state Checkbox",
                        self.checkbox1_checked.unwrap_or_default(),
                    )
                    .on_toggle(Message::Checkbox1Toggled),
                    Some(text::body1(format![
                        "Output: {}",
                        match self.checkbox1_checked {
                            Some(false) => "\nYou unchecked the box.",
                            Some(true) => "\nYou checked the box.",
                            None => "",
                        }
                    ])),
                    None::<Element<Message>>,
                ),
                widget_example(
                    "A 3-state Checkbox.",
                    checkbox::three_state(
                        "Three-state Checkbox",
                        self.checkbox2_state.unwrap_or(ThreeState::Unchecked),
                    )
                    .on_toggle(|_| Message::Checkbox2Toggled),
                    Some(text::body1(format![
                        "Output: {}",
                        match self.checkbox2_state {
                            Some(ThreeState::Unchecked) => "\nCheckbox is unchecked.",
                            Some(ThreeState::Checked) => "\nCheckbox is checked.",
                            Some(ThreeState::Indeterminate) => "\nCheckbox is indeterminant.",
                            None => "",
                        }
                    ])),
                    None::<Element<Message>>,
                ),
                widget_example(
                    "Using exclusive Checkboxes.",
                    column![
                        checkbox::three_state("Select All", self.group_parent_state)
                            .on_toggle(|_| Message::CheckboxParentToggled),
                        column![
                            checkbox::two_state("Option 1", self.group_option1_checked)
                                .on_toggle(Message::Option1Toggled),
                            checkbox::two_state("Option 2", self.group_option2_checked)
                                .on_toggle(Message::Option2Toggled),
                            checkbox::two_state("Option 3", self.group_option3_checked)
                                .on_toggle(Message::Option3Toggled),
                        ]
                        .padding(Padding::default().left(24))
                        .spacing(12),
                    ]
                    .spacing(12),
                    None::<Element<Message>>,
                    None::<Element<Message>>,
                ),
            ],
        )
    }
}
