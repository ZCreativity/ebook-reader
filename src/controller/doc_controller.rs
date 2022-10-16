use std::path::PathBuf;
use epub::doc::EpubDoc;
use crate::model::book::Book;

pub struct DocController {}

impl DocController {

    /**
    Converts an .epub file into the Book struct
     */
    pub fn epub_to_book(path: PathBuf) -> Option<Book> {
        let doc = EpubDoc::new(path);
        let doc = match doc {
            Ok(doc) => { doc }
            Err(e) => {
                println!("Error: {}", e);
                return None;
            }
        };

        let title = doc.mdata("title").unwrap();
        println!("Title: {}", title);
        Some(Book::new(doc, title))
    }
}