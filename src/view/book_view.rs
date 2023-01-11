use std::sync::Arc;

use druid::{
    widget::{
        Button, Container, CrossAxisAlignment, Flex, Label, MainAxisAlignment, Padding, Scroll,
        SizedBox, ViewSwitcher,
    },
    Command, Env, Insets, Target, Widget, WidgetExt,
};
use druid_widget_nursery::navigator::ViewController;

use crate::{
    controller::{parser::parse, view::BOOK_EDIT},
    helper::config::PADDING_LG,
    model::{app_state::AppState, ui_view::UiView},
};

// details views - this is the second view after clicking on a contact
pub fn book_view() -> Box<dyn Widget<AppState>> {
    let book_menu = book_menu();
    let top_right_buttons = top_right();
    let book_controls = book_controls();
    let book_page = book_page();

    let top_bar = Flex::row()
        .with_child(book_menu)
        .with_child(top_right_buttons)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .must_fill_main_axis(true);

    let book_text = Scroll::new(book_page).vertical().fix_height(600.0);
    //.fix_width(700.0);

    let bottom_bar = Flex::row()
        .with_child(book_controls)
        .main_axis_alignment(MainAxisAlignment::End);

    let layout = Flex::column()
        .with_child(top_bar)
        .with_child(book_text)
        .with_spacer(20_f64)
        .with_child(bottom_bar)
        .cross_axis_alignment(CrossAxisAlignment::Center);

    let container = Padding::new(PADDING_LG, Container::new(layout.center()));

    Box::new(container)
}

fn book_menu() -> impl Widget<AppState> {
    let back_button = Button::new("Back").on_click(|_event, data: &mut AppState, _env| {
        data.pop_view();
    });

    let edit_button = Button::new("Edit").on_click(|event, data: &mut AppState, _env| {
        let views = Arc::make_mut(&mut data.nav_state);
        views.push(UiView::BookEdit);
        data.nav_state = Arc::new(views.to_owned());
        data.set_editing_page();
        event.submit_command(Command::new(
            BOOK_EDIT,
            data.get_selected().unwrap(),
            Target::Auto,
        ));
    });

    let increase_font_button = Button::new("Aa +").on_click(|_ctx, data: &mut AppState, _env| {
        println!("Increasing font");
        data.increase_font_size();
    });

    let decrease_font_button = Button::new("Aa -").on_click(|_ctx, data: &mut AppState, _env| {
        println!("Decreasing font");
        data.decrease_font_size();
    });

    let save_book_progress = Button::new("Bookmark").on_click(|_ctx, data: &mut AppState, _env| {
        println!("Saving book progress");
        data.save_book_progress();
    });

    let flex = Flex::row()
        .with_child(back_button)
        .with_child(edit_button)
        .with_child(increase_font_button)
        .with_child(decrease_font_button)
        .with_child(save_book_progress)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .must_fill_main_axis(true)
        .padding(Insets::new(0.0, 0.0, 0.0, PADDING_LG));

    flex
}

fn top_right() -> impl Widget<AppState> {
    let page_counter = Label::dynamic(|data: &AppState, _env: &Env| {
        if let Some(idx) = data.get_selected() {
            format!(
                "Page {}/{}",
                data.get_library()[idx].get_current_page(),
                data.get_library()[idx].get_book_length()
            )
        } else {
            "".to_string()
        }
    });

    let ocr_button =
        Button::new("Search from photo").on_click(|_ctx, data: &mut AppState, _env| {
            data.ocr_from_file();
        });

    let reverse_ocr_button = Button::new("Get physical page of this chapter").on_click(
        |_ctx, data: &mut AppState, _env| {
            data.reverse_ocr();
        },
    );

    let reverse_ocr_result_label = ViewSwitcher::new(
        |data: &AppState, _env| {
            data.get_library()[data.get_selected().unwrap()].get_physical_page_range()
        },
        |f, data, _env| {
            if f.is_some() {
                println!("Poradsafds");
                let (start, end) = data.get_library()[data.get_selected().unwrap()]
                    .get_physical_page_range()
                    .unwrap();
                Box::new(Label::new(format!("Page range: {}-{}", start, end)))
            } else {
                Box::new(Label::new(""))
            }
        },
    );

    Flex::row()
        .with_child(ocr_button)
        .with_default_spacer()
        .with_child(reverse_ocr_button)
        .with_default_spacer()
        .with_child(reverse_ocr_result_label)
        .with_child(page_counter)
}

fn book_controls() -> impl Widget<AppState> {
    let control_next = ViewSwitcher::new(
        |data: &AppState, _env| data.has_next_page(),
        |f, _data, _env| {
            if *f {
                let last_page_button =
                    Button::new("Last page").on_click(|_event, data: &mut AppState, _env| {
                        data.navigate_to_last_page();
                    });

                let next_button =
                    Button::new("Next").on_click(|_ctx, data: &mut AppState, _env| {
                        data.navigate_to_next_page();
                    });

                Box::new(
                    Flex::row()
                        .with_child(next_button)
                        .with_child(last_page_button),
                )
            } else {
                Box::new(SizedBox::empty())
            }
        },
    );

    let control_prev = ViewSwitcher::new(
        |data: &AppState, _env| data.has_prev_page(),
        |f, _data, _env| {
            if *f {
                let first_page_button =
                    Button::new("First page").on_click(|_event, data: &mut AppState, _env| {
                        data.navigate_to_first_page();
                    });

                let prev_button =
                    Button::new("Prev").on_click(|_ctx, data: &mut AppState, _env| {
                        data.navigate_to_prev_page();
                    });

                Box::new(
                    Flex::row()
                        .with_child(first_page_button)
                        .with_child(prev_button),
                )
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

fn book_page() -> impl Widget<AppState> {
    let page_switcher = ViewSwitcher::new(
        |data: &AppState, _env| data.get_library()[data.get_selected().unwrap()].get_current_page(),
        |page_index, data, _env| {
            let page = data.get_library()[data.get_selected().unwrap()].get_page_str(*page_index);
            let parsed_page = parse(page.unwrap());
            Box::new(parsed_page)
        },
    );

    page_switcher
}
