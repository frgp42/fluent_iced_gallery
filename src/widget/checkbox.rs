use crate::{fluent_icon::FluentIcon, font, widget::Checkbox};

use iced::{
    widget::{
        checkbox::Icon,
        text::{LineHeight, Shaping},
    },
    Pixels,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ThreeState {
    Checked,
    Unchecked,
    Indeterminate,
}

impl ThreeState {
    pub fn toggle(&mut self) -> Self {
        match self {
            ThreeState::Checked => ThreeState::Indeterminate,
            ThreeState::Unchecked => ThreeState::Checked,
            ThreeState::Indeterminate => ThreeState::Unchecked,
        }
    }
}

fn base<'a, Message>(
    label: impl Into<String>,
    is_checked: bool,
    code_point: char,
) -> Checkbox<'a, Message> {
    let check_mark = Icon {
        font: font::SEGOE_FLUENT_ICONS,
        code_point,
        size: Some(Pixels(14.0)),
        line_height: LineHeight::default(),
        shaping: Shaping::Advanced,
    };

    Checkbox::new(label, is_checked)
        .size(20)
        .font(font::SEGOE)
        .text_size(14)
        .text_line_height(LineHeight::Absolute(Pixels(20.0)))
        .icon(check_mark)
}

pub fn two_state<'a, Message>(label: impl Into<String>, is_checked: bool) -> Checkbox<'a, Message> {
    base(label, is_checked, FluentIcon::CheckMark.codepoint())
}

pub fn three_state<'a, Message>(
    label: impl Into<String>,
    check_state: ThreeState,
) -> Checkbox<'a, Message> {
    match check_state {
        ThreeState::Checked => base(label, true, FluentIcon::CheckMark.codepoint()),
        ThreeState::Unchecked => base(label, false, FluentIcon::CheckMark.codepoint()),
        ThreeState::Indeterminate => base(label, true, FluentIcon::DashKey.codepoint()),
    }
}
