use std::time::Duration;

use egui::Ui;
use egui_probe::Probe;
use egui_router::Route;
use log::{error, info};

use crate::State;

pub fn settings_view() -> impl Route<State> {
    |ui: &mut Ui, state: &mut State| {
        ui.heading("Settings");
        Probe::new(&mut state.settings).show(ui);

        ui.add_space(30.0);

        ui.horizontal(|ui| {
            if ui.button("Save Settings").clicked() {
                match state.settings.save() {
                    Ok(_) => {
                        state.toasts
                            .success("Settings saved")
                            .duration(Some(Duration::from_secs(2)));

                        info!("Settings saved successfully")
                    }
                    Err(err) => {
                        state.toasts
                            .error(err.to_string())
                            .duration(Some(Duration::from_secs(5)));

                        error!("Failed to save settings: {err}")
                    }
                }
            }
        });
    }
}
