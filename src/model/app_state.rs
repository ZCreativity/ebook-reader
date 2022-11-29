use crate::model::library::Library;
use druid::{Data, Lens};

use super::book::Book;

#[derive(Data, Clone, Lens)]
pub struct AppState {
    library: Library,
    opened_book: Option<Book>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            library: Library::new(),
            opened_book: None,
        }
    }

    pub fn add_book(&mut self) {
        self.library.add_book();
    }

    pub fn open_book(&mut self, book: Book) {
        self.opened_book = Some(book);
    }

    pub fn close_book(&mut self) {
        self.opened_book = None;
    }

    pub fn get_opened_book(&self) -> Option<Book> {
        self.opened_book.clone()
    }
}
