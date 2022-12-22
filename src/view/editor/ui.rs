use druid::widget::{TextBox, Widget, WidgetExt};
use druid::{
    Env,
    UpdateCtx,
    widget::Controller
};

pub fn book_editor() -> impl Widget<String>{
    let textbox = build_root_widget();
    textbox
}

struct UpdateCallback();

impl Controller<String, TextBox<String>> for UpdateCallback {
    fn update(&mut self, 
        child: &mut TextBox<String>, 
        ctx: &mut UpdateCtx<'_, '_>, 
        old_data: &String, 
        data: &String, 
        env: &Env
    ) {
        if old_data != data {
            // the data has changed, you can call your function here
            println!("{}", data);
        }
        // also inform the child that the data has changed
        child.update(ctx, old_data, data, env)
    }
}

fn build_root_widget() -> impl Widget<String> {
    TextBox::new().controller(UpdateCallback())
}