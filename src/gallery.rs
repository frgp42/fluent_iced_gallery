use crate::{
    fluent_icon::FluentIcon,
    page::{self, page},
    style,
    theme::Theme,
    widget::{pick_list, side_nav, text, toggler, Container, Element},
};

use std::mem::discriminant;

use iced::{
    alignment::Vertical,
    border,
    widget::{
        center, column, container, horizontal_space, iced, image, mouse_area, opaque, row, stack,
    },
    window, ContentFit, Length, Size, Subscription,
};

#[derive(Clone, Debug, Default)]
pub enum Page {
    #[default]
    Home,

    // Basic input
    Button(page::button::Button),
    DropDownButton(page::drop_down_button::DropDownButton),
    Checkbox(page::checkbox::Checkbox),
    ToggleButton(page::toggle_button::ToggleButton),
    SplitButton(page::split_button::SplitButton),
    ComboBox(page::combo_box::ComboBox),
    Radio(page::radio::Radio),
    Slider(page::slider::Slider),
    Toggler(page::toggler::Toggler),

    // Dialogs and flyouts
    Dialog(page::dialog::Dialog),

    // Layout
    RowColumn(page::row_column::RowColumn),

    // Media
    Image(page::image::Image),
    Svg(page::svg::Svg),

    // Menu & toolbars
    AppBarButton(page::app_bar_button::AppBarButton),
    MenuBar(page::menu_bar::MenuBar),
    Ribbon(page::ribbon::Ribbon),

    // Text
    TextInput(page::text_input::TextInput),

    Settings,
}

