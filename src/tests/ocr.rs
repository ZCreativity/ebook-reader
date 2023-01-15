#[allow(unused)]
const OCR_TEST_FILE_PATH: &str = "./src/library/ocr/the-grand-hall.png";
#[allow(unused)]
const TEST_NOTRE_DAME_PATH: &str = "./src/library/hugo-hunchback-of-notre-dame.epub";

/** OCR Tests */

/**
 * Tests that the text of an image is extracted
 */
#[test]
fn ocr_from_file() {
    let text = tesseract::ocr(OCR_TEST_FILE_PATH, "eng");
    assert_eq!(text.is_ok(), true);
    assert_eq!(text.unwrap(), "The Grand Hall.\n");
}

/**
 * Tests that the text of an image is extracted and the page is found in a book
 */
#[test]
fn ocr_from_file_with_book() {
    let book =
        crate::helper::functions::epub_to_book(std::path::PathBuf::from(TEST_NOTRE_DAME_PATH));
    match book {
        Some(book) => {
            let text = tesseract::ocr(OCR_TEST_FILE_PATH, "eng");
            assert_eq!(text.is_ok(), true);
            let page = book.get_page_from_ocr_text(text.unwrap().trim().to_string());
            assert_eq!(page.is_some(), true);
            assert_eq!(page.unwrap(), 6);
        }
        None => {
            panic!("Book not created")
        }
    }
}

/**
 * Tests that given an epub page, returns the physical corrisponding range of pages
 */
#[test]
fn reverse_ocr() {
    let book =
        crate::helper::functions::epub_to_book(std::path::PathBuf::from(TEST_NOTRE_DAME_PATH));
    match book {
        Some(mut book) => {
            book.set_page(1);
            book.reverse_ocr();
            assert_eq!(book.get_physical_page_range().is_some(), true);
            assert_eq!(book.get_physical_page_range().unwrap(), (1, 2));
        }
        None => {
            panic!("Book not created")
        }
    }
}
