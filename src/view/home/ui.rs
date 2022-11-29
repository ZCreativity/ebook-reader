use crate::helper::config::{COVER_PLACEHOLDER, DISPLAY_HEIGHT, DISPLAY_WIDTH, PADDING_LG, TITLE};
use crate::model::book::Book;
use crate::model::library::Library;
use crate::{AppState, APP_NAME};
use druid::theme::BORDER_LIGHT;
use druid::widget::{
    Button, Click, Container, ControllerHost, CrossAxisAlignment, Flex, Label, List,
    MainAxisAlignment, Padding,
};
use druid::widget::{FillStrat, Image, Scroll, Svg, ViewSwitcher};

use druid::{Color, EventCtx, FontStyle, WindowDesc};
use druid::{FontDescriptor, FontFamily, FontWeight, Insets, LensExt, Widget, WidgetExt};

use html2text::from_read_rich;
use html2text::render::text_renderer::{RichAnnotation, TaggedLine};
use std::collections::HashMap;

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
    let _books_list = Scroll::new(List::new(book_item))
        .vertical()
        .lens(AppState::library.then(Library::books)); // Lens chaining

    // View switcher for book view and library view
    let view_switcher = ViewSwitcher::new(
        |data: &AppState, _env| data.get_opened_book().is_some(),
        |condition: &bool, data: &AppState, _env| {
            if *condition {
                // Book view
                let book = data.get_opened_book().unwrap();
                let book_view = Scroll::new(List::new(book_text))
                    .vertical()
                    .lens(data.get_opened_book());
                // Box::new(book_view)
            } else {
                // Library view
            }
        },
    );

    let books_texts = Scroll::new(List::new(book_text)).vertical();
    let books_texts_lens = books_texts.lens(AppState::library.then(Library::books));
    //.lens(AppState::library.then(Library::books));
    let layout = Flex::row().with_child(header).with_child(books_texts_lens);
    Padding::new(
        Insets::new(PADDING_LG, PADDING_LG, PADDING_LG, PADDING_LG),
        layout,
    )
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
