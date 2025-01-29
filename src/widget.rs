pub mod button;
pub mod canvas;
pub mod checkbox;
pub mod combo_box;
pub mod dialog;
pub mod menu;
pub mod number_input;
pub mod pick_list;
pub mod quad;
pub mod radio;
pub mod ribbon;
pub mod scrollable;
pub mod side_nav;
pub mod slider;
pub mod text;
pub mod text_input;
pub mod toggler;
pub mod underline;

use crate::theme::Theme;

type Renderer = iced::Renderer;
pub type Button<'a, Message> = iced::widget::Button<'a, Message, Theme, Renderer>;
pub type Canvas<P, Message> = iced::widget::Canvas<P, Message, Theme, Renderer>;
pub type Checkbox<'a, Message> = iced::widget::Checkbox<'a, Message, Theme, Renderer>;
pub type Column<'a, Message> = iced::widget::Column<'a, Message, Theme, Renderer>;
pub type ComboBox<'a, T, Message> = iced::widget::ComboBox<'a, T, Message, Theme, Renderer>;
pub type Container<'a, Message> = iced::widget::Container<'a, Message, Theme, Renderer>;
pub type DropDown<'a, Message> = iced_aw::widget::DropDown<'a, Message, Theme, Renderer>;
pub type Element<'a, Message> = iced::Element<'a, Message, Theme, Renderer>;
pub type Menu<'a, Message> = iced_aw::menu::Menu<'a, Message, Theme, Renderer>;
pub type MenuBar<'a, Message> = iced_aw::menu::MenuBar<'a, Message, Theme, Renderer>;
pub type MenuItem<'a, Message> = iced_aw::menu::Item<'a, Message, Theme, Renderer>;
pub type PickList<'a, T, L, V, Message> =
    iced::widget::PickList<'a, T, L, V, Message, Theme, Renderer>;
pub type Quad<'a, Theme> = crate::widget::quad::Quad<'a, Theme>;
pub type Radio<'a, Message> = iced::widget::Radio<'a, Message, Theme, Renderer>;
pub type Row<'a, Message> = iced::widget::Row<'a, Message, Theme, Renderer>;
pub type Scrollable<'a, Message> = iced::widget::Scrollable<'a, Message, Theme, Renderer>;
pub type Slider<'a, T, Message> = iced::widget::Slider<'a, T, Message, Theme>;
pub type Text<'a> = iced::widget::Text<'a, Theme, Renderer>;
pub type TextInput<'a, Message> = iced::widget::TextInput<'a, Message, Theme, Renderer>;
pub type Toggler<'a, Message> = iced::widget::Toggler<'a, Message, Theme, Renderer>;
pub type Wrap<'a, Message, Direction> =
    iced_aw::widget::Wrap<'a, Message, Direction, Theme, Renderer>;
