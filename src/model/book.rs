use std::fs::File;
use std::sync::Arc;
use druid::{Data, Lens};
use epub::doc::EpubDoc;

#[derive(Data, Clone, Lens)]
pub struct Book {
    title: String,
    doc: Arc<EpubDoc<File>>
}

impl Book {
    pub fn new(doc: EpubDoc<File>, title: String) -> Self {
        Self { doc: Arc::new(doc), title}
    }
    
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
}