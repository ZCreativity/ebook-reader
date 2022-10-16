use std::path::PathBuf;
use native_dialog::{FileDialog};

pub fn open_native_dialog() -> Option<PathBuf> {
    let path = FileDialog::new().set_location("~/Desktop").add_filter("EPub", &["epub"]).show_open_single_file().unwrap();
    path
}