use crate::helper::config::{COVER_PLACEHOLDER, PADDING_LG, TITLE};
use crate::model::book::Book;
use crate::model::library::Library;
use crate::{AppState, APP_NAME};
use druid::widget::{Button, Flex, Label, List, MainAxisAlignment, Padding, Container};
use druid::widget::{FillStrat, Image, Scroll, Svg, ViewSwitcher};
use druid::{Insets, LensExt, Widget, WidgetExt,Color};

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
    let books_list = Scroll::new(List::new(book_item))
        .vertical()
        .lens(AppState::library.then(Library::books)); // Lens chaining
    let layout = Flex::column()
        .with_child(header)
        .with_child(books_list)
        .fix_height(500.0);
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

/* Book item */
fn book_item() -> impl Widget<Book> {
    let title = Label::raw().lens(Book::title);

    // // Clickable widget needs click controller and controller host
    // let click_controller = Click::new(|_ctx, data: &mut Book, _env| {
    //     // TODO: Open new window with book data
    //     println!("Clicked book {}", data.get_title())
    // });
    // let controller_host = ControllerHost::new(container, click_controller);

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
    let author= Label::raw().lens(Book::title);

    let mut book_layout = Flex::row();
    let col_cover = Flex::column().with_child(cover);

    let mut col_details = Flex::column();
    col_details.add_child(title);
    col_details.add_spacer(10.0);
    col_details.add_child(author);
    col_details.add_spacer(50.0);

    book_layout.add_child(col_cover);
    book_layout.add_spacer(10.0);
    book_layout.add_child(col_details);

    let container = Container::new(book_layout)
         .fix_size(300.0, 200.0)
         .padding(2.0)
         .border(Color::YELLOW, 2.0);

    
    return container;
}
