use druid::Widget;
use crate::widgets::header::header;

pub fn ui_builder() -> impl Widget<()> {
    let header_widget = header();
    header_widget
}