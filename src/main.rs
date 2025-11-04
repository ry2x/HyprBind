mod app;
mod font;
mod icons;
mod models;
mod parser;
mod ui;
mod config;

use eframe::egui;
use app::KeybindsApp;
use font::setup_custom_fonts;

fn main() -> Result<(), eframe::Error> {
    let icon_data = load_icon();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_title("HyprBind")
            .with_icon(icon_data),
        ..Default::default()
    };

    eframe::run_native(
        "HyprBind",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(KeybindsApp::new()))
        }),
    )
}

fn load_icon() -> egui::IconData {
    let icon_bytes = include_bytes!("../assets/logo_hyprbind.png");
    let image = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon")
        .into_rgba8();
    let (width, height) = image.dimensions();
    
    egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}
