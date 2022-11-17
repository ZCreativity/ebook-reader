use druid::{Data, ImageBuf, Lens};
use epub::doc::EpubDoc;
use std::fs::File;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::helper::functions::path_to_bytes;

#[derive(Data, Clone, Lens)]
pub struct Book {
    title: String,
    cover: Option<Arc<ImageBuf>>,
    doc: Arc<Mutex<EpubDoc<File>>>,
}

impl Book {
    pub fn new(doc: EpubDoc<File>, title: String, cover_path: String) -> Self {
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
            doc: Arc::new(Mutex::new(doc)),
            title,
            cover,
        }
    }

    pub fn get_image_buf(&self) -> Option<Arc<ImageBuf>> {
        match &self.cover {
            Some(cover) => Some(cover.clone()),
            None => None,
        }
    }
}
