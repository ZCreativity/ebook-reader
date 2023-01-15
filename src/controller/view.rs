use crate::model::{app_state::AppState, ui_view::UiView};
use druid::Selector;
use druid_widget_nursery::navigator::{View, ViewController};
use std::sync::Arc;

pub const BOOK_READ: Selector<usize> = Selector::new("book-read");
pub const BOOK_EDIT: Selector<usize> = Selector::new("book-edit");
pub const POP_VIEW: Selector<()> = Selector::new("navigator.pop-view");

// implements the view trait for your view type
impl View for UiView {}

/**
 * Here you define Viewcontroller for your AppState. The navigator widget will
 * only accept AppStates that implement this trait. The methods here are used
 * handle modifying your navigation state without manually doing that with your
 * own methods. Look at the docs to see what each method is useful for.
 */
impl ViewController<UiView> for AppState {
    /**
     * This method is used to add a new view to the navigation stack
     */
    fn add_view(&mut self, view: UiView) {
        let views: &mut Vec<UiView> = Arc::make_mut(&mut self.nav_state);
        views.push(view);
        let views = Arc::new(views.clone());
        self.nav_state = views;
    }

    /**
     * This method is used to remove the last view from the navigation stack
     */
    fn pop_view(&mut self) {
        let views = Arc::make_mut(&mut self.nav_state);
        views.pop();
        let views = Arc::new(views.clone());
        self.nav_state = views;
    }

    fn current_view(&self) -> &UiView {
        self.nav_state.last().expect("No views in stack")
    }

    fn len(&self) -> usize {
        self.nav_state.len()
    }

    fn is_empty(&self) -> bool {
        self.nav_state.is_empty()
    }
}