pub struct Gallery {
    current_page: Page,
    side_nav_display_mode: side_nav::DisplayMode,
    pages: Vec<PageGroup>,
    footer_pages: Vec<PageGroup>,
    page_group_overlay_open: Option<&'static str>,
    window_size: Size,
    theme: Theme,
    explain: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    PageSelected(Page),
    PageGroupToggled(&'static str),
    PageGroupOverlayDismissed,
    SideNavDisplayModeToggled,
    WindowResized((window::Id, Size)),
    ThemeToggled,
    ExplainToggled,

    // Basic input
    ButtonPage(page::button::Message),
    DropDownButtonPage(page::drop_down_button::Message),
    ToggleButtonPage(page::toggle_button::Message),
    SplitButtonPage(page::split_button::Message),
    CheckboxPage(page::checkbox::Message),
    ComboBoxPage(page::combo_box::Message),
    SliderPage(page::slider::Message),
    TogglerPage(page::toggler::Message),

    // Dialogs and flyouts
    DialogPage(page::dialog::Message),

    // Layout
    RowColumnPage(page::row_column::Message),

    // Media
    ImagePage(page::image::Message),
    SvgPage(page::svg::Message),

    // Menu & toolbars
    AppBarButton(page::app_bar_button::Message),
    MenuBar(page::menu_bar::Message),
    Ribbon(page::ribbon::Message),

    // Text
    TextInputPage(page::text_input::Message),
    RadioPage(page::radio::Message),
}

const SIDE_NAV_COMPACT_WIDTH: f32 = 1000.0;

impl Gallery {
    pub fn update(&mut self, message: Message) {
        match (message, &mut self.current_page) {
            (Message::PageSelected(page), _) => {
                self.select_page(page);
            }
            (Message::PageGroupToggled(label), _) => {
                if let Some(page_group) = self
                    .pages
                    .iter_mut()
                    .find(|page_group| page_group.label == label)
                {
                    if self.side_nav_display_mode == side_nav::DisplayMode::Compact
                        || self.window_size.width < SIDE_NAV_COMPACT_WIDTH
                    {
                        self.page_group_overlay_open = Some(page_group.label);
                    } else if self.side_nav_display_mode == side_nav::DisplayMode::Full {
                        page_group.expanded = !page_group.expanded;
                    }
                }
            }
            (Message::PageGroupOverlayDismissed, _) => self.page_group_overlay_open = None,
            (Message::SideNavDisplayModeToggled, _) => {
                self.side_nav_display_mode = match self.side_nav_display_mode {
                    side_nav::DisplayMode::Compact => side_nav::DisplayMode::Full,
                    side_nav::DisplayMode::Full => side_nav::DisplayMode::Compact,
                }
            }
            (Message::WindowResized((_, size)), _) => {
                self.window_size = size;

                if size.width < SIDE_NAV_COMPACT_WIDTH {
                    self.page_group_overlay_open = None;
                }
            }

            (Message::ThemeToggled, _) => {
                self.theme = match self.theme {
                    Theme::Light => Theme::Dark,
                    Theme::Dark => Theme::Light,
                }
            }
            (Message::ExplainToggled, _) => self.explain = !self.explain,

            // Page messages
            // Basic input
            (Message::ButtonPage(message), Page::Button(page)) => page.update(message),
            (Message::DropDownButtonPage(message), Page::DropDownButton(page)) => {
                page.update(message)
            }
            (Message::ToggleButtonPage(message), Page::ToggleButton(page)) => page.update(message),
            (Message::SplitButtonPage(message), Page::SplitButton(page)) => page.update(message),
            (Message::CheckboxPage(message), Page::Checkbox(page)) => page.update(message),
            (Message::ComboBoxPage(message), Page::ComboBox(page)) => page.update(message),
            (Message::RadioPage(message), Page::Radio(page)) => page.update(message),
            (Message::SliderPage(message), Page::Slider(page)) => page.update(message),
            (Message::TogglerPage(message), Page::Toggler(page)) => page.update(message),

            // Dialogs and flyouts
            (Message::DialogPage(message), Page::Dialog(page)) => page.update(message),

            // Layout
            (Message::RowColumnPage(message), Page::RowColumn(page)) => page.update(message),

            // Media
            (Message::ImagePage(message), Page::Image(page)) => page.update(message),
            (Message::SvgPage(message), Page::Svg(page)) => page.update(message),

            // Menu & toolbars
            (Message::AppBarButton(message), Page::AppBarButton(page)) => page.update(message),
            (Message::MenuBar(message), Page::MenuBar(page)) => page.update(message),
            (Message::Ribbon(message), Page::Ribbon(page)) => page.update(message),

            // Text
            (Message::TextInputPage(message), Page::TextInput(page)) => page.update(message),

            _ => panic!("Message, Page pair not valid."),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let window_resize_sub = iced::window::resize_events().map(Message::WindowResized);

        let menu_bar_sub = match &self.current_page {
            Page::MenuBar(page) => page.subscription().map(Message::MenuBar),
            _ => Subscription::none(),
        };

        iced::Subscription::batch([window_resize_sub, menu_bar_sub])
    }

    pub fn view(&self) -> Element<Message> {
        let nav_items = |pages: &[PageGroup]| -> Vec<side_nav::Group<Message>> {
            pages
                .iter()
                .map(|group| {
                    let items = group.page_items.iter().map(|item| {
                        side_nav::Item::new(
                            None,
                            item.label,
                            Message::PageSelected(item.page.clone()),
                        )
                    });

                    let expanded = if self.side_nav_display_mode == side_nav::DisplayMode::Compact
                        || self.window_size.width < SIDE_NAV_COMPACT_WIDTH
                    {
                        self.page_group_overlay_open == Some(group.label)
                    } else if self.side_nav_display_mode == side_nav::DisplayMode::Full {
                        group.expanded
                    } else {
                        false
                    };

                    let group_message = if group.page_items.is_empty() {
                        Message::PageSelected(group.page.clone().expect("Group should have page"))
                    } else {
                        Message::PageGroupToggled(group.label)
                    };

                    side_nav::Group::new(group.icon, group.label, group_message)
                        .with_items(items)
                        .expanded(expanded)
                        .overlay_width(group.overlay_width)
                        .on_overlay_dismiss(Message::PageGroupOverlayDismissed)
                })
                .collect()
        };

        let page_items = nav_items(&self.pages);
        let footer_page_items = nav_items(&self.footer_pages);

        let display_mode = if self.side_nav_display_mode == side_nav::DisplayMode::Full
            && self.window_size.width > SIDE_NAV_COMPACT_WIDTH
        {
            side_nav::DisplayMode::Full
        } else {
            side_nav::DisplayMode::Compact
        };

        let side_nav = side_nav::SideNav::new(display_mode, Message::SideNavDisplayModeToggled)
            .with_groups(page_items)
            .with_footer_groups(footer_page_items);

        let padding = match self.current_page {
            Page::Home => 0,
            _ => 24,
        };

        let page = Container::new(match &self.current_page {
            Page::Home => self.home_page_view(),

            // Basic input
            Page::Button(page) => page.view().map(Message::ButtonPage),
            Page::DropDownButton(page) => page.view().map(Message::DropDownButtonPage),
            Page::ToggleButton(page) => page.view().map(Message::ToggleButtonPage),
            Page::SplitButton(page) => page.view().map(Message::SplitButtonPage),
            Page::Checkbox(page) => page.view().map(Message::CheckboxPage),
            Page::ComboBox(page) => page.view().map(Message::ComboBoxPage),
            Page::Radio(page) => page.view().map(Message::RadioPage),
            Page::Slider(page) => page.view().map(Message::SliderPage),
            Page::Toggler(page) => page.view().map(Message::TogglerPage),

            // Dialogs and flyouts
            Page::Dialog(page) => page.view().map(Message::DialogPage),

            // Layout
            Page::RowColumn(page) => page.view().map(Message::RowColumnPage),

            // Media
            Page::Image(page) => page.view().map(Message::ImagePage),
            Page::Svg(page) => page.view().map(Message::SvgPage),

            // Menu & toolbars
            Page::AppBarButton(page) => page.view().map(Message::AppBarButton),
            Page::MenuBar(page) => page.view().map(Message::MenuBar),
            Page::Ribbon(page) => page.view().map(Message::Ribbon),

            // Text
            Page::TextInput(page) => page.view().map(Message::TextInputPage),

            Page::Settings => self.settings_page_view(),
        })
        .style(|theme| {
            container::Style::default()
                .background(theme.palette().solid_background_fill_color_tertiary)
                .border(border::rounded(8))
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(padding);

        let content = Element::new(container(row![side_nav, page,].spacing(4)).padding(4));

        let view = if let Page::Dialog(page) = &self.current_page {
            if page.is_dialog_open() {
                modal(content, page.dialog().map(Message::DialogPage))
            } else {
                content
            }
        } else {
            content
        };

        if self.explain {
            view.explain(self.theme.palette().control_strong_stroke_color_default)
        } else {
            view
        }
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn select_page(&mut self, page: Page) {
        self.page_group_overlay_open = None;

        if discriminant(&self.current_page) != discriminant(&page) {
            self.current_page = page
        }
    }

    fn home_page_view(&self) -> Element<Message> {
        let image_path = match self.theme {
            Theme::Light => "/assets/images/bloom_light.jpg",
            Theme::Dark => "/assets/images/bloom_dark.jpg",
        };

        stack![
            image(format! {
                "{}{}", env!("CARGO_MANIFEST_DIR"), image_path
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .content_fit(ContentFit::Cover),
            container(iced(80)).padding(24),
        ]
        .into()
    }

    fn settings_page_view(&self) -> Element<Message> {
        fn setting_card<'a>(
            icon: char,
            label: &'static str,
            control: Element<'a, Message>,
        ) -> Element<'a, Message> {
            Container::new(
                row![
                    text::icon(icon).size(20),
                    text::body1(label),
                    horizontal_space(),
                    control,
                ]
                .height(Length::Fill)
                .spacing(20)
                .align_y(Vertical::Center),
            )
            .width(Length::Fill)
            .height(68)
            .padding(20)
            .style(style::container::card)
            .into()
        }

        let theme_widget = pick_list::standard(
            [Theme::Light, Theme::Dark],
            Some(self.theme.clone()),
            |_| Message::ThemeToggled,
        )
        .into();

        let explain_widget = toggler::standard(self.explain)
            .label(if self.explain { "On" } else { "Off" })
            .on_toggle(|_| Message::ExplainToggled)
            .into();

        page(
            "Settings",
            [column![
                setting_card(FluentIcon::Color.codepoint(), "Theme", theme_widget),
                setting_card(
                    FluentIcon::PageMarginLandscapeNormal.codepoint(),
                    "Explain",
                    explain_widget
                ),
            ]
            .spacing(4)
            .into()],
        )
    }
}

impl Default for Gallery {
    fn default() -> Self {
        Gallery {
            current_page: Page::default(),
            pages: pages(),
            footer_pages: footer_pages(),
            page_group_overlay_open: None,
            side_nav_display_mode: side_nav::DisplayMode::Full,
            window_size: Size::default(),
            theme: <Theme as Default>::default(),
            explain: false,
        }
    }
}

struct PageGroup {
    icon: char,
    label: &'static str,
    expanded: bool,
    overlay_width: Length,
    page: Option<Page>,
    page_items: Vec<PageItem>,
}

struct PageItem {
    label: &'static str,
    page: Page,
}

fn pages() -> Vec<PageGroup> {
    vec![
        PageGroup {
            icon: FluentIcon::Home.codepoint(),
            label: "Home",
            expanded: false,
            overlay_width: 100.into(),
            page: Some(Page::Home),
            page_items: Vec::new(),
        },
        PageGroup {
            icon: FluentIcon::CheckboxComposite.codepoint(),
            label: "Basic input",
            expanded: false,
            overlay_width: 160.into(),
            page: None,
            page_items: vec![
                PageItem {
                    label: "Button",
                    page: Page::Button(page::button::Button::default()),
                },
                PageItem {
                    label: "Drop Down Button",
                    page: Page::DropDownButton(page::drop_down_button::DropDownButton::default()),
                },
                PageItem {
                    label: "Toggle Button",
                    page: Page::ToggleButton(page::toggle_button::ToggleButton::default()),
                },
                PageItem {
                    label: "Split Button",
                    page: Page::SplitButton(page::split_button::SplitButton::default()),
                },
                PageItem {
                    label: "Checkbox",
                    page: Page::Checkbox(page::checkbox::Checkbox::default()),
                },
                PageItem {
                    label: "ComboBox",
                    page: Page::ComboBox(page::combo_box::ComboBox::default()),
                },
                PageItem {
                    label: "Radio",
                    page: Page::Radio(page::radio::Radio::default()),
                },
                PageItem {
                    label: "Slider",
                    page: Page::Slider(page::slider::Slider::default()),
                },
                PageItem {
                    label: "Toggler",
                    page: Page::Toggler(page::toggler::Toggler::default()),
                },
            ],
        },
        PageGroup {
            icon: FluentIcon::Message.codepoint(),
            label: "Dialogs and flyouts",
            expanded: false,
            overlay_width: 120.into(),
            page: None,
            page_items: vec![PageItem {
                label: "Dialog",
                page: Page::Dialog(page::dialog::Dialog::default()),
            }],
        },
        PageGroup {
            icon: FluentIcon::PreviewLink.codepoint(),
            label: "Layout",
            expanded: false,
            overlay_width: 120.into(),
            page: None,
            page_items: vec![PageItem {
                label: "Row & Column",
                page: Page::RowColumn(page::row_column::RowColumn::default()),
            }],
        },
        PageGroup {
            icon: FluentIcon::Slideshow.codepoint(),
            label: "Media",
            expanded: false,
            overlay_width: 120.into(),
            page: None,
            page_items: vec![
                PageItem {
                    label: "Image",
                    page: Page::Image(page::image::Image::default()),
                },
                PageItem {
                    label: "Svg",
                    page: Page::Svg(page::svg::Svg::default()),
                },
            ],
        },
        PageGroup {
            icon: FluentIcon::Save.codepoint(),
            label: "Menu & toolbars",
            expanded: false,
            overlay_width: 120.into(),
            page: None,
            page_items: vec![
                PageItem {
                    label: "AppBar Button",
                    page: Page::AppBarButton(page::app_bar_button::AppBarButton::default()),
                },
                PageItem {
                    label: "MenuBar",
                    page: Page::MenuBar(page::menu_bar::MenuBar::default()),
                },
                PageItem {
                    label: "Ribbon",
                    page: Page::Ribbon(page::ribbon::Ribbon::default()),
                },
            ],
        },
        PageGroup {
            icon: FluentIcon::Font.codepoint(),
            label: "Text",
            expanded: false,
            overlay_width: 120.into(),
            page: None,
            page_items: vec![PageItem {
                label: "Text Input",
                page: Page::TextInput(page::text_input::TextInput::default()),
            }],
        },
    ]
}

fn footer_pages() -> Vec<PageGroup> {
    vec![PageGroup {
        icon: FluentIcon::Settings.codepoint(),
        label: "Settings",
        expanded: false,
        overlay_width: 100.into(),
        page: Some(Page::Settings),
        page_items: Vec::new(),
    }]
}

fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message>
where
    Message: 'a + Clone,
{
    let area = mouse_area(
        center(opaque(content)).style(|theme| iced::widget::container::Style {
            background: Some(theme.palette().smoke_fill_color_default.into()),
            ..iced::widget::container::Style::default()
        }),
    );

    stack![base.into(), opaque(area),].into()
}
