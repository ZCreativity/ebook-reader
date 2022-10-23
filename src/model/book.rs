use std::fs::File;
use std::path::{PathBuf};
use std::sync::{Arc, Mutex};
use druid::{Data, Lens};
use epub::doc::EpubDoc;

#[derive(Data, Clone, Lens)]
pub struct Book {
    title: String,
    pub cover_path: String,
    doc: Arc<Mutex<EpubDoc<File>>>
}

impl Book {
    pub fn new(doc: EpubDoc<File>, title: String, cover_path: String) -> Self {
        Self { doc: Arc::new(Mutex::new(doc)), title, cover_path }
    }

    pub fn get_cover_path(&self) -> String {
        self.cover_path.clone()
    }
}