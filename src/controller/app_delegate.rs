use druid::{AppDelegate, Handled, Selector};

use crate::model::{app_state::AppState, book::Book};

// Open book selector
pub const OPEN_BOOK: Selector<Book> = Selector::new("open-book");
pub const CLOSE_BOOK: Selector = Selector::new("close-book");

pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    // Custom command handler with custom data
    fn command(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        cmd: &druid::Command,
        data: &mut AppState,
        _env: &druid::Env,
    ) -> Handled {
        if let Some(book) = cmd.get(OPEN_BOOK) {
            data.open_book(book.clone());
            return Handled::Yes;
        }
        if cmd.is(CLOSE_BOOK) {
            data.close_book();
            return Handled::Yes;
        }
        Handled::No
    }
}
