use eframe::egui;
use egui::epaint::text::{FontInsert, InsertFontFamily};

/// Setup custom fonts including Nerd Font
pub fn setup_custom_fonts(ctx: &egui::Context) {
    
    ctx.add_font(FontInsert::new(
        nerd_font::NerdFont::FONT_FAMILY, 
        egui::FontData::from_static(nerd_font::NerdFont::FONT_BYTES), 
        vec![
            InsertFontFamily {
                family: egui::FontFamily::Proportional,
                priority: egui::epaint::text::FontPriority::Highest,
            },
            InsertFontFamily {
                family: egui::FontFamily::Monospace,
                priority: egui::epaint::text::FontPriority::Lowest,
            },
        ]
    ));
    
    eprintln!("✓ Loaded Nerd Font");
}
