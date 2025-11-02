use eframe::egui;

/// Setup custom fonts including Nerd Font icons
pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Load Nerd Font (using system font if available)
    if let Some(font_data) = load_nerd_font() {
        fonts.font_data.insert(
            "nerd_font".to_owned(),
            std::sync::Arc::new(egui::FontData::from_owned(font_data)),
        );

        // Add Nerd Font to the font families
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "nerd_font".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "nerd_font".to_owned());
    }

    ctx.set_fonts(fonts);
}

/// Try to load Nerd Font from system
fn load_nerd_font() -> Option<Vec<u8>> {
    // Try common Nerd Font paths
    let font_paths = [
        "/usr/share/fonts/TTF/JetBrainsMonoNerdFont-Regular.ttf",
        "/usr/share/fonts/TTF/FiraCodeNerdFont-Regular.ttf",
        "/usr/share/fonts/TTF/InputMonoNerdFont-Regular.ttf",
        "/usr/share/fonts/TTF/M+1CodeNerdFont-Regular.ttf",
        "/usr/share/fonts/TTF/M+CodeLat50NerdFont-Regular.ttf",
        "/usr/share/fonts/truetype/JetBrainsMonoNerdFont-Regular.ttf",
        "~/.local/share/fonts/JetBrainsMonoNerdFont-Regular.ttf",
    ];

    for path in &font_paths {
        let expanded_path = if path.starts_with('~') {
            if let Some(home) = std::env::var_os("HOME") {
                std::path::PathBuf::from(home).join(&path[2..])
            } else {
                continue;
            }
        } else {
            std::path::PathBuf::from(path)
        };

        if let Ok(data) = std::fs::read(&expanded_path) {
            eprintln!("✓ Loaded Nerd Font from: {:?}", expanded_path);
            return Some(data);
        }
    }

    eprintln!("✗ No Nerd Font found in any of the searched paths");
    None
}
