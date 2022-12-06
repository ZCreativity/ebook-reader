use crate::controller::app_delegate::OPEN_BOOK;
use crate::helper::config::{ALERT, COVER_PLACEHOLDER, PADDING_LG, TITLE};
use crate::model::book::Book;
use crate::model::library::Library;
use crate::view::book::ui::book_view;
use crate::{AppState, APP_NAME};
use druid::widget::{
    Button, Click, ControllerHost, CrossAxisAlignment, Flex, Label, List, MainAxisAlignment,
    Padding,
};
use druid::widget::{FillStrat, Image, Scroll, Svg, ViewSwitcher};
use druid::{Color, EventCtx, Insets, LensExt, Widget, WidgetExt};

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
        |data: &AppState, _env| data.get_opened_book().is_some(),
        |f, _data, _env| {
            if *f {
                Box::new(book_view())
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
    let book_list = Flex::row().with_child(Scroll::new(List::new(book_item))
        .vertical()
        .lens(AppState::library.then(Library::books))); // Lens chaining

    let mut layout = Flex::column();
    layout.add_child(header);
    layout.add_child(Flexbook_list);

    Padding::new(
        Insets::new(PADDING_LG, PADDING_LG, PADDING_LG, PADDING_LG),
        layout,
    )
}

/* Book item */
fn book_item() -> impl Widget<Book> {
    let title = Label::raw().lens(Book::title);
    let author = Label::raw().lens(Book::author);

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

    let mut book_layout = Flex::row();

    let mut col_details = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    col_details.add_child(title);
    col_details.add_spacer(10.0);
    col_details.add_child(author);
    col_details.add_child(button);
    col_details.add_spacer(50.0);

    book_layout.add_child(cover);
    book_layout.add_spacer(10.0);
    book_layout.add_child(col_details);

    let container = Flex::column()
        .with_child(book_layout)
        .fix_size(340.0, 200.0)
        .padding(2.0)
        .border(Color::GRAY, 2_f64);

    let controller_host = ControllerHost::new(container, Click::new(|_, _, _| println!("Click")));

    return controller_host;
}

/* Header section */
fn header() -> impl Widget<AppState> {
    let header_label = Label::new(APP_NAME).with_font(TITLE);

    let add_book_button =
        Button::new("Add book").on_click(|ctx: &mut EventCtx, data: &mut AppState, _| {
            ctx.request_update();
            data.add_book();
        });

    let mut header = Flex::row()
        .with_child(header_label)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .must_fill_main_axis(true);

    header.add_child(add_book_button);

    header
}
