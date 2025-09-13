use egui_probe::EguiProbe;



#[derive(EguiProbe, Default)]
pub struct AppSettings {
    #[egui_probe(name = "Daily Notes Roots")]
    pub daily_notes_roots: Vec<String>
}
