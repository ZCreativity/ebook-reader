use std::fs;
use std::fs::ReadDir;
use std::sync::Arc;
use druid::{Data, Lens};
use crate::controller::doc_controller::DocController;
use crate::helper::config::LIBRARY_PATH;
use crate::model::book::Book;


#[derive(Data, Clone, Lens)]
pub struct AppState {
    books_list: Arc<Vec<Book>>,
}

impl AppState {
    pub fn new() -> Self {
        // Scan through library folder and init books
        let dir = fs::read_dir(LIBRARY_PATH);
        let dir: Option<ReadDir> = match dir {
            Ok(dir) => Some(dir),
            Err(e) => {
                println!("Error: {:?}", e);
                None
            }
        };

        // Book list
        let mut book_list: Vec<Book> = Vec::new();

        if dir.is_some() {
            for file in dir.unwrap() {
                let book = DocController::epub_to_book(file.as_ref().unwrap().path());
                match book {
                    None => {
                        eprintln!("Unable to add book {}", file.unwrap().path().display());
                    }
                    Some(book) => { book_list.push(book); }
                }
            }

            return Self { books_list: Arc::new(book_list)};
        }

        // No files found
        Self { books_list: Arc::new(Vec::new()) }
    }
}