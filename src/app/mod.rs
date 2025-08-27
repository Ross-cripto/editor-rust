pub mod editor;

use iced::Application; 
pub use editor::Editor;

use iced::Settings;

pub fn run() -> iced::Result {
    Editor::run(Settings {
        default_font: iced::Font::MONOSPACE,
        fonts: vec![include_bytes!("../fonts/editor.ttf").as_slice().into()],
        ..Settings::default()
    })
}
