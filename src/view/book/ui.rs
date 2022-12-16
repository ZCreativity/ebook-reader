use crate::controller::app_delegate::CLOSE_BOOK;
use crate::controller::parser::parse;
use crate::helper::config::PADDING_LG;
use crate::model::book::Book;
use druid::widget::{
    Button, CrossAxisAlignment, Flex, Label, MainAxisAlignment, Padding, SizedBox,
};
use druid::widget::{Scroll, ViewSwitcher};
use druid::Insets;
use druid::{Widget, WidgetExt};

pub fn book_view() -> impl Widget<Book> {
    // Return a widget that can be used to display a book
    let book_text = Scroll::new(book_text())
        .vertical()
        .fix_height(600.0)
        .expand_width();
    let book_menu = book_menu();
    let book_controls = book_controls();

    Padding::new(
        Insets::new(PADDING_LG, PADDING_LG, PADDING_LG, PADDING_LG),
        Flex::column()
            .with_child(book_menu)
            .with_child(book_text)
            .with_child(book_controls),
    )
}

fn book_menu() -> impl Widget<Book> {
    let back_button = Button::new("Back").on_click(|ctx, _data: &mut Book, _env| {
        println!("Going back");
        ctx.submit_command(CLOSE_BOOK);
    });

    let edit_button = Button::new("Edit").on_click(|ctx, _data: &mut Book, _env| {
        println!("Editing book");
        // For now just close the book
        ctx.submit_command(CLOSE_BOOK);
    });

    let change_font_button = Button::new("Change font").on_click(|ctx, _data: &mut Book, _env| {
        println!("Changing font");
        // For now just close the book
        ctx.submit_command(CLOSE_BOOK);
    });

    let flex = Flex::row()
        .with_child(back_button)
        .with_child(edit_button)
        .with_child(change_font_button)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .must_fill_main_axis(true)
        .padding(Insets::new(0.0, 0.0, 0.0, PADDING_LG));

    flex
}

fn book_controls() -> impl Widget<Book> {
    let control_next = ViewSwitcher::new(
        |data: &Book, _env| data.has_next_page(),
        |f, _data, _env| {
            if *f {
                Box::new(Button::new("Next").on_click(|ctx, data: &mut Book, _env| {
                    data.next_page();
                    ctx.request_update();
                }))
            } else {
                Box::new(SizedBox::empty())
            }
        },
    );

    let control_prev = ViewSwitcher::new(
        |data: &Book, _env| data.has_prev_page(),
        |f, _data, _env| {
            if *f {
                Box::new(Button::new("Prev").on_click(|ctx, data: &mut Book, _env| {
                    data.prev_page();
                    ctx.request_update();
                }))
            } else {
                Box::new(SizedBox::empty())
            }
        },
    );

    Flex::row()
        .with_child(control_prev)
        .with_child(control_next)
        .must_fill_main_axis(true)
        .main_axis_alignment(MainAxisAlignment::End)
}

fn book_text() -> impl Widget<Book> {
    let book_page = ViewSwitcher::new(
        |data: &Book, _env| data.get_current_page(),
        |index, data, _env| {
            let page = data.get_page_str(*index);
            match page {
                Some(page) => {
                    let page_flex = parse(page, 24.0);
                    Box::new(page_flex)
                }
                None => Box::new(Label::new("No page")),
            }
        },
    );
    Flex::column().with_child(book_page)
}
