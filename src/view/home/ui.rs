use crate::helper::config::{COVER_PLACEHOLDER, PADDING_LG, TITLE, ALERT};
use crate::model::book::Book;
use crate::model::library::Library;
use crate::{AppState, APP_NAME};
use druid::widget::{Button, Flex, Label, MainAxisAlignment, Padding, CrossAxisAlignment, Click, ControllerHost};
use druid::widget::{FillStrat, Image, Scroll, Svg, ViewSwitcher};
use druid::{Insets, Widget, WidgetExt,Color};

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

    let container = books_container().lens(AppState::library);

    let layout = Flex::column()
        .with_child(header)
        .with_child(container)
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

fn books_container() -> impl Widget<Library> {
    // let books_list = Flex::row().with_child(List::new(book_item).horizontal())
    //    .lens(AppState::library.then(Library::books));

    let container = Flex::row().with_child(ViewSwitcher::new(
        |data: &Library, _env| (data.get_length() > 0_f64), //Se ho libri in libreria
        move |f, data, _env| {
            if *f  {
                let cols = 3;
                let rows = (data.get_length() / 3_f64).ceil() as i32;
                
                let mut col = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
                let books = data.get_books_vec();
                let mut component_vector = Vec::new(); //Vettore di componenti fatti con la funzione di gui

                for book in books {
                    component_vector.push(book_component(book));
                }
                
                for _ in 0..rows  {
                    let mut row = Flex::row();
                    for _ in 0..cols {
                        let component = component_vector.pop();
                        match component {
                            Some(x) => { row = row.with_child(x)}
                            None => { }
                        }
                    }
                    col = col.with_child(row);
                }

                Box::new(Scroll::new(col))

            } else {
                let label = Label::new("There are no books in the library!").with_font(ALERT).padding(Insets::new(0.0, 200.0, 0.0, 0.0));
                Box::new(label)
            }
        },
    ));

    return container;
    

}

fn book_component(book: Book) -> impl Widget<Library> {

    let title = Label::new(book.get_title().as_str());
    let author = Label::new(book.get_author().as_str());
    
    let mut book_layout = Flex::row();

    //TODO: Gestire il caso di mancanza della copertina

    let mut col_details = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    col_details.add_child(title);
    col_details.add_spacer(10.0);
    col_details.add_child(author);
    col_details.add_spacer(50.0);

    match book.get_image_buf() {
        Some(_) => {
            let cover = Image::new(book.get_image_buf().unwrap().as_ref().clone()).fix_size(100.0, 200.0);
            book_layout.add_child(Flex::column().with_child(cover));
            }
        None => { 
            let cover = Svg::new(COVER_PLACEHOLDER.parse().unwrap()).fill_mode(FillStrat::Fill);
            book_layout.add_child(Flex::column().with_child(cover))
        }
    }
    book_layout.add_spacer(10.0);
    book_layout.add_child(col_details);

    let container = Flex::column().with_child(book_layout)
         .fix_size(340.0, 200.0)
         .padding(2.0)
        .border(Color::GRAY, 2_f64);

    let controller_host = ControllerHost::new(container,
                                              Click::new(|_, _, _| println!("Click")));

    return controller_host;
}
