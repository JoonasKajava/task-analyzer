use egui::Ui;
use egui_router::Route;

use crate::State;


pub fn home_view() -> impl Route<State> {
    |ui: &mut Ui, state: &mut State| {
        ui.label("Home View");
    }
}
