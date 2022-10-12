use druid::{FontDescriptor, FontFamily, FontWeight};

/* UI */
pub const DISPLAY_WIDTH: f64 = 800.0;
pub const DISPLAY_HEIGTH: f64 = 600.0;
pub const APP_NAME: &str = "EBook Reader";
pub const TITLE_SIZE: f64 = 24.0;

/* Text Styles */
pub const TITLE: FontDescriptor = FontDescriptor::new(FontFamily::SANS_SERIF).with_size(TITLE_SIZE).with_weight(FontWeight::BLACK);