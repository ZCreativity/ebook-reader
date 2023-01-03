use std::sync::Arc;

use druid::{
    widget::{Button, Container, CrossAxisAlignment, Flex, Label, MainAxisAlignment},
    Color, Command, Env, Target, Widget, WidgetExt,
};
use druid_widget_nursery::navigator::ViewController;

use crate::{
    controller::view::BOOK_EDIT,
    model::{app_state::AppState, ui_view::UiView},
};

// details views - this is the second view after clicking on a contact
pub fn book_view() -> Box<dyn Widget<AppState>> {
    let name = Label::dynamic(|data: &AppState, _env: &Env| {
        if let Some(idx) = data.selected {
            format!("Book: {}", data.library[idx].get_title())
        } else {
            "".to_string()
        }
    })
    .with_text_size(20.);

    // let email = Label::new(|data: &AppState, _env: &Env| {
    //     if let Some(idx) = data.selected {
    //         format!("Email: {}", data.contacts[idx].email)
    //     } else {
    //         "".to_string()
    //     }
    // })
    // .with_text_size(20.);

    // let age = Label::new(|data: &AppState, _env: &Env| {
    //     if let Some(idx) = data.selected {
    //         format!("Age: {}", data.contacts[idx].age)
    //     } else {
    //         "".to_string()
    //     }
    // })
    // .with_text_size(20.);

    // let favorite_food = Label::new(|data: &AppState, _env: &Env| {
    //     if let Some(idx) = data.selected {
    //         format!("Favorite food: {}", data.contacts[idx].favorite_food)
    //     } else {
    //         "".to_string()
    //     }
    // })
    // .with_text_size(20.);

    // you might want to define a command that pops a view so that you may scope down your AppState
    let back_button = Button::new("Back").on_click(|_event, data: &mut AppState, _env| {
        data.pop_view();
    });

    // let edit_button = Button::new("Edit").on_click(|event, data: &mut AppState, _env| {
    //     let views = Arc::make_mut(&mut data.nav_state);
    //     views.push(UiView::new("contact edit".to_string()));
    //     data.nav_state = Arc::new(views.to_owned());
    //     event.submit_command(Command::new(
    //         BOOK_EDIT,
    //         data.selected.unwrap(),
    //         Target::Auto,
    //     ));
    // });

    let layout = Flex::column()
        .with_child(name)
        // .with_child(email)
        // .with_child(age)
        // .with_child(favorite_food)
        .cross_axis_alignment(CrossAxisAlignment::Start);
    let layout = Flex::column()
        .with_child(back_button)
        .with_child(layout)
        // .with_child(edit_button)
        .must_fill_main_axis(true)
        .main_axis_alignment(MainAxisAlignment::SpaceAround);

    let container = Container::new(layout.center());

    Box::new(container)
}
