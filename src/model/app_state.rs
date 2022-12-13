use super::book::Book;
use crate::model::library::Library;
use druid::{Data, Lens};

#[derive(Data, Clone, Lens)]
pub struct AppState {
    library: Library,
    is_reading_book: bool,
    opened_book: Book,
    current_page: usize,
}

impl AppState {
    pub fn new() -> Self {
        let empty_book = Book::new_empty();
        Self {
            library: Library::new(),
            is_reading_book: false,
            opened_book: empty_book,
            current_page: 1,
        }
    }

    pub fn add_book(&mut self) {
        self.library.add_book();
    }

    pub fn open_book(&mut self, book: Book) {
        self.opened_book = book;
        self.is_reading_book = true;
    }

    pub fn close_book(&mut self) {
        self.is_reading_book = false;
    }

    pub fn get_is_reading_book(&self) -> bool {
        self.is_reading_book
    }
}
