use druid::{Data, ImageBuf, Lens};
use epub::doc::EpubDoc;
use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::helper::config::SAVED_PROGRESS_PATH;
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
    has_progress: bool,
    word_count_chapters: Arc<Vec<i32>>,
    physical_page_range: Option<(i32, i32)>,
}

impl Book {
    pub fn new(
        doc: EpubDoc<BufReader<File>>,
        title: String,
        author: String,
        cover_path: String,
        file_path: String,
        word_count_chapters: Arc<Vec<i32>>,
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

        // Check if the book has a title.json file in progress folder
        let progress = Path::new(SAVED_PROGRESS_PATH)
            .join(title.clone().replace(" ", "-") + ".json")
            .exists();

        let mut current_page_index = 1;
        if progress {
            let saved_progress_path =
                SAVED_PROGRESS_PATH.to_owned() + &title.as_str().replace(" ", "-") + ".json";
            let file = File::open(saved_progress_path);
            current_page_index = match file {
                Ok(file) => {
                    let reader = BufReader::new(file);
                    let json: usize = serde_json::from_reader(reader).unwrap();
                    json
                }
                Err(e) => {
                    eprintln!("Error getting progress: {}", e);
                    1
                }
            }
        }

        // Create book
        Self {
            doc: Some(Arc::new(Mutex::new(doc))),
            title,
            author,
            cover,
            current_page_index,
            current_page_str: String::new(),
            file_path,
            has_progress: progress,
            word_count_chapters,
            physical_page_range: None,
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
            has_progress: false,
            word_count_chapters: Arc::new(Vec::new()),
            physical_page_range: None,
        }
    }

    /** Getters */
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

    pub fn get_file_path(&self) -> String {
        self.file_path.clone()
    }

    pub fn get_word_count(&self) -> Arc<Vec<i32>> {
        self.word_count_chapters.clone()
    }

    pub fn get_current_page_str(&self) -> String {
        self.current_page_str.clone()
    }

    /** Edit */
    pub fn set_page_str(&mut self, page_str: String) {
        self.current_page_str = page_str;
    }

    /** Book navigation methods */
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
        self.reset_page_range();
        if self.has_next_page() {
            self.current_page_index += 1;
        }
    }

    pub fn prev_page(&mut self) {
        self.reset_page_range();
        if self.has_prev_page() {
            self.current_page_index -= 1;
        }
    }

    pub fn set_page(&mut self, page_index: usize) {
        self.reset_page_range();
        self.current_page_index = page_index;
    }

    /**
     * doc.resources
     * It’s a HashMap<a: String, (b: PathBuf, c: String)> where ‘a’ is the resource id, ‘b’ is the resource full path and ‘c’ is the resource mimetype
     * Get the resource id of the resource which the resource full path ends with the link
     */
    pub fn navigate_to(&mut self, link: Rc<String>) {
        self.reset_page_range();
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

    /**
     * Get the page index from ocr text.
     */
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

    /**
     * Get the physical page from chapter page.
     */
    pub fn reverse_ocr(&mut self) {
        let page = self.get_current_page();

        let word_count_chapter = self.get_word_count();
        let word_count_till_page = word_count_chapter.iter().take(page + 1).sum::<i32>();
        let page_range_start = word_count_till_page / 450;
        let page_range_end = word_count_till_page / 300;
        println!("Page range start: {}", page_range_start);
        println!("Page range end: {}", page_range_end);
        // +1 because the page 0 on a physical book has weird numbering
        self.set_physical_page_range((page_range_start + 1, page_range_end + 1));
    }

    pub fn set_physical_page_range(&mut self, page_range: (i32, i32)) {
        self.physical_page_range = Some(page_range);
    }

    pub fn reset_page_range(&mut self) {
        self.physical_page_range = None;
    }

    pub fn get_physical_page_range(&self) -> Option<(i32, i32)> {
        self.physical_page_range
    }

    /**
     * Save the current page to a json file (for keeping track of the reading page)
     */
    pub fn save_progress(&mut self) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string(&self.current_page_index)?;
        let saved_progress_path =
            SAVED_PROGRESS_PATH.to_owned() + &self.title.as_str().replace(" ", "-") + ".json";
        let mut file = File::create(saved_progress_path)?;
        file.write_all(json.as_bytes())?;
        self.set_has_progress();
        Ok(())
    }

    pub fn get_has_progress(&self) -> bool {
        self.has_progress
    }

    pub fn set_has_progress(&mut self) {
        self.has_progress = true;
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
            .field("word_count", &self.word_count_chapters)
            .finish()
    }
}
