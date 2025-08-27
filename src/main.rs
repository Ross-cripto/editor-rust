mod app;
mod error;
mod icons;
mod io;
mod ui;

fn main() -> iced::Result {
    app::run()
}

pub fn default_file() -> std::path::PathBuf {
    std::path::PathBuf::from(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR")))
}
