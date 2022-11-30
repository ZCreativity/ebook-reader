use crate::helper::config::COVERS_PATH;
use crate::model::book::Book;
use epub::doc::EpubDoc;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct DocController {}

impl DocController {
    /**
    Converts an .epub file into the Book struct
     */
    pub fn epub_to_book(path: PathBuf) -> Option<Book> {
        let doc = EpubDoc::new(path);
        let mut doc = match doc {
            Ok(doc) => doc,
            Err(e) => {
                println!("Error: {}", e);
                return None;
            }
        };

        let title = doc.mdata("title").unwrap();
        let author = doc.mdata("creator").unwrap();

        // Cover
        let cover_data = doc.get_cover().unwrap();
        let mut cover_path = String::from(COVERS_PATH);
        cover_path
            .push_str(format!("{}.png", doc.mdata("title").unwrap().replace(' ', "-")).as_str());
        let path = Path::new(cover_path.as_str());
        println!("Path: {:?}", path);
        let f = fs::File::create(path);
        let cover_path = match f {
            Ok(mut file) => {
                let resp = file.write_all(&cover_data);
                match resp {
                    Ok(_) => {
                        println!("Book cover path: {}", cover_path);
                        Some(cover_path)
                    }
                    Err(_) => {
                        eprintln!("Unable to fetch cover");
                        None
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                None
            }
        };

        match cover_path {
            None => Some(Book::new(doc, title, author, String::new())),
            Some(cover_path) => Some(Book::new(doc, title, author, cover_path)),
        }
    }
}
