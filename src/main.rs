use druid::{AppLauncher, WindowDesc};
use helper::config::{APP_NAME, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use model::app_state::AppState;
use view::navigator::navigator;

mod controller;
mod helper;
mod model;
mod tests;
mod view;

fn main() {
    let window = WindowDesc::new(navigator())
        .title(APP_NAME)
        .window_size((DISPLAY_WIDTH, DISPLAY_HEIGHT));

    let app_state = AppState::new();

    AppLauncher::with_window(window)
        .log_to_console()
        .launch(app_state)
        .unwrap();
}
