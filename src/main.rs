mod helper;
mod model;
mod ui;
mod controller;

use crate::ui::home::*;

use crate::helper::config::{APP_NAME, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use druid::{AppLauncher, PlatformError, WindowDesc};
use crate::model::app_state::AppState;

fn main() -> Result<(), PlatformError> {

    let app_state: AppState = AppState::new();

    let main_window = WindowDesc::new(home_page_ui::ui_builder)
        .window_size((DISPLAY_WIDTH, DISPLAY_HEIGHT))
        .title(APP_NAME);
    AppLauncher::with_window(main_window).launch(app_state)
}
