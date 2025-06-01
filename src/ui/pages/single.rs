use iced::{alignment::{Horizontal, Vertical}, border::Radius, widget::{
    self, button, column, container, row, scrollable, text, Column
}, Background, Border, Color, Element, Length, Padding};

use crate::core::{App, DnDStatus, Exif, Message};

pub fn view(app: &App) -> Element<Message>{
    let status = match &app.file_dnd{
        DnDStatus::Ok => {
            Color::parse("#0f0").unwrap()
        },
        DnDStatus::Wrong => {
            Color::parse("#f00").unwrap()
        },
        DnDStatus::Exists => {
            Color::parse("#ffa500").unwrap()
        },
        _ => {
            Color::TRANSPARENT
        }
    };
    let files = Column::from_vec(app.images.iter().map(|file|
        button(
            text(file.file_name().unwrap().to_str().unwrap())
        ).style(|_, st| button::Style{
            background: match st {
                button::Status::Hovered => 
                    Some(Background::Color(Color::parse("#333").unwrap())),
                _ => None
            },
            text_color: Color::WHITE,
            ..Default::default()
        }).width(Length::Fill)
        .on_press(Message::SelectImage(Some(file.clone())))
        .into()
    ).collect::<Vec<_>>());

    container(
        column![
            row![
                container(
                    if app.images.is_empty(){
                        container(
                            text("Перенесите изображения в окно")
                                .color(Color::WHITE)
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .align_x(Horizontal::Center)
                                .align_y(Vertical::Center)
                        )
                    } else{
                        container(
                            scrollable(
                                files
                                .padding(Padding::from(4))
                                .width(Length::Fill)
                            )
                        )
                        .height(Length::Fill)
                    }
                )
                .style(move |_| container::Style { 
                    border: Border{
                        color: Color::WHITE,
                        width: 1.0,
                        radius: Radius::from(0)
                    },
                    background: Some(Background::Color(status.scale_alpha(0.01))),
                    ..Default::default()
                }).width(Length::FillPortion(1))
                .height(Length::FillPortion(1)),
                container(
                    if app.selected_image.is_some(){
                        container(
                            button(
                                widget::image(app.selected_image.clone().unwrap())
                            ).on_press(Message::SelectImage(None))
                            .style(|_, st| button::Style{
                                background: match st {
                                    button::Status::Hovered => 
                                        Some(Background::Color(Color::parse("#333").unwrap())),
                                    _ => None
                                },
                                ..Default::default()
                            }).padding(Padding::from(4))
                        ).width(Length::Fill)
                        .height(Length::Fill)
                        .align_y(Vertical::Center)
                        .align_x(Horizontal::Center)
                    } else{
                        container(
                            text("Выберите изображение")
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .align_x(Horizontal::Center)
                                .align_y(Vertical::Center)
                        )
                    }
                ).padding(Padding::from(4))
                .style(move |_| container::Style { 
                    border: Border{
                        color: Color::WHITE,
                        width: 1.0,
                        radius: Radius::from(0)
                    },
                    ..Default::default()
                }).width(Length::FillPortion(1))
                .height(Length::FillPortion(1))
            ],
            container(
                if app.selected_image.is_some(){
                    container(
                        match &app.exif_metadata{
                            Exif::Ok(res) => 
                                text(res),
                            Exif::Err(err) => 
                                text(err),
                            _ => text("Нет данных")
                        }
                    ).width(Length::Fill)
                    .height(Length::Fill)
                } else{
                    container(
                        text("Выберите изображение")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .align_x(Horizontal::Center)
                            .align_y(Vertical::Center)
                    )
                }
            ).padding(Padding::from(4))
            .style(move |_| container::Style { 
                border: Border{
                    color: Color::WHITE,
                    width: 1.0,
                    radius: Radius::from(0)
                },
                ..Default::default()
            }).width(Length::Fill)
            .height(Length::FillPortion(1))
        ]
    ).padding(Padding::from(4))
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}