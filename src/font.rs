use eframe::egui;
use egui::epaint::text::{FontInsert, InsertFontFamily};

/// Setup custom fonts including Nerd Font and Japanese font
pub fn setup_custom_fonts(ctx: &egui::Context) {
    ctx.add_font(FontInsert::new(
        nerd_font::NerdFont::FONT_FAMILY,
        egui::FontData::from_static(nerd_font::NerdFont::FONT_BYTES),
        vec![InsertFontFamily {
            family: egui::FontFamily::Proportional,
            priority: egui::epaint::text::FontPriority::Highest,
        }],
    ));

    eprintln!("✓ Loaded Nerd Font");

    // Load Japanese font (Medium weight ≈ 500, close to SemiBold)
    let font_data = egui::FontData::from_static(include_bytes!("../assets/NotoSansCJK-Medium.ttc"));

    ctx.add_font(FontInsert::new(
        "Noto Sans CJK TC Medium",
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

    eprintln!("✓ Loaded Noto Sans CJK TC (Medium)");
}
