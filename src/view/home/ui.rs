use druid::{Insets, Widget, WidgetExt};
use druid::widget::{Button, CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, Padding};
use crate::{APP_NAME, AppState};
use crate::helper::config::{PADDING_LG, TITLE};
use crate::model::book::Book;
use crate::model::library::Library;

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
    let header = header();
    let books_list = List::new(book_item).lens(AppState::books_list);
    let layout = Flex::column().with_child(header).with_child(books_list);
    Padding::new(Insets::new(PADDING_LG, PADDING_LG, PADDING_LG, PADDING_LG), layout)
}

/* Header section */
fn header() -> impl Widget<AppState> {
    let header_label = Label::new(APP_NAME).with_font(TITLE);
    let add_book_button = Button::new("Add book").on_click(|_, _, _| Library::add_book());
    let mut header = Flex::row().with_child(header_label).main_axis_alignment(MainAxisAlignment::SpaceBetween).must_fill_main_axis(true);
    header.add_child(add_book_button);
    header
}

/* Book item */
fn book_item() -> impl Widget<Book> {
    let title = Label::raw().lens(Book::title);
    Flex::column().with_child(title)
}

