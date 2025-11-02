mod app;
mod font;
mod icons;
mod models;
mod parser;

use eframe::egui;
use app::KeybindsApp;
use font::setup_custom_fonts;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_title("Hyprland Keybinds Viewer"),
        ..Default::default()
    };

    eframe::run_native(
        "Hyprland Keybinds",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(KeybindsApp::new()))
        }),
    )
}
