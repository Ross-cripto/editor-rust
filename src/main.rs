use std::ops::RangeFull;
use std::path::PathBuf;
use std::{default, path};
use std::{io, path::Path, sync::Arc};

use iced::theme::{self, Container};
use iced::{keyboard, Font};
use iced::widget::{button, column, tooltip};
use iced::{
    Application, Command, Element, Length, Settings, Theme,
    widget::{container, horizontal_space, row, pick_list, text, text_editor},
};
use iced::highlighter::{self, Highlight, Highlighter};

pub fn main() -> iced::Result {
    Editor::run(Settings {
        default_font: Font::MONOSPACE,
        fonts: vec![include_bytes!("./fonts/editor.ttf").as_slice().into()],
        ..Settings::default()
    })
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    Save,
    New,
    Open,
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    FileSaved(Result<PathBuf, Error>),
    ThemeSelected(highlighter::Theme),
}

struct Editor {
    path: Option<PathBuf>,
    content: text_editor::Content,
    error: Option<Error>,
    theme: highlighter::Theme,
    is_dirty: bool,
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
                theme: highlighter::Theme::SolarizedDark,
                is_dirty: true,
            },
            Command::perform(load_file(default_file()), Message::FileOpened),
        )
    }

    fn title(&self) -> String {
        String::from("A Editor")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        keyboard::on_key_press(|key_code, modifiers| {
            match key_code {
                keyboard::KeyCode::S if modifiers.command() => Some(Message::Save), _ => None,
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
                Command::perform(save_file(self.path.clone(), text), Message::FileSaved)
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
            Message::Open => Command::perform(pick_file(), Message::FileOpened),
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
        let controls = row![
            action(open_icon(), Some(Message::Open), "Open a new File"),
            action(new_icon(), Some(Message::New), "Create a new File"),
            action(save_icon(), self.is_dirty.then_some(Message::Save), "Save the current File"),
            horizontal_space(Length::Fill),
            pick_list(highlighter::Theme::ALL, Some(self.theme), Message::ThemeSelected)
        ]
        .spacing(10);

        let input:  Element<'_, Message> = text_editor(&self.content)
            .on_edit(Message::Edit)
            .highlight::<Highlighter>(
                highlighter::Settings {
                theme: self.theme,
                extension: self
                .path
                .as_ref()
                .and_then(|path| path.extension()?.to_str())
                .unwrap_or("rs")
                .to_string(),
            }, |highlight, theme| highlight.to_format()).into();

        let status_bar = {
            let file_path = if let Some(Error::IO(error)) = self.error.as_ref() {
                text(error.to_string()).style(iced::Color::from_rgb(1.0, 0.0, 0.0))
            } else {
                match self.path.as_deref().and_then(Path::to_str) {
                    Some(path) => text(path).size(20),
                    None => text("No file opened").size(14),
                }
            };

            let position = {
                let (line, column) = self.content.cursor_position();
                text(format!("Ln {}, Col {}", line, column))
            };

            row!(file_path, horizontal_space(Length::Fill), position)
        };

        container(column![controls, input, status_bar].spacing(10))
            .padding(10)
            .into()
    }
    fn theme(&self) -> Theme {
        if self.theme.is_dark(){
        Theme::Dark} else {
            Theme::Light
        }
    }
}

async fn pick_file() -> Result<(PathBuf, Arc<String>), Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a text file")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(handle.path().to_owned()).await
}

async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|error| error.kind())
        .map_err(Error::IO)?;

    Ok((path, contents))
}

#[derive(Debug, Clone)]
enum Error {
    DialogClosed,
    IO(io::ErrorKind),
}

fn default_file() -> PathBuf {
    PathBuf::from(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR")))
}

async fn save_file(path: Option<PathBuf>, text: String) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
            .set_title("Choose a file to save")
            .save_file()
            .await
            .ok_or(Error::DialogClosed)
            .map(|handle| handle.path().to_owned())?
    };
    tokio::fs::write(&path, text)
        .await
        .map_err(|error| error.kind());

    Ok(path)
}

fn new_icon<'a>() -> Element<'a, Message> {
    icon('\u{E802}')
}

fn save_icon<'a>() -> Element<'a, Message> {
    icon('\u{E800}')
}

fn open_icon<'a>() -> Element<'a, Message> {
    icon('\u{E801}')
}

fn icon<'a>(codepoint: char) -> Element<'a, Message> {
    const ICON_FONTS: Font = Font::with_name("editor");

    text(codepoint).font(ICON_FONTS).into()
}

fn action<'a>(content: Element<'a, Message>, on_press: Option<Message>, label: &str) -> Element<'a, Message> {
    let is_disabled = on_press.is_none();
    tooltip(
        button(container(content).width(30).center_x())
            .on_press_maybe(on_press)
            .padding([5, 10]).style(    
                if is_disabled {
                    theme::Button::Secondary
                } else {
                    theme::Button::Primary
                }
            ),
            label,
            tooltip::Position::FollowCursor,
    ).style(Container::Box)
    .into()
}
