use std::{sync::Arc, path::PathBuf, ffi::OsString};
use druid::{widget::ScopeTransfer, Data, Lens};
use crate::helper::config::LIBRARY_PATH;
use super::{app_state::AppState, book::Book};

use zip::{ZipArchive};
use std::{fs::{File, self}, io::Write};

// this holds state that will be used when on the edit page
#[derive(Clone, Data, Lens, Debug)]
pub struct EditState {
    pub book: Book,
    pub index: usize,
    pub was_saved: bool,
}

impl EditState {
    pub fn new(data: AppState) -> Self {
        let (book, index) = if let Some(idx) = data.get_selected() {
            (data.get_library()[idx].clone(), idx)
        } else {
            (Book::new_empty(), 0)
        };
        Self {
            book,
            index,
            was_saved: false,
        }
    }
}

pub struct EditTransfer;

impl ScopeTransfer for EditTransfer {
    type In = AppState;

    type State = EditState;

    fn read_input(&self, state: &mut Self::State, inner: &Self::In) {
        // Only read data in if the input was saved
        if state.was_saved {
            let selected = inner.get_selected();
            let idx = if let Some(idx) = selected { idx } else { 0 };
            state.book = inner.get_library()[idx].clone();
            state.index = idx;
            state.was_saved = false;
        }
    }

    fn write_back_input(&self, state: &Self::State, inner: &mut Self::In) {
        if state.was_saved {
            // Here the current_page_str of the book is updated with changes made in the edit page
            println!("Saving book: {:?}", state.book);

            // TODO: Update the library with the edited book, to reflect the changes instantly (Doesn't work yet)
            let books = Arc::make_mut(&mut inner.library);
            books[state.index] = state.book.clone();
            inner.library = Arc::new(books.to_owned());

            println!("Library: {:?}", inner.library);

            // Update the selected book html file with the changes made in the edit page
            let page_to_update = state.book.get_current_page_str();
            let file_to_update = state.book.get_current_doc_path();
            match file_to_update {
                None => {
                    eprintln!("Unable to update file");
                }
                Some(file) => {
                    
                    let zip_path = "src/library/copy.zip";
                    let new_zip_path = "src/library/new.zip";
                    let dir_path = "src/library/new";
                    let edited_html = page_to_update.as_str();

                    let path_buf = file.to_path_buf();
                    let file_path = path_buf.as_os_str().to_str()
                        .expect("Error while retrieving file path");
                    let dir_file_path = [dir_path,file_path].join("/");
                    
                    let epub_path = state.book.get_file_path();
                    let new_epub_path = epub_path.replace(".epub", "-edit.epub");

                    //Converting epub into zip
                    fs::copy(epub_path, zip_path)
                        .expect("Error encountered while copying file!");
            
                    let zip_file = File::options().write(true).read(true).open(zip_path).unwrap();

                    //Extract the zip 
                    let mut archive = ZipArchive::new(zip_file).unwrap();
                    archive.extract(dir_path)
                        .expect("Error encountered while extracting zip files");
                    
                    //Replace old file with the edit one
                    let mut edited_file =  File::create(dir_file_path.as_str())
                        .expect("Error encountered while editing old file!");
                    edited_file.write_all(edited_html.as_bytes())
                        .expect("Error encountered while editing old file!");

                    //Replace the title
                    
                    
                    let dir_metadata_path = [dir_path,"OEBPS/package.opf"].join("/");
                    let edited_metadata_string = fs::read_to_string(dir_metadata_path.as_str())
                        .expect("Error encountered while reading metadata file!");
                    
                    let mut metadata_file = File::create(dir_metadata_path.as_str())
                        .expect("Error encountered while editing metadata file!");
                    metadata_file.write_all(edited_metadata_string.replace("</dc:title>",  " (edit)</dc:title>").as_bytes())
                        .expect("Error encountered while editing metafile");
                    
                    //Converting directory into zip
                    let new_zip_path_buf = PathBuf::from(OsString::from(new_zip_path));
                    let dir_path_buf = PathBuf::from(OsString::from(dir_path));
                    zip_extensions::write::zip_create_from_directory(&new_zip_path_buf, &dir_path_buf)
                        .expect("Error while zipping the directory");

                    //Converting zip into epub
                    fs::copy(new_zip_path, new_epub_path)
                        .expect("Error encountered while copying file!");

                    //Delete unnecesesary file
                    fs::remove_file(zip_path)
                        .expect("Error while deleting old zip");
                    fs::remove_file(new_zip_path)
                        .expect("Error while deleting new zip");
                    fs::remove_dir_all(dir_path)
                        .expect("Error encountered while deleting directory");
                    
                }
            }
        }
    }
}
