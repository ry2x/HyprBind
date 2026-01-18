use eframe::egui;
use egui::epaint::text::{FontInsert, InsertFontFamily};

/// Setup custom fonts including Nerd Font and Japanese font
pub fn setup_custom_fonts(ctx: &egui::Context) {
    // Load Japanese font
    let font_data: egui::FontData =
        egui::FontData::from_static(include_bytes!("../assets/Firple-Bold.ttf"));

    ctx.add_font(FontInsert::new(
        "Firple Bold",
        font_data,
        vec![
            InsertFontFamily {
                family: egui::FontFamily::Proportional,
                priority: egui::epaint::text::FontPriority::Highest,
            },
            InsertFontFamily {
                family: egui::FontFamily::Monospace,
                priority: egui::epaint::text::FontPriority::Highest,
            },
        ],
    ));

    eprintln!("✓ Loaded Firple Bold");
}
