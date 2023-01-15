use epub::doc::EpubDoc;
use html2text::from_read;
use native_dialog::FileDialog;
use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{
    helper::config::{COVERS_PATH, LIBRARY_PATH},
    model::book::Book,
};

/**
 * Open a file dialog and return the path to the selected file.
 */
pub fn open_native_dialog() -> Option<PathBuf> {
    let path = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("EPub", &["epub"])
        .show_open_single_file();

    match path {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error opening file dialog: {}", e);
            None
        }
    }
}

pub fn open_native_dialog_images() -> Option<PathBuf> {
    let path = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("Image", &["png", "jpg", "jpeg"])
        .show_open_single_file();

    match path {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error opening file dialog: {}", e);
            None
        }
    }
}

/**
 * Convert a path into a file and then into a byte array [u8] and return it.
 */
pub fn path_to_bytes(path: PathBuf) -> Option<Vec<u8>> {
    let file = std::fs::File::open(path);
    let file = match file {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: {}", e);
            return None;
        }
    };
    let mut bytes = Vec::new();
    let mut reader = std::io::BufReader::new(file);
    reader.read_to_end(&mut bytes).expect("Unable to read file");
    Some(bytes)
}

/**
* Converts an .epub file into the Book struct
 */
pub fn epub_to_book(path: PathBuf) -> Option<Book> {
    let filename = path
        .file_name()
        .expect("Unable to read file")
        .to_str()
        .expect("Unable to read filename")
        .to_string();
    let file_path = format!("{}/{}", LIBRARY_PATH, filename);

    let doc = EpubDoc::new(path);
    let mut doc = match doc {
        Ok(doc) => doc,
        Err(e) => {
            println!("Error: {}", e);
            return None;
        }
    };

    let title = doc.mdata("title").unwrap_or("Unknown".to_string());
    let author = doc.mdata("creator").unwrap_or("Unknown".to_string());

    // Cover
    let cover_data = match doc.get_cover() {
        Ok(cover_data) => cover_data,
        Err(e) => {
            println!("Error: {}", e);
            return None;
        }
    };
    let mut cover_path = String::from(COVERS_PATH);
    cover_path.push_str(format!("{}.png", title.clone().replace(' ', "-")).as_str());
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

    // Count words in book
    let mut word_count_chapters = Vec::<i32>::new();
    for _i in 0..doc.get_num_pages() - 1 {
        let page_str = doc.get_current_str();
        let text = from_read(page_str.unwrap_or("".to_string()).as_bytes(), 1000);
        match doc.go_next() {
            Ok(_) => (),
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
        word_count_chapters.push(text.split_whitespace().count() as i32);
    }
    println!("Word count: {:?}", word_count_chapters);

    match cover_path {
        None => Some(Book::new(
            doc,
            title,
            author,
            String::new(),
            file_path,
            Arc::new(word_count_chapters),
        )),
        Some(cover_path) => Some(Book::new(
            doc,
            title,
            author,
            cover_path,
            file_path,
            Arc::new(word_count_chapters),
        )),
    }
}
