use eframe::egui;

pub fn render_zen_info_modal(
    ctx: &egui::Context,
    show_zen_info_modal: &mut bool,
    show_options_window: &mut bool,
) {
    // Close modal with Enter key
    if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
        *show_zen_info_modal = false;
        *show_options_window = false;
        return;
    }

    egui::Window::new("ZEN Mode")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.label(egui::RichText::new("\u{f06e} ZEN Mode Activated").size(18.0).strong());
                ui.add_space(15.0);
                ui.label("All distractions are hidden.");
                ui.label("Focus on your keybindings.");
                ui.add_space(10.0);
                ui.label(egui::RichText::new("Press Z key to exit ZEN mode").italics());
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("OK").size(14.0)).clicked() {
                    *show_zen_info_modal = false;
                    *show_options_window = false;
                }
                ui.add_space(10.0);
            });
        });
}

pub fn handle_zen_keyboard_shortcuts(
    ctx: &egui::Context,
    zen_mode: &mut bool,
) {
    // Z key to exit ZEN mode
    if *zen_mode && ctx.input(|i| i.key_pressed(egui::Key::Z)) {
        *zen_mode = false;
    }
}
