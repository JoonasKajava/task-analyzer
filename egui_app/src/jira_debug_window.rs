use egui::{Ui};

use crate::App;


#[derive(Default)]
pub struct JiraDebugWindow {
    task_string: String,
    is_open: bool
}

impl JiraDebugWindow {
    pub fn window(&mut self, ctx: & egui::Context) {
        egui::Window::new("JIRA Debug")
            .collapsible(true)
            .open(&mut self.is_open)
            .show(ctx, |ui| {
                ui.text_edit_singleline(&mut self.task_string);

                let activity = App::get_task(&self.task_string);

                ui.label(match activity {
                    Some(_) => format!("Parsed Activity: {activity:?}"),
                    None => "Not valid".into(),
                });
            });
    }

    pub fn button(&mut self, ui: &mut Ui) {
        if ui
            .add_enabled(
                !self.is_open,
                egui::Button::new("Open JIRA"),
            )
            .clicked()
        {
            self.is_open = true;
        }
    }
}
