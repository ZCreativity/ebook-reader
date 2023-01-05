use std::sync::Arc;

use druid::{widget::ScopeTransfer, Data, Lens};

use super::{app_state::AppState, book::Book};

// this holds state that will be used when on the edit page
#[derive(Clone, Data, Lens, Debug)]
pub struct EditState {
    pub book: Book,
    pub index: usize,
    pub was_saved: bool,
}

impl EditState {
    pub fn new(data: AppState) -> Self {
        let (book, index) = if let Some(idx) = data.get_selected() {
            (data.get_library()[idx].clone(), idx)
        } else {
            (Book::new_empty(), 0)
        };
        Self {
            book,
            index,
            was_saved: false,
        }
    }
}

pub struct EditTransfer;

impl ScopeTransfer for EditTransfer {
    type In = AppState;

    type State = EditState;

    fn read_input(&self, state: &mut Self::State, inner: &Self::In) {
        // Only read data in if the input was saved
        if state.was_saved {
            let selected = inner.get_selected();
            let idx = if let Some(idx) = selected { idx } else { 0 };
            state.book = inner.get_library()[idx].clone();
            state.index = idx;
            state.was_saved = false;
        }
    }

    fn write_back_input(&self, state: &Self::State, inner: &mut Self::In) {
        if state.was_saved {
            // Here the current_page_str of the book is updated with changes made in the edit page
            println!("Saving book: {:?}", state.book);

            // TODO: Update the library with the edited book, to reflect the changes instantly (Doesn't work yet)
            let books = Arc::make_mut(&mut inner.library);
            books[state.index] = state.book.clone();
            inner.library = Arc::new(books.to_owned());

            println!("Library: {:?}", inner.library)

            // TODO: Update the selected book html file
        }
    }
}
