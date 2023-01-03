use std::sync::Arc;

use crate::{
    controller::view::BOOK_READ,
    helper::config::{APP_NAME, COVER_PLACEHOLDER, PADDING_LG, TITLE},
    model::{app_state::AppState, book::Book, ui_view::UiView},
};
use druid::{
    widget::{
        Button, Container, FillStrat, Flex, Image, Label, List, ListIter, MainAxisAlignment,
        Padding, Painter, Scroll, Svg, ViewSwitcher,
    },
    Color, Command, Data, EventCtx, RenderContext, Target, Widget, WidgetExt,
};

// main page and contains list view of contacts
// notice that this must return Box<dyn Widget<YourState>> instead of impl Widget<YourState>
// navigator needs Boxed widgets in order to store the widgets
pub fn library() -> Box<dyn Widget<AppState>> {
    // Book list
    let list = List::new(|| {
        // Book title
        let book_title = Label::new(
            |(_views, book, _selection, _idx): &(Arc<Vec<UiView>>, Book, Option<usize>, usize),
             _env: &_| { book.get_title() },
        );

        // Book author
        let email_text = Label::new(
            |(_views, book, _selected, _idx): &(Arc<Vec<UiView>>, Book, Option<usize>, usize),
             _env: &_| { book.get_author() },
        );

        // Book cover
        let cover = Flex::row().with_child(ViewSwitcher::new(
            |data: &Book, _env| data.get_image_buf().is_some(),
            move |f, data, _env| {
                if *f {
                    Box::new(
                        Image::new(data.get_image_buf().as_ref().unwrap().as_ref().clone())
                            .fix_size(100.0, 200.0),
                    )
                } else {
                    Box::new(
                        Svg::new(COVER_PLACEHOLDER.parse().unwrap()).fill_mode(FillStrat::Fill),
                    )
                }
            },
        ));

        // Layout of single book
        let details = Flex::column().with_child(book_title).with_child(email_text);
        let layout = Flex::row().with_child(details);

        let layout = layout.on_click(|event, data, _env| {
            let new_views = Arc::make_mut(&mut data.0);
            new_views.push(UiView::BookRead);
            data.0 = Arc::new(new_views.to_owned());
            data.2 = Some(data.3);
            event.submit_command(Command::new(BOOK_READ, data.3, Target::Auto));
        });

        // Highlight book on hover
        layout.background(Painter::new(|ctx, _data, _env| {
            let is_hot = ctx.is_hot();
            let is_active = ctx.is_active();
            let rect = ctx.size().to_rect();
            let background_color = if is_active {
                Color::BLACK
            } else if is_hot {
                Color::BLACK
            } else {
                Color::TRANSPARENT
            };
            ctx.stroke(rect, &background_color, 0.);
            ctx.fill(rect, &background_color);
        }))
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
            data.add_book();
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
        for (idx, contact) in self.library.iter().enumerate() {
            let nav_state = self.nav_state.clone();
            cb(&(nav_state, contact.clone(), self.selected, idx), idx);
        }
    }

    fn for_each_mut(
        &mut self,
        mut cb: impl FnMut(&mut (Arc<Vec<UiView>>, Book, Option<usize>, usize), usize),
    ) {
        let mut any_shared_changed = false;
        for (idx, contact) in self.library.iter().enumerate() {
            let mut d = (self.nav_state.clone(), contact.clone(), self.selected, idx);

            cb(&mut d, idx);
            if !any_shared_changed && !self.nav_state.same(&d.0) {
                any_shared_changed = true;
            }
            if any_shared_changed {
                self.nav_state = d.0;
                self.selected = d.2;
            }
        }
    }

    fn data_len(&self) -> usize {
        self.library.len()
    }
}
