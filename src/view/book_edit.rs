use crate::{
    controller::view::POP_VIEW,
    model::{
        app_state::AppState,
        book::Book,
        edit_state::{EditState, EditTransfer},
    }, helper::config::{TITLE, PADDING_LG},
};
use druid::{
    widget::{
        Button, Container, CrossAxisAlignment, Flex, Label, MainAxisAlignment, Scope, Scroll,
        TextBox, Padding,
    },
    Widget, WidgetExt,
};
use druid_widget_nursery::navigator::ViewController;

fn header() -> impl Widget<AppState> {
    let header_label = Label::new("Ebook Editor").with_font(TITLE);

    let mut header = Flex::row()
        .with_child(header_label)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .must_fill_main_axis(true);
        
    let back_button = Button::new("Back").on_click(|_event, data: &mut AppState, _env| {
        data.pop_view();
    });

    header.add_child(back_button);

    header
    
}

pub fn book_edit() -> Box<dyn Widget<AppState>> {

    let textbox = Flex::column()
        .with_child(
            Scroll::new(
                TextBox::multiline()
                    .with_text_size(20.)
                    .expand_width()
                    .lens(Book::current_page_str),
            )
            .vertical()
            .fix_height(600.0),
        )
        .cross_axis_alignment(CrossAxisAlignment::Start);

    let layout = Flex::column()
        .with_child(textbox)
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
        .with_child(header())
        .with_flex_child(layout, 1.0)
        .main_axis_alignment(MainAxisAlignment::SpaceAround);

        Box::new(Padding::new(PADDING_LG, Container::new(layout)))
}
