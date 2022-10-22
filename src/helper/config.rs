use druid::{FontDescriptor, FontFamily, FontWeight};

/* UI */
pub const DISPLAY_WIDTH: f64 = 800.0;
pub const DISPLAY_HEIGHT: f64 = 600.0;
pub const APP_NAME: &str = "EBook Reader";
pub const TITLE_SIZE: f64 = 24.0;
pub const PADDING_SM: f64 = 8.0;
pub const PADDING_MD: f64 = 16.0;
pub const PADDING_LG: f64 = 32.0;
pub const PADDING_XL: f64 = 64.0;

/* Text Styles */
pub const TITLE: FontDescriptor = FontDescriptor::new(FontFamily::SERIF)
    .with_size(TITLE_SIZE)
    .with_weight(FontWeight::BOLD);

/* Data */
pub const LIBRARY_PATH: &str = "./src/library";


/* Icons */
