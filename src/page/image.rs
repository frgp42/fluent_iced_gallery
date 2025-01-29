use crate::{
    page::{page, widget_example},
    widget::{radio, text, Element},
};

use iced::{
    widget::{column, image},
    ContentFit,
};

#[derive(Clone, Debug, Default)]
pub struct Image {
    content_fit: ContentFit,
}

#[derive(Clone, Debug)]
pub enum Message {
    ContentFitChanged(ContentFit),
}

impl Image {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ContentFitChanged(content_fit) => self.content_fit = content_fit,
        }
    }

    pub fn view(&self) -> Element<Message> {
        page(
            "Image",
            [
                widget_example(
                    "A basic image from a local file.",
                    image(format! {
                        "{}/assets/images/treetops.jpg", env!("CARGO_MANIFEST_DIR")
                    })
                    .height(100),
                    None::<Element<Message>>,
                    None::<Element<Message>>,
                ),
                widget_example(
                    "Image stretching.",
                    image(format! {
                        "{}/assets/images/valley.jpg", env!("CARGO_MANIFEST_DIR")
                    })
                    .width(100)
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
