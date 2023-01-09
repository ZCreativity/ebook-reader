use druid::{Data, ImageBuf, Lens};
use epub::doc::EpubDoc;
use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::helper::functions::path_to_bytes;

#[derive(Data, Clone, Lens)]
pub struct Book {
    title: String,
    author: String,
    cover: Option<Arc<ImageBuf>>,
    doc: Option<Arc<Mutex<EpubDoc<BufReader<File>>>>>,
    current_page_index: usize,
    current_page_str: String,
    file_path: String,
}

impl Book {
    pub fn new(
        doc: EpubDoc<BufReader<File>>,
        title: String,
        author: String,
        cover_path: String,
        file_path: String,
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
            current_page_str: String::new(),
            file_path,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            doc: None,
            title: String::new(),
            author: String::new(),
            cover: None,
            current_page_index: 0,
            current_page_str: String::new(),
            file_path: String::new(),
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

    pub fn get_author(&self) -> String {
        self.author.clone()
    }

    pub fn get_book_length(&self) -> usize {
        match &self.doc {
            Some(doc) => doc.lock().unwrap().spine.len() - 1,
            None => 0,
        }
    }

    pub fn get_current_page(&self) -> usize {
        self.current_page_index
    }

    pub fn has_next_page(&self) -> bool {
        self.current_page_index < self.get_book_length() - 1
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

    pub fn set_page(&mut self, page_index: usize) {
        self.current_page_index = page_index;
    }

    pub fn get_current_page_str(&self) -> String {
        self.current_page_str.clone()
    }

    pub fn set_page_str(&mut self, page_str: String) {
        self.current_page_str = page_str;
    }

    pub fn get_file_path(&self) -> String {
        self.file_path.clone()
    }

    /**
     * doc.resources
     * It’s a HashMap<a: String, (b: PathBuf, c: String)> where ‘a’ is the resource id, ‘b’ is the resource full path and ‘c’ is the resource mimetype
     * Get the resource id of the resource which the resource full path ends with the link
     */
    pub fn navigate_to(&mut self, link: Rc<String>) {
        let binding = self.get_doc().unwrap();
        let doc = binding.lock().unwrap();

        // From "chapter_001.xhtml" to resource_id in the spine
        let resource_id = doc
            .resources
            .iter()
            .find(|(_, (path, _))| path.ends_with(link.as_str()))
            .map(|(id, _)| id);

        match resource_id {
            Some(resource_id) => {
                let page_index = doc.resource_id_to_chapter(resource_id);
                println!(
                    "Navigating to link: {} (page_index: {:?})",
                    link, page_index
                );
                self.set_page(page_index.unwrap_or(1))
            }
            None => {
                eprintln!("Error navigating to link: {}", link);
            }
        }
    }

    pub fn navigate_to_index(&mut self) {
        let binding = self.get_doc().unwrap();
        let doc = binding.lock().unwrap();

        let resource_id = doc
            .resources
            .iter()
            .find(|(_, (path, _))| path.ends_with("OEBPS/bk01-toc.xhtml"))
            .map(|(id, _)| id);

        match resource_id {
            Some(resource_id) => {
                let page_index = doc.resource_id_to_chapter(resource_id);
                println!("Navigating to index");
                self.set_page(page_index.unwrap_or(1))
            }
            None => {
                eprintln!("Error navigating to index");
            }
        }
    }

    /**
     * Get the current doc path
     * Example: OEBPS/chapter_001.xhtml (relative path to the epub file)
     */
    pub fn get_current_doc_path(&self) -> Option<PathBuf> {
        let doc = self.get_doc().unwrap();
        let doc = doc.lock().unwrap();
        match doc.get_current_path() {
            Ok(path) => Some(path),
            Err(e) => {
                eprintln!("Error getting current doc path: {}", e);
                None
            }
        }
    }

    pub fn get_page_str(&self, page_index: usize) -> Option<String> {
        if page_index > 0 && page_index <= self.get_book_length() {
            let doc = self.get_doc().unwrap();
            let mut doc_mut = doc.lock().unwrap();
            doc_mut.set_current_page(page_index).unwrap();
            match doc_mut.get_current_str() {
                Ok(current_str) => Some(current_str),
                Err(err) => {
                    println!("{:?}", err);
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn get_page_from_ocr_text(&self, text: String) -> Option<usize> {
        // Iterate through all the text files in the epub doc
        let doc = self.get_doc().unwrap();
        let mut doc_mut = doc.lock().unwrap();
        let mut page_index = 1;
        while page_index <= doc_mut.spine.len() - 1 {
            doc_mut.set_current_page(page_index).unwrap();
            match doc_mut.get_current_str() {
                Ok(current_str) => {
                    if current_str.contains(&text) {
                        println!("Found text in page: {}", page_index);
                        return Some(page_index);
                    }
                }
                Err(err) => {
                    eprintln!("No page found, error: {:?}", err);
                    return None;
                }
            }
            page_index += 1;
        }
        return None;
    }
}

impl Debug for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Book")
            .field("title", &self.title)
            .field("author", &self.author)
            .field("cover", &self.cover)
            .field("current_page_index", &self.current_page_index)
            .field("current_page_str", &self.current_page_str)
            .finish()
    }
}
