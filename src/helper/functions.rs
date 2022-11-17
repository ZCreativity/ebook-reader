use native_dialog::FileDialog;
use std::{io::Read, path::PathBuf};

/**
 * Open a file dialog and return the path to the selected file.
 */
pub fn open_native_dialog() -> Option<PathBuf> {
    let path = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("EPub", &["epub"])
        .show_open_single_file()
        .unwrap();
    path
}

/**
 * Convert a path into a file and then into a byte array [u8] and return it.
 */
pub fn path_to_bytes(path: PathBuf) -> Option<Vec<u8>> {
    let file = std::fs::File::open(path).unwrap();
    let mut bytes = Vec::new();
    let mut reader = std::io::BufReader::new(file);
    reader.read_to_end(&mut bytes).unwrap();
    Some(bytes)
}
