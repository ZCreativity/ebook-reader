use crate::helper::config::{APP_NAME, TITLE};
use druid::widget::{Button, Flex, Label};
use druid::{Widget};
use crate::AppState;
use crate::model::library::Library;

pub fn header() -> impl Widget<AppState> {
    let header_label = Label::new(APP_NAME).with_font(TITLE);
    let add_book_button = Button::new("Add book").on_click(|_, _, _| Library::add_book());
    Flex::row().with_child(header_label).with_child(add_book_button)
}
