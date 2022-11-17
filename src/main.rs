mod helper;
mod model;
mod view;
mod controller;

use crate::helper::config::{APP_NAME, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use druid::{AppLauncher, PlatformError, WindowDesc};
use crate::model::app_state::AppState;
use crate::view::home::ui::build_ui;

fn main() -> Result<(), PlatformError> {

    let app_state: AppState = AppState::new();

    let main_window = WindowDesc::new(|| build_ui())
        .window_size((DISPLAY_WIDTH, DISPLAY_HEIGHT))
        .title(APP_NAME);
    AppLauncher::with_window(main_window).launch(app_state)
}
