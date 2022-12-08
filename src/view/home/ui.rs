use crate::controller::app_delegate::OPEN_BOOK;
use crate::helper::config::{COVER_PLACEHOLDER, PADDING_LG, TITLE};
use crate::model::book::Book;
use crate::model::library::Library;
use crate::view::book::ui::book_view;
use crate::{AppState, APP_NAME};
use druid::widget::{Button, Flex, Label, List, MainAxisAlignment, Padding};
use druid::widget::{FillStrat, Image, Scroll, Svg, ViewSwitcher};
use druid::{Insets, LensExt, Widget, WidgetExt};

/** Notes on Data and Lens.
   Il tratto Lens permette di accedere ad una porzione di una struttura dati
   (che implementa il tratto Lens). Ad esempio .lens(AppState::books_list)
   permette di accedere al campo books_list di AppState. Vogliamo mostrare una lista
   di libri, per fare ciò abbiamo la lista di "oggetti" Book (model) dentro al vettore
   books_list di AppState. La funzione book_item ritorna un widget per ogni libro presente nel
   vettore (notare che la funzione book_item ritorna una impl Widget<Book> in modo che possiamo fare
   .lens(Book::title)). Per essere sottoposto al tratto Lens una struttura deve implementare anche Data
*/

/* Home ui builder */
pub fn build_ui() -> impl Widget<AppState> {
    // View switcher for book view and library view
    let view_switcher = ViewSwitcher::new(
        |data: &AppState, _env| data.get_is_reading_book(),
        |f, _data, _env| {
            if *f {
                Box::new(book_view().lens(AppState::opened_book))
            } else {
                Box::new(library_view())
            }
        },
    );

    Padding::new(
        Insets::new(PADDING_LG, PADDING_LG, PADDING_LG, PADDING_LG),
        view_switcher,
    )
}

/* Library View */
fn library_view() -> impl Widget<AppState> {
    let header = header();
    let book_list = Scroll::new(List::new(book_item))
        .vertical()
        .lens(AppState::library.then(Library::books)); // Lens chaining

    let layout = Flex::row().with_child(header).with_child(book_list);

    Padding::new(
        Insets::new(PADDING_LG, PADDING_LG, PADDING_LG, PADDING_LG),
        layout,
    )
}

/* Book item */
fn book_item() -> impl Widget<Book> {
    let title = Label::raw().lens(Book::title);

    let cover = Flex::row().with_child(ViewSwitcher::new(
        |data: &Book, _env| data.get_image_buf().is_some(),
        move |f, data, _env| {
            if *f {
                Box::new(
                    Image::new(data.get_image_buf().as_ref().unwrap().as_ref().clone())
                        .fix_size(100.0, 200.0),
                )
            } else {
                Box::new(Svg::new(COVER_PLACEHOLDER.parse().unwrap()).fill_mode(FillStrat::Fill))
            }
        },
    ));

    let button = Button::new("Open").on_click(|ctx, data: &mut Book, _env| {
        println!("Opening book: {}", data.get_title());
        ctx.submit_command(OPEN_BOOK.with(data.clone()));
    });
    Flex::row()
        .with_child(title)
        .with_child(cover)
        .with_child(button)
}

/* Header section */
fn header() -> impl Widget<AppState> {
    let header_label = Label::new(APP_NAME).with_font(TITLE);
    let add_book_button =
        Button::new("Add book").on_click(|_, data: &mut AppState, _| data.add_book());
    let mut header = Flex::row()
        .with_child(header_label)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .must_fill_main_axis(true);
    header.add_child(add_book_button);
    header
}
