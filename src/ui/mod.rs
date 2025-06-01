use iced::{alignment::Horizontal, widget::{button, column, container, row, text}, Background, Color, Element, Length};
use pages::Page;

use crate::{core::Message, App};

pub mod pages;

impl App {
    pub fn view(&self) -> Element<Message> {
        container(
            column![
                row![
                    button(
                        text("Single")
                        .align_x(Horizontal::Center)
                    ).on_press(Message::ChangePage(Page::Single))
                        .width(Length::Fill),
                    button(
                        text("Bulk")
                        .align_x(Horizontal::Center)
                    ).on_press(Message::ChangePage(Page::Bulk))
                        .width(Length::Fill)
                ].width(Length::Fill)
                .spacing(4)
                .padding(4),
                match self.page {
                    Page::Single => pages::single::view(self),
                    Page::Bulk => todo!()
                }
            ]
        ).width(Length::Fill)
        .height(Length::Fill)
        .style(|_|
            container::Style{
                text_color: Some(Color::WHITE),
                background: Some(Background::Color(Color::BLACK)),
                ..Default::default()
            }
        ).into()
    }
}