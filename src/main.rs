mod ui;
mod helper;

use crate::ui::home::*;

use druid::{AppLauncher, PlatformError, WindowDesc};
use crate::helper::config::{APP_NAME, DISPLAY_HEIGTH, DISPLAY_WIDTH};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(home_page_ui::ui_builder)
        .window_size((DISPLAY_WIDTH, DISPLAY_HEIGTH))
        .title(APP_NAME);
    AppLauncher::with_window(main_window).launch(())
}