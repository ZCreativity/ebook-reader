mod controller;
mod helper;
mod model;
mod view;

use crate::helper::config::{APP_NAME, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::model::app_state::AppState;

use crate::view::home::ui::build_ui;
use controller::app_delegate::Delegate;
use druid::{AppLauncher, PlatformError, WindowDesc};

fn main() -> Result<(), PlatformError> {
    let app_state: AppState = AppState::new();

    let main_window = WindowDesc::new(build_ui)
        .window_size((DISPLAY_WIDTH, DISPLAY_HEIGHT))
        .resizable(false)
        .title(APP_NAME);
    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .launch(app_state)
}
