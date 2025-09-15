use egui::Ui;
use egui_probe::Probe;
use egui_router::Route;

use crate::State;


pub fn settings_view() -> impl Route<State> {
    |ui: &mut Ui, state: &mut State| {
        ui.heading("Settings");
        Probe::new(&mut state.settings).show(ui);
    }
}
