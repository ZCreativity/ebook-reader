use crate::model::library::Library;
use druid::{Data, Lens};

#[derive(Data, Clone, Lens)]
pub struct AppState {
    library: Library,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            library: Library::new(),
        }
    }

    pub fn add_book(&mut self) {
        self.library.add_book();
    }
}
