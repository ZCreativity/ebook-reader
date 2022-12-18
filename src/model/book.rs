use druid::{Data, ImageBuf, Lens};
use epub::doc::EpubDoc;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::helper::functions::path_to_bytes;

#[derive(Data, Clone, Lens)]
pub struct Book {
    title: String,
    author: String,
    cover: Option<Arc<ImageBuf>>,
    doc: Option<Arc<Mutex<EpubDoc<BufReader<File>>>>>,
    current_page_index: usize,
    font_size_offset: f64,
}

impl Book {
    pub fn new(
        doc: EpubDoc<BufReader<File>>,
        title: String,
        author: String,
        cover_path: String,
    ) -> Self {
        // Extract cover image from cover_path
        let cover = if cover_path.is_empty() {
            None
        } else {
            let cover_path = PathBuf::from(cover_path);
            let bytes = path_to_bytes(cover_path).unwrap();
            match ImageBuf::from_data(bytes.as_slice()) {
                Ok(cover) => Some(Arc::new(cover)),
                Err(e) => {
                    eprintln!("Error loading cover: {}", e);
                    None
                }
            }
        };

        // Create book
        Self {
            doc: Some(Arc::new(Mutex::new(doc))),
            title,
            author,
            cover,
            current_page_index: 1,
            font_size_offset: 0.0,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            doc: None,
            title: String::new(),
            author: String::new(),
            cover: None,
            current_page_index: 0,
            font_size_offset: 0.0,
        }
    }

    pub fn get_image_buf(&self) -> Option<Arc<ImageBuf>> {
        self.cover.as_ref().cloned()
    }

    pub fn get_doc(&self) -> Option<Arc<Mutex<EpubDoc<BufReader<File>>>>> {
        self.doc.as_ref().cloned()
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_book_length(&self) -> usize {
        match &self.doc {
            Some(doc) => doc.lock().unwrap().spine.len(),
            None => 0,
        }
    }

    pub fn get_current_page(&self) -> usize {
        self.current_page_index
    }

    pub fn has_next_page(&self) -> bool {
        self.current_page_index < self.get_book_length()
    }

    pub fn has_prev_page(&self) -> bool {
        self.current_page_index > 1
    }

    pub fn next_page(&mut self) {
        if self.has_next_page() {
            self.current_page_index += 1;
        }
    }

    pub fn prev_page(&mut self) {
        if self.has_prev_page() {
            self.current_page_index -= 1;
        }
    }

    pub fn get_page_str(&self, page_index: usize) -> Option<String> {
        if page_index > 0 && page_index <= self.get_book_length() {
            let doc = self.get_doc().unwrap();
            let mut doc_mut = doc.lock().unwrap();
            doc_mut.set_current_page(page_index).unwrap();
            Some(doc_mut.get_current_str().unwrap())
        } else {
            None
        }
    }

    pub fn get_font_size_offset(&self) -> f64 {
        self.font_size_offset
    }

    pub fn increase_font_size(&mut self) {
        self.font_size_offset += 2.0;
    }

    pub fn decrease_font_size(&mut self) {
        self.font_size_offset -= 2.0;
    }
}
