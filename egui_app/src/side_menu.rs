use egui::{containers, Margin, RichText, Sense, Ui};
use egui_phosphor::regular::{GEAR, HOUSE};

use crate::{App, Message};

fn side_panel_button(app: &mut App, ui: &mut Ui, path: &str, icon: &str) {
    let active = app.router.active_route() == Some(path);
    if ui
        .add(
            egui::Button::new(RichText::new(icon.to_string()).size(32.0))
                .selected(active)
                .sense(match active {
                    true => Sense::empty(),
                    false => Sense::click(),
                }),
        )
        .clicked()
    {
        app.state.send_message(Message::Navigate(path.into()));
    }
}

pub fn show_side_menu(app: &mut App, ctx: &egui::Context) {
    egui::SidePanel::left("side_panel")
        .resizable(false)
        .min_width(0.0)
        .frame(containers::Frame {
            inner_margin: Margin::same(10),
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing.y = 10.0;

            side_panel_button(app, ui, "/", HOUSE);
            side_panel_button(app, ui, "/settings", GEAR);

            ui.shrink_width_to_current();
        });
}
