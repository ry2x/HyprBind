use eframe::egui;
use crate::models::SearchOptions;
use super::types::{Theme, ColumnVisibility};

pub fn render_options_window(
    ctx: &egui::Context,
    show_options_window: &mut bool,
    theme: &mut Theme,
    column_visibility: &mut ColumnVisibility,
    search_options: &mut SearchOptions,
) {
    egui::Window::new("Options")
        .open(show_options_window)
        .resizable(false)
        .show(ctx, |ui| {
            ui.heading("\u{f050e} Theme");
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.radio_value(theme, Theme::Dark, "\u{f0594} Dark");
                ui.radio_value(theme, Theme::Light, "\u{e30d} Light");
            });
            ui.add_space(10.0);
            
            ui.separator();
            ui.add_space(10.0);
            
            ui.heading("\u{f0db} Visible Columns");
            ui.add_space(5.0);
            ui.checkbox(&mut column_visibility.keybind, "\u{ea65} Keybind");
            ui.checkbox(&mut column_visibility.description, "\u{f29e} Description");
            ui.checkbox(&mut column_visibility.command, "\u{ebc4} Command");
            ui.add_space(10.0);
            
            ui.separator();
            ui.add_space(10.0);
            
            ui.heading("\u{e68f} Search Options");
            ui.add_space(5.0);
            ui.label("Search in:");
            ui.checkbox(&mut search_options.keybind, "\u{ea65} Keybind");
            ui.checkbox(&mut search_options.description, "\u{f29e} Description");
            ui.checkbox(&mut search_options.command, "\u{ebc4} Command");
        });
}
