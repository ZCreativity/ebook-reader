use crate::{
    controller::view::BOOK_READ,
    helper::config::{APP_NAME, COVER_PLACEHOLDER, PADDING_LG, PADDING_SM, TITLE},
    model::{app_state::AppState, book::Book, ui_view::UiView},
};
use druid::{
    widget::{
        Button, Container, FillStrat, Flex, Image, Label, List, ListIter, MainAxisAlignment,
        Padding, Scroll, SizedBox, Svg, ViewSwitcher,
    },
    Command, Data, EventCtx, Target, Widget, WidgetExt,
};
use std::sync::Arc;

/**
 * Main page and contains list view of books
 * notice that this must return Box<dyn Widget<YourState>> instead of impl Widget<YourState>
 * navigator needs Boxed widgets in order to store the widgets.
 *
 * Widgets take the AppState as their data type, which holds a tuple of
 * (Arc<Vec<UiView>>, Book, Option<usize>, usize)
 * data.0 -> Current View
 * data.1 -> Book
 * data.2 -> Selected book index (or None)
 * data.3 -> Index of the book
 */
pub fn library() -> Box<dyn Widget<AppState>> {
    // Book list
    let list = List::new(|| {
        // Book title
        let book_title = Label::new(
            |(_views, book, _selection, _idx): &(Arc<Vec<UiView>>, Book, Option<usize>, usize),
             _env: &_| { book.get_title() },
        );

        // Book author
        let author = Label::new(
            |(_views, book, _selected, _idx): &(Arc<Vec<UiView>>, Book, Option<usize>, usize),
             _env: &_| { book.get_author() },
        );

        // Book cover
        let cover = Flex::row().with_child(ViewSwitcher::new(
            |(_views, book, _selected, _idx): &(Arc<Vec<UiView>>, Book, Option<usize>, usize),
             _env| book.get_image_buf().is_some(),
            move |f, data, _env| {
                if *f {
                    Box::new(
                        Image::new(data.1.get_image_buf().as_ref().unwrap().as_ref().clone()) //Unwrap is safe because the image is checked
                            .fix_size(100.0, 200.0),
                    )
                } else {
                    Box::new(
                        Svg::new(COVER_PLACEHOLDER.parse().unwrap()).fill_mode(FillStrat::Fill), //Unwrap is safe because the missing-cover is hardcoded
                    )
                }
            },
        ));

        // Book progress (if any)
        let progress_switcher = ViewSwitcher::new(
            |(_views, book, _selected, _idx): &(Arc<Vec<UiView>>, Book, Option<usize>, usize),
             _env| book.get_has_progress(),
            move |f, _data, _env| {
                if *f {
                    Box::new(Button::new("Keep Reading").on_click(
                        |event, data: &mut (Arc<Vec<UiView>>, Book, Option<usize>, usize), _env| {
                            println!("Keep Reading: {}", data.1.get_title());
                            let new_views = Arc::make_mut(&mut data.0);
                            new_views.push(UiView::BookRead);
                            data.0 = Arc::new(new_views.to_owned());
                            data.2 = Some(data.3);
                            event.submit_command(Command::new(BOOK_READ, data.3, Target::Auto));
                        },
                    ))
                } else {
                    Box::new(SizedBox::empty())
                }
            },
        );

        // Details and functions of the book
        let details = Flex::column()
            .with_child(book_title)
            .with_spacer(PADDING_SM)
            .with_child(author)
            .with_spacer(PADDING_SM)
            .with_child(progress_switcher);

        //Entire book layout
        let book_layout = Flex::row()
            .with_child(cover)
            .with_spacer(PADDING_SM)
            .with_child(details);

        // Open book on click
        // 1. Make the view arc mutable
        // 2. Add the BookRead view to the views (this will trigger the view switcher)
        // 3. Set the selected book (index) to the current book (index)
        // 4. Send the command to open the book with the index as payload
        let book_layout = book_layout.on_click(|event, data, _env| {
            let new_views = Arc::make_mut(&mut data.0);
            new_views.push(UiView::BookRead);
            data.0 = Arc::new(new_views.to_owned());
            data.2 = Some(data.3);
            event.submit_command(Command::new(BOOK_READ, data.3, Target::Auto));
        });

        book_layout
    });

    // Layout of the page
    let layout = Flex::column()
        .with_child(header())
        .with_flex_child(Scroll::new(list.with_spacing(20.0)).center(), 1.)
        .must_fill_main_axis(true)
        .expand_width();

    Box::new(Padding::new(PADDING_LG, Container::new(layout)))
}

/* Header section */
fn header() -> impl Widget<AppState> {
    let header_label = Label::new(APP_NAME).with_font(TITLE);

    let add_book_button =
        Button::new("Add book").on_click(|_ctx: &mut EventCtx, data: &mut AppState, _| {
            data.add_book_from_file();
        });

    let mut header = Flex::row()
        .with_child(header_label)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .must_fill_main_axis(true);

    header.add_child(add_book_button);

    header
}

// TODO: Da capire bene
// A little special implementation to give the list view all that it needs
// to list the Book structs
impl ListIter<(Arc<Vec<UiView>>, Book, Option<usize>, usize)> for AppState {
    fn for_each(&self, mut cb: impl FnMut(&(Arc<Vec<UiView>>, Book, Option<usize>, usize), usize)) {
        for (idx, contact) in self.get_library().iter().enumerate() {
            let nav_state = self.nav_state.clone();
            cb(&(nav_state, contact.clone(), self.get_selected(), idx), idx);
        }
    }

    fn for_each_mut(
        &mut self,
        mut cb: impl FnMut(&mut (Arc<Vec<UiView>>, Book, Option<usize>, usize), usize),
    ) {
        let mut any_shared_changed = false;
        for (idx, contact) in self.get_library().iter().enumerate() {
            let mut d = (
                self.nav_state.clone(),
                contact.clone(),
                self.get_selected(),
                idx,
            );

            cb(&mut d, idx);
            if !any_shared_changed && !self.nav_state.same(&d.0) {
                any_shared_changed = true;
            }
            if any_shared_changed {
                self.nav_state = d.0;
                self.set_selected(d.2);
            }
        }
    }

    fn data_len(&self) -> usize {
        self.get_library().len()
    }
}
