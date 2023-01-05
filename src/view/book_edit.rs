use crate::{
    controller::view::POP_VIEW,
    model::{
        app_state::AppState,
        book::Book,
        edit_state::{EditState, EditTransfer},
    },
};
use druid::{
    widget::{
        Button, Container, CrossAxisAlignment, Flex, Label, MainAxisAlignment, Scope, TextBox,
    },
    Color, Widget, WidgetExt,
};
use druid_widget_nursery::navigator::ViewController;

pub fn book_edit() -> Box<dyn Widget<AppState>> {
    let back_button = Button::new("Back").on_click(|_event, data: &mut AppState, _env| {
        data.pop_view();
    });

    let name_input = Flex::column()
        .with_child(Label::new("Title"))
        .with_child(
            TextBox::new()
                .with_text_size(20.)
                .fix_width(300.)
                .lens(Book::title),
        )
        .cross_axis_alignment(CrossAxisAlignment::Start);

    let layout = Flex::column()
        .with_child(name_input)
        .must_fill_main_axis(true)
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .lens(EditState::book);

    let save_button = Button::new("Save").on_click(|event, data: &mut EditState, _env| {
        data.was_saved = true;
        // use a command here because EditState does not have access to the navigation state
        event.submit_command(POP_VIEW);
    });

    let layout = Flex::column()
        .with_flex_child(layout, 1.0)
        .with_child(save_button)
        .main_axis_alignment(MainAxisAlignment::SpaceAround);

    // use this scope widget to independently update data used for this view
    // if a lens is used the data would update automatically.
    // using a scope allows you to control when to update the AppState such as only
    // when the save button is clicked
    let layout = Scope::from_function(EditState::new, EditTransfer, layout);
    let layout = Flex::column()
        .with_child(back_button)
        .with_flex_child(layout, 1.0)
        .main_axis_alignment(MainAxisAlignment::SpaceAround);

    let container = Container::new(layout).background(Color::WHITE);

    Box::new(container)
}
