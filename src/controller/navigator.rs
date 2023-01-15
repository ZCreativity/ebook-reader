use super::view::POP_VIEW;
use crate::model::{app_state::AppState, ui_view::UiView};
use druid::{widget::Controller, Env, Event, Widget};
use druid_widget_nursery::navigator::{Navigator, ViewController};

/**
 * NavigatorController
 * Handles events for the Navigator widget.
 */
pub struct NavigatorController;

impl Controller<AppState, Navigator<AppState, UiView>> for NavigatorController {
    fn event(
        &mut self,
        child: &mut Navigator<AppState, UiView>,
        ctx: &mut druid::EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        match event {
            Event::Command(selector) if selector.is(POP_VIEW) => {
                ctx.request_update();
                data.pop_view();
            }
            _ => (),
        }
        child.event(ctx, event, data, env)
    }
}
