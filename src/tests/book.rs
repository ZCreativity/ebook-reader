/**
 * Tests that a book is created from an epub file
 */
#[test]
fn book_created_from_epub_file() {
    let file_path = "./src/library/hope-prisoner-of-zenda.epub";
    let book = crate::helper::functions::epub_to_book(std::path::PathBuf::from(file_path));
    match book {
        Some(book) => {
            assert_eq!(book.get_title(), "The Prisoner of Zenda");
            assert_eq!(book.get_author(), "Anthony Hope");
            assert_eq!(book.get_file_path(), file_path);
        }
        None => {
            panic!("Book not created")
        }
    }
}

/**
 * Tests that a book is created from an epub file with a cover
 */
#[test]
fn book_created_from_epub_file_with_cover() {
    let file_path = "./src/library/hope-prisoner-of-zenda.epub";
    let book = crate::helper::functions::epub_to_book(std::path::PathBuf::from(file_path));
    match book {
        Some(book) => {
            assert_eq!(book.get_title(), "The Prisoner of Zenda");
            assert_eq!(book.get_author(), "Anthony Hope");
            assert_eq!(book.get_file_path(), file_path);
            assert!(book.get_image_buf().is_some());
        }
        None => {
            panic!("Book not created")
        }
    }
}

/**
 * Tests that a book is not created from an epub file that does not exist
 */
#[test]
fn book_created_from_epub_file_not_existing() {
    let file_path = "./src/library/unknown.epub";
    let book = crate::helper::functions::epub_to_book(std::path::PathBuf::from(file_path));
    match book {
        Some(book) => {
            assert_eq!(book.get_title(), "");
            assert_eq!(book.get_author(), "");
            assert_eq!(book.get_file_path(), "");
            assert!(book.get_image_buf().is_none());
        }
        None => {
            panic!("Book not created")
        }
    }
}
