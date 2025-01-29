use crate::{
    page::{page, widget_example},
    widget::{radio, text, Element},
};

use iced::{
    widget::{column, svg},
    ContentFit, Length,
};

#[derive(Clone, Debug, Default)]
pub struct Svg {
    content_fit: ContentFit,
}

#[derive(Clone, Debug)]
pub enum Message {
    ContentFitChanged(ContentFit),
}

impl Svg {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ContentFitChanged(content_fit) => self.content_fit = content_fit,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let handle = svg::Handle::from_path(format! {
            "{}/assets/images/MirrorPCConsent.svg", env!("CARGO_MANIFEST_DIR")
        });

        page(
            "Svg",
            [
                widget_example(
                    "An SVG image.",
                    svg(handle.clone()).width(Length::Shrink).height(100),
                    None::<Element<Message>>,
                    None::<Element<Message>>,
                ),
                widget_example(
                    "SVG image stretching.",
                    svg(handle)
                        .width(Length::Shrink)
                        .height(100)
                        .content_fit(self.content_fit),
                    None::<Element<Message>>,
                    Some(
                        column![
                            text::body1("Image content fit"),
                            radio::standard(
                                "Contain",
                                ContentFit::Contain,
                                Some(self.content_fit),
                                Message::ContentFitChanged
                            ),
                            radio::standard(
                                "Cover",
                                ContentFit::Cover,
                                Some(self.content_fit),
                                Message::ContentFitChanged
                            ),
                            radio::standard(
                                "Fill",
                                ContentFit::Fill,
                                Some(self.content_fit),
                                Message::ContentFitChanged
                            ),
                            radio::standard(
                                "None",
                                ContentFit::None,
                                Some(self.content_fit),
                                Message::ContentFitChanged
                            ),
                            radio::standard(
                                "ScaleDown",
                                ContentFit::ScaleDown,
                                Some(self.content_fit),
                                Message::ContentFitChanged
                            ),
                        ]
                        .spacing(16),
                    ),
                ),
            ],
        )
    }
}
