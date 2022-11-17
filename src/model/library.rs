use crate::helper::config::LIBRARY_PATH;
use crate::helper::functions::open_native_dialog;
use druid::{Data, Lens};
use std::fs;
use std::fs::ReadDir;
use druid::im::Vector;
use crate::model::book::Book;
use crate::controller::doc_controller::DocController;

#[derive(Data, Lens, Clone)]
pub struct Library {
    books: Vector<Book>,  // Using druid "im" crate for immutable data structures, which can me modified and copied efficiently
}

impl Library {

    /** Initialize library
     */
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
        let mut book_list: Vector<Book> = Vector::new();

        if dir.is_some() {
            for file in dir.unwrap() {
                let book = DocController::epub_to_book(file.as_ref().unwrap().path());
                match book {
                    None => { eprintln!("Unable to add book {}", file.unwrap().path().display()); }
                    Some(book) => { book_list.push_back(book); }
                }
            }

            return Self { books: book_list };
        }

        // No files found
        Self { books: Vector::new() }
    }

    /**
    Adds new book to the library
     */
    pub fn add_book(&mut self) {
        let path = open_native_dialog();
        let path = match path {
            None => {
                println!("No book selected");
                return;
            }
            Some(path) => path,
        };

        let book = DocController::epub_to_book(path.clone());
        let filename = path.clone().file_name().unwrap().to_str().unwrap().to_string();
        let to = format!("{}/{}", LIBRARY_PATH, filename);
        let result = fs::copy(path.clone(), to);
        match result {
            Ok(_) => {
                match book {
                    None => { eprintln!("Unable to generate ebook for this file")}
                    Some(book) => {
                        self.books.push_back(book);
                    }
                }
                println!("Book added successfully")
            }
            Err(e) => {
                eprintln!("Error adding the book: {}", e)
            }
        }
    }
}
