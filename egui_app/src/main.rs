mod jira_debug_window;
mod routes;
pub mod views;
pub mod side_menu;
mod app_settings;

use data_processing::activity_entry::ActivityEntry;
use data_processing::parsing::task_parser::TaskParser;
use egui_inbox::{UiInbox, UiInboxSender};
use egui_router::EguiRouter;
use log::error;

use crate::{app_settings::AppSettings, jira_debug_window::JiraDebugWindow, routes::router, side_menu::show_side_menu};

fn main() -> eframe::Result {
    env_logger::init();

    let native_options = eframe::NativeOptions::default();

    let (tx, inbox) = UiInbox::channel();

    let mut state = State {
        jira_debug: JiraDebugWindow::default(),
        settings: AppSettings::default(),
        tx,
    };

    let router = router(&mut state);

    eframe::run_native(
        "Task Analyzer",
        native_options,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();
            egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);

            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(App {
                router,
                inbox,
                state,
            }))
        }),
    )
}

pub struct State {
    pub jira_debug: JiraDebugWindow,
    pub settings: AppSettings,
    pub tx: UiInboxSender<Message>,
}

impl State {
    pub fn send_message(&self, message: Message) {
        self.tx.send(message).unwrap_or_else(|err| {
            error!("navigation error: {err:?}");
        });
    }
}

#[derive(Debug)]
pub enum Message {
    Navigate(String),
}

pub struct App {
    router: EguiRouter<State>,
    inbox: UiInbox<Message>,
    state: State,
}

impl App {
    fn get_task(string: &String) -> Option<ActivityEntry> {
        TaskParser::parse_activity(string.as_bytes()).ok()?
    }

}

impl eframe::App for App {
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
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("Debug", |ui| {
                    self.state.jira_debug.button(ui);
                });
            })
        });

        show_side_menu(self, ctx);

        self.state.jira_debug.window(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(Message::Navigate(path)) = self.inbox.read(ui).last() {
                self.router
                    .navigate(&mut self.state, path)
                    .unwrap_or_else(|err| {
                        error!("navigation error: {err:?}");
                    });
            }

            self.router.ui(ui, &mut self.state);
        });
    }
}
