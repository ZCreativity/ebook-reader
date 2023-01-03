use crate::{
    controller::navigator::NavigatorController,
    model::{app_state::AppState, ui_view::UiView},
};
use druid::{Widget, WidgetExt};
use druid_widget_nursery::navigator::Navigator;

use super::library::library;

// use super::{contact_detail::contact_details, contact_edit::contact_edit, contacts::contacts};

// creates the navigator widget responsible for changing views
pub fn navigator() -> impl Widget<AppState> {
    Navigator::new(UiView::Library, library)
        // .with_view_builder(UiView::new("contact details".to_string()), contact_details)
        // .with_view_builder(UiView::new("contact edit".to_string()), contact_edit)
        .controller(NavigatorController)
}
