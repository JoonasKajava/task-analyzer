use egui_router::{EguiRouter, TransitionConfig};

use crate::{views::{home_view::{home_view}, settings_view::settings_view}, State};




pub fn router(state: &mut State) -> EguiRouter<State> {
    EguiRouter::builder()
        .transition(TransitionConfig::fade_up().with_easing(egui_animation::easing::quad_out))
        .default_duration(0.2)
        .default_path("/")
        .route("/", home_view)
        .route("/settings", settings_view)
        .build(state)
}
