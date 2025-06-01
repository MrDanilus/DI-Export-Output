use std::path::PathBuf;
use file::check_image;
use iced::{event, Event, Subscription};

use crate::ui::pages::Page;

mod file;
mod exif;
mod params;

#[derive(Debug, Default)]
pub struct App {
    pub page: Page,

    pub file_dnd: DnDStatus,
    pub images: Vec<PathBuf>,

    pub selected_image: Option<PathBuf>,
    pub exif_metadata: Exif
}

#[derive(Debug, Default)]
pub enum Exif{
    Ok(String),
    Err(String),
    #[default]
    None
}

#[derive(Debug, Default)]
pub enum DnDStatus{
    Ok,
    Wrong,
    Exists,
    #[default]
    None
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangePage(Page),

    NewEvent(Event),
    SelectImage(Option<PathBuf>)
}

impl App {
    pub fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::NewEvent)
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChangePage(page) =>
                self.page = page,

            Message::NewEvent(event) => {
                match event{
                    Event::Window(event) => {
                        match event{
                            iced::window::Event::FileDropped(path) => {
                                if check_image(&path) == false{
                                    self.file_dnd = DnDStatus::Wrong;
                                    return;
                                }
                                if self.images.contains(&path){
                                    self.file_dnd = DnDStatus::Exists;
                                    return;
                                }

                                self.images.push(path);
                                self.file_dnd = DnDStatus::None;
                            },
                            iced::window::Event::FileHovered(path) => {
                                if self.images.contains(&path){
                                    self.file_dnd = DnDStatus::Exists;
                                    return;
                                }
                                match check_image(&path){
                                    true => self.file_dnd = DnDStatus::Ok,
                                    false => self.file_dnd = DnDStatus::Wrong
                                }
                            },
                            iced::window::Event::FilesHoveredLeft => 
                                self.file_dnd = DnDStatus::None,
                            _ => {}
                        }
                    },
                    _ => {}
                }
            },
            Message::SelectImage(path) => {
                if path.is_none(){
                    self.exif_metadata = Exif::None;
                    self.selected_image = None;
                    return;
                }
                self.exif_metadata = match exif::parse_image(&path.clone().unwrap()){
                    Ok(res) => Exif::Ok(res),
                    Err(err) => Exif::Err(err),
                };
                self.selected_image = path;
            }
        }
    }
}