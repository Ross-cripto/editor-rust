
use std::sync::Arc;
use std::path::Path;

use iced::highlighter::Highlighter;
use iced::widget::{column, container, row, horizontal_space, pick_list, text, text_editor};
use iced::{highlighter, Element, Length, Theme};

use crate::app::editor::{Editor, Message};
use crate::icons;
use crate::error::Error;

pub fn view(editor: &Editor) -> Element<'_, Message> {
    let controls = row![
        icons::action(icons::open_icon(), Some(Message::Open), "Open a new File"),
        icons::action(icons::new_icon(), Some(Message::New), "Create a new File"),
        icons::action(icons::save_icon(), editor.is_dirty.then_some(Message::Save), "Save the current File"),
        horizontal_space(Length::Fill),
        pick_list(highlighter::Theme::ALL, Some(editor.theme), Message::ThemeSelected)
    ]
    .spacing(10);

    let input:  Element<'_, Message> = text_editor(&editor.content)
            .on_edit(Message::Edit)
            .highlight::<Highlighter>(
                highlighter::Settings {
                theme: editor.theme,
                extension: editor
                .path
                .as_ref()
                .and_then(|path| path.extension()?.to_str())
                .unwrap_or("rs")
                .to_string(),
            }, |highlight, theme| highlight.to_format()).into();

    let file_path = match editor.error.as_ref() {
        Some(Error::IO(e)) => text(e.to_string()).style(iced::Color::from_rgb(1.0,0.0,0.0)),
        _ => match editor.path.as_deref().and_then(Path::to_str) {
            Some(path) => text(path).size(20),
            None => text("No file opened").size(14),
        }
    };

    let position = {
        let (line, col) = editor.content.cursor_position();
        text(format!("Ln {}, Col {}", line, col))
    };

    let status_bar = row![file_path, horizontal_space(Length::Fill), position];

    container(column![controls, input, status_bar].spacing(10)).padding(10).into()
}
