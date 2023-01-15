use crate::helper::{
    config::{DEFAULT_FONT_SIZE, LIBRARY_PATH},
    functions::{epub_to_book, open_native_dialog, open_native_dialog_images},
};

use super::{book::Book, ui_view::UiView};
use druid::{Data, Lens};

use std::{
    fs::{self, ReadDir},
    path::PathBuf,
    rc::Rc,
    sync::Arc,
};

#[derive(Clone, Data, Lens, Debug)]
pub struct AppState {
    // this will act as the backing data for your navigation state
    // this should always be initialized with one view and should
    // ideally never be empty, otherwise things might not work correctly
    pub nav_state: Arc<Vec<UiView>>,
    pub library: Arc<Vec<Book>>,
    selected: Option<usize>,
    font_size: f64,
}

impl AppState {
    pub fn new() -> Self {
        let library = Self::initialize_library();
        Self {
            library: Arc::new(library),
            nav_state: Arc::new(vec![UiView::Library]),
            selected: None,
            font_size: DEFAULT_FONT_SIZE,
        }
    }

    /**
     * Initialize library, scan the "library" folder and add all the books
     */
    fn initialize_library() -> Vec<Book> {
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
                let book = epub_to_book(file.as_ref().unwrap().path());
                match book {
                    None => {
                        eprintln!("Unable to add book {}", file.unwrap().path().display());
                    }
                    Some(book) => {
                        book_list.push(book);
                    }
                }
            }
            return book_list;
        }

        // No files found
        Vec::new()
    }

    /**
     * Adds new book to the library
     */
    pub fn add_book_from_file(&mut self) {
        let path = open_native_dialog();
        let path = match path {
            None => {
                println!("No book selected");
                return;
            }
            Some(path) => path,
        };

        let book = epub_to_book(path.clone());
        let filename = path.file_name().unwrap().to_str().unwrap().to_string();
        let to = format!("{}/{}", LIBRARY_PATH, filename);
        let result = fs::copy(path, to);
        match result {
            Ok(_) => {
                match book {
                    None => {
                        eprintln!("Unable to generate ebook for this file")
                    }
                    Some(book) => {
                        let library = Arc::make_mut(&mut self.library);
                        library.push(book);
                    }
                }
                println!("Book added successfully")
            }
            Err(e) => {
                eprintln!("Error adding the book: {}", e)
            }
        }
    }

    pub fn add_book_from_path(&mut self, file: PathBuf) {
        self.add_book(Some(file))
    }

    fn add_book(&mut self, path: Option<PathBuf>) {
        let book = epub_to_book(path.unwrap());
        match book {
            Some(book) => {
                let library = Arc::make_mut(&mut self.library);
                library.push(book);
            }
            None => {
                eprintln!("Error adding the book")
            }
        }
    }

    /**
     * Get reference to the library
     */
    pub fn get_library(&self) -> Arc<Vec<Book>> {
        self.library.clone()
    }

    /**
     * Get reference to the selected book
     */
    pub fn get_selected(&self) -> Option<usize> {
        self.selected
    }

    /**
     * Set the selected book
     */
    pub fn set_selected(&mut self, selected: Option<usize>) {
        self.selected = selected;
    }

    /**
     * Font size methods
     */
    pub fn get_font_size(&self) -> f64 {
        self.font_size
    }

    pub fn increase_font_size(&mut self) {
        if self.font_size >= 20.0 {
            self.font_size = 20.0;
            return;
        }
        self.font_size += 2.0;
    }

    pub fn decrease_font_size(&mut self) {
        if self.font_size <= 4.0 {
            self.font_size = 4.0;
            return;
        }
        self.font_size -= 2.0;
    }

    /**
     * Book navigation methods
     */
    pub fn navigate_to_page(&mut self, link: Rc<String>) {
        if self.selected.is_none() {
            return;
        }
        // Call the navigate_to method of the selected book
        let library = Arc::make_mut(&mut self.library);
        let book = library.get_mut(self.selected.unwrap()).unwrap();
        book.navigate_to(link);
    }

    pub fn has_next_page(&self) -> bool {
        if self.selected.is_none() {
            return false;
        }
        // Call the has_next_page method of the selected book
        let library = self.library.clone();
        let book = library.get(self.selected.unwrap()).unwrap();
        book.has_next_page()
    }

    pub fn has_prev_page(&self) -> bool {
        if self.selected.is_none() {
            return false;
        }
        // Call the has_prev_page method of the selected book
        let library = self.library.clone();
        let book = library.get(self.selected.unwrap()).unwrap();
        book.has_prev_page()
    }

    pub fn navigate_to_next_page(&mut self) {
        if self.selected.is_none() {
            return;
        }
        // Call the navigate_to_next_page method of the selected book
        let library = Arc::make_mut(&mut self.library);
        let book = library.get_mut(self.selected.unwrap()).unwrap();
        book.next_page();
    }

    pub fn navigate_to_prev_page(&mut self) {
        if self.selected.is_none() {
            return;
        }
        // Call the navigate_to_prev_page method of the selected book
        let library = Arc::make_mut(&mut self.library);
        let book = library.get_mut(self.selected.unwrap()).unwrap();
        book.prev_page();
    }

    pub fn navigate_to_page_index(&mut self, page: usize) {
        let library = Arc::make_mut(&mut self.library);
        let book = library.get_mut(self.selected.unwrap()).unwrap();
        book.set_page(page);
    }

    pub fn navigate_to_first_page(&mut self) {
        self.navigate_to_page_index(1);
    }

    pub fn navigate_to_last_page(&mut self) {
        let library = Arc::make_mut(&mut self.library);
        let book = library.get_mut(self.selected.unwrap()).unwrap();
        book.set_page(book.get_book_length());
    }

    /** EDIT */

    /**
     * Sets the current page as the editing page, in order to be
     * displayed in the editor
     */
    pub fn set_editing_page(&mut self) {
        if self.selected.is_none() {
            return;
        }
        // Call the set_editing_page method of the selected book
        let library = Arc::make_mut(&mut self.library);
        let book = library.get_mut(self.selected.unwrap()).unwrap();
        let page_string = book.get_page_str(book.get_current_page());
        match page_string {
            None => {
                println!("Error getting page string");
                return;
            }
            Some(page) => {
                book.set_page_str(page);
            }
        }
    }

    /** Save book progress */
    pub fn save_book_progress(&mut self) {
        if self.selected.is_none() {
            return;
        }
        let library = Arc::make_mut(&mut self.library);
        let book = library.get_mut(self.selected.unwrap()).unwrap();
        match book.save_progress() {
            Ok(_) => println!("Saved"),
            Err(err) => eprintln!("Not saved, {}", err),
        }
    }

    /** OCR */

    /**
     * Open a native dialog to select an image file
     * then run OCR on it, extract the text and find the page where
     * text is located
     */
    pub fn ocr_from_file(&mut self) {
        let selected_file = open_native_dialog_images();
        match selected_file {
            Some(file) => {
                let text = tesseract::ocr(file.as_os_str().to_str().unwrap(), "eng");
                match text {
                    Ok(text) => {
                        println!("Text: {}", text);
                        let book = self.get_library()[self.get_selected().unwrap()].clone();
                        let page = book.get_page_from_ocr_text(text.trim().to_string());
                        println!("Page: {:?}", page);
                        match page {
                            Some(page) => {
                                self.navigate_to_page_index(page);
                            }
                            None => {
                                eprintln!("No page found");
                            }
                        }
                    }
                    Err(err) => println!("Error {:?}", err),
                }
            }
            None => {
                eprintln!("No file selected")
            }
        }
    }

    /**
     * Reverse OCR
     * Get the word count from page 1 to current page, then
     * exstimate a total of 300-400 words for page in a real book
     * and existimate the possible physical page.
     */
    pub fn reverse_ocr(&mut self) {
        let library = Arc::make_mut(&mut self.library);
        let book = library.get_mut(self.selected.unwrap()).unwrap();
        book.reverse_ocr()
    }
}
