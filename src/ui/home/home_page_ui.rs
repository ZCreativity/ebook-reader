use crate::widgets::header::header;
use druid::{UnitPoint, Widget, WidgetExt};
use druid::widget::{CrossAxisAlignment, Flex, Label, List, Scroll};
use crate::AppState;
use crate::model::book::Book;

pub fn ui_builder() -> impl Widget<AppState> {
    let header_widget = header();

    let mut lists = Flex::row().cross_axis_alignment(CrossAxisAlignment::Start);

    lists.add_flex_child(header_widget, 1.0);

    lists.add_flex_child(
        Scroll::new(List::new(|| {
            Label::new(|item: &Book, _env: &_| format!("List item #{}", item.get_title()))
                .align_vertical(UnitPoint::LEFT)
                .padding(10.0)
                .expand()
                .height(50.0)
        }))
            .vertical()
            .lens(AppState::books_list),
        1.0,
    );

    lists
}
