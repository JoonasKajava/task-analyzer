use egui::RichText;

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
    pub font_size: f32
}

impl TaskAnalyzerApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
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
            ui.add(egui::Slider::new(&mut self.font_size, 1.0..=100.0));
            ui.label(RichText::new("This is a template for a Rust GUI application using eframe and egui.").size(self.font_size));
        });
    }
}
