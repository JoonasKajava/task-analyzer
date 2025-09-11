use parser::{obsidian::obsidian::ActivityEntry, task_parser::TaskParser};

fn main() -> eframe::Result {
    env_logger::init();

    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Task Analyzer",
        native_options,
        Box::new(|cc| Ok(Box::new(TaskAnalyzerApp::new(cc)))),
    )
}

#[derive(Default)]
struct TaskAnalyzerApp {
    pub font_size: f32,
    pub test_task_string: String,
}

impl TaskAnalyzerApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn get_task(string: &String) -> Option<ActivityEntry> {
        TaskParser::parse_activity(string.as_bytes()).ok()?
    }
}

impl eframe::App for TaskAnalyzerApp {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        _visuals.window_fill().to_normalized_gamma_f32()
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn raw_input_hook(&mut self, _ctx: &egui::Context, _raw_input: &mut egui::RawInput) {}

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");

            ui.text_edit_singleline(&mut self.test_task_string);

            let activity = TaskAnalyzerApp::get_task(&self.test_task_string);

            ui.label(match activity {
                Some(_) => format!("Parsed Activity: {activity:?}"),
                None => "Not valid".into(),
            });
            
        });
    }
}
