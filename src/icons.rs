use iced::widget::{button, container, tooltip, text};
use iced::Element;
use iced::Length;
use iced::theme;
use iced::widget::tooltip::Position;
use crate::app::editor::Message;

const ICON_FONTS: iced::Font = iced::Font::with_name("editor");

pub fn icon(codepoint: char) -> Element<'static, Message> {
    text(codepoint).font(ICON_FONTS).into()
}

pub fn new_icon() -> Element<'static, Message> { icon('\u{E802}') }
pub fn save_icon() -> Element<'static, Message> { icon('\u{E800}') }
pub fn open_icon() -> Element<'static, Message> { icon('\u{E801}') }

pub fn action(
    content: Element<'static, Message>,
    on_press: Option<Message>,
    label: &str,
) -> Element<'static, Message> {
    let is_disabled = on_press.is_none();
    tooltip(
        button(container(content).width(30).center_x())
            .on_press_maybe(on_press)
            .padding([5, 10])
            .style(if is_disabled { theme::Button::Secondary } else { theme::Button::Primary }),
        label,
        Position::FollowCursor,
    )
    .style(iced::theme::Container::Box)
    .into()
}
