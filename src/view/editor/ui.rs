use druid::widget::{TextBox, Widget};

pub fn book_editor() -> impl Widget<String>{
    let textbox = TextBox::new();
    textbox
}