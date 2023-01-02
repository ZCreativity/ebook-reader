use druid::widget::{Flex, Scroll, SizedBox, TextBox, ViewSwitcher, Widget, WidgetExt};
use druid::{widget::Controller, Env, UpdateCtx};

use crate::model::book::Book;

pub fn book_editor() -> impl Widget<Book> {
    let view_switcher = ViewSwitcher::new(
        |data: &Book, _env: &Env| data.is_editing_book(),
        |editing: &bool, _data: &Book, _env: &Env| {
            if *editing {
                let textbox = Scroll::new(
                    TextBox::multiline()
                        .controller(UpdateCallback())
                        .lens(Book::current_editing_page)
                        .expand_width(),
                );

                Box::new(Flex::column().with_child(textbox))
            } else {
                Box::new(SizedBox::empty())
            }
        },
    );
    view_switcher
}

struct UpdateCallback();

impl Controller<String, TextBox<String>> for UpdateCallback {
    fn update(
        &mut self,
        child: &mut TextBox<String>,
        ctx: &mut UpdateCtx<'_, '_>,
        old_data: &String,
        data: &String,
        env: &Env,
    ) {
        if old_data != data {
            // the data has changed, you can call your function here
            println!("{}", data);
        }
        // also inform the child that the data has changed
        child.update(ctx, old_data, data, env)
    }
}
