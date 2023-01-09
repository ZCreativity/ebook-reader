/** EBOOK CREATION TESTS */
const TEST_FILE_PATH: &str = "./src/library/hope-prisoner-of-zenda.epub";

/**
 * Tests that a book is created from an epub file
 */
#[test]
fn book_created_from_epub_file() {
    let book = crate::helper::functions::epub_to_book(std::path::PathBuf::from(TEST_FILE_PATH));
    match book {
        Some(book) => {
            assert_eq!(book.get_title(), "The Prisoner of Zenda");
            assert_eq!(book.get_author(), "Anthony Hope");
            assert_eq!(book.get_file_path(), TEST_FILE_PATH);
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
    let book = crate::helper::functions::epub_to_book(std::path::PathBuf::from(TEST_FILE_PATH));
    match book {
        Some(book) => {
            assert_eq!(book.get_title(), "The Prisoner of Zenda");
            assert_eq!(book.get_author(), "Anthony Hope");
            assert_eq!(book.get_file_path(), TEST_FILE_PATH);
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
    assert_eq!(book.is_none(), true);
}

/** BOOK NAVIGATION TESTS */

/**
 * Tests that the first page of a book is loaded
 */
#[test]
fn first_page_of_book_loaded() {
    let book = crate::helper::functions::epub_to_book(std::path::PathBuf::from(TEST_FILE_PATH));
    match book {
        Some(book) => {
            assert_eq!(book.get_current_page(), 1);
        }
        None => {
            panic!("Book not created")
        }
    }
}

/**
 * Tests that the next page of a book is loaded
 */
#[test]
fn next_page_of_book_loaded() {
    let book = crate::helper::functions::epub_to_book(std::path::PathBuf::from(TEST_FILE_PATH));
    match book {
        Some(mut book) => {
            book.next_page();
            assert_eq!(book.get_current_page(), 2);
        }
        None => {
            panic!("Book not created")
        }
    }
}

/**
 * Tests that the previous page of a book is loaded
 */
#[test]
fn previous_page_of_book_loaded() {
    let book = crate::helper::functions::epub_to_book(std::path::PathBuf::from(TEST_FILE_PATH));
    match book {
        Some(mut book) => {
            book.next_page();
            book.prev_page();
            assert_eq!(book.get_current_page(), 1);
        }
        None => {
            panic!("Book not created")
        }
    }
}
