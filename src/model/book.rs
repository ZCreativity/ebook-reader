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
}
