use druid::{Widget};
use druid::widget::{Flex, Label};
use crate::helper::config::{APP_NAME, TITLE};

pub fn header() -> impl Widget<()> {
    let header_label = Label::new(APP_NAME).with_font(TITLE);
    Flex::row().with_child(header_label)
}