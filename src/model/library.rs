use crate::helper::functions::open_native_dialog;

pub struct Library {}

impl Library {

    /**
    Adds new book to the library
     */
    pub fn add_book() {
        let path = open_native_dialog();
        let path = match path {
            None => { println!("No book selected"); return; }  //TODO: Handle error
            Some(path) => { path }
        };



        println!("Added book {:?}", path);
    }
}
