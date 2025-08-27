use std::path::PathBuf;
use std::sync::Arc;

use iced::keyboard;
use iced::widget::{column, horizontal_space, pick_list, text_editor};
use iced::{Application, Command, Element, Length, Settings, Theme};

use crate::error::Error;
use crate::icons;
use crate::io;
use crate::ui;

#[derive(Debug, Clone)]
pub enum Message {
    Edit(text_editor::Action),
    Save,
    New,
    Open,
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    FileSaved(Result<PathBuf, Error>),
    ThemeSelected(iced::highlighter::Theme),
}

pub struct Editor {
    pub(crate) path: Option<PathBuf>,
    pub(crate) content: text_editor::Content,
    pub(crate) error: Option<Error>,
    pub(crate) theme: iced::highlighter::Theme,
    pub(crate) is_dirty: bool,
}

impl Application for Editor {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                path: None,
                content: text_editor::Content::new(),
                error: None,
                theme: iced::highlighter::Theme::SolarizedDark,
                is_dirty: true,
            },
            Command::perform(io::fs::load_file(crate::default_file()), Message::FileOpened),
        )
    }

    fn title(&self) -> String { "A Editor".into() }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        keyboard::on_key_press(|key_code, modifiers| {
            match key_code {
                keyboard::KeyCode::S if modifiers.command() => Some(Message::Save),
                _ => None,
            }
        })
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::New => {
                self.path = None;
                self.is_dirty = true;
                self.content = text_editor::Content::new();
                Command::none()
            }
            Message::Save => {
                let text = self.content.text();
                Command::perform(io::fs::save_file(self.path.clone(), text), Message::FileSaved)
            }
            Message::FileSaved(Ok(path)) => {
                self.is_dirty = false;
                self.path = Some(path);
                Command::none()
            }
            Message::FileSaved(Err(error)) => {
                self.error = Some(error);
                Command::none()
            }
            Message::Edit(action) => {
                self.is_dirty = self.is_dirty || action.is_edit();
                self.error = None;
                self.content.edit(action);
                Command::none()
            }
            Message::Open => Command::perform(io::fs::pick_file(), Message::FileOpened),
            Message::FileOpened(Ok((path, content))) => {
                self.is_dirty = false;
                self.path = Some(path);
                self.content = text_editor::Content::with(&content);
                Command::none()
            }
            Message::FileOpened(Err(error)) => {
                self.error = Some(error);
                Command::none()
            }
            Message::ThemeSelected(theme) => {
                self.theme = theme;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        ui::widgets::view(self)

    }

    fn theme(&self) -> Theme {
        if self.theme.is_dark() { Theme::Dark } else { Theme::Light }
    }
}
