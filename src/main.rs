mod models;
mod parser;

use eframe::egui;
use models::KeyBindings;
use parser::parse_hyprctl_binds;

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

/// Setup custom fonts including Nerd Font icons
fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Load Nerd Font (using system font if available)
    // You can also embed a font file using include_bytes!
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

struct KeybindsApp {
    keybindings: KeyBindings,
    search_query: String,
    error_message: Option<String>,
}

impl KeybindsApp {
    fn new() -> Self {
        match parse_hyprctl_binds() {
            Ok(keybindings) => Self {
                keybindings,
                search_query: String::new(),
                error_message: None,
            },
            Err(e) => Self {
                keybindings: KeyBindings::new(),
                search_query: String::new(),
                error_message: Some(format!("Failed to load keybindings: {}", e)),
            },
        }
    }
}

impl eframe::App for KeybindsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("  Hyprland Keybinds Viewer");
            ui.add_space(10.0);

            // Display error message if any
            if let Some(error) = &self.error_message {
                ui.colored_label(egui::Color32::RED, format!("  {}", error));
                ui.add_space(10.0);
            }

            // Search bar
            ui.horizontal(|ui| {
                ui.label("  Search:");
                ui.text_edit_singleline(&mut self.search_query);
                if ui.button("  Clear").clicked() {
                    self.search_query.clear();
                }
            });
            ui.add_space(10.0);

            // Get filtered keybindings
            let filtered = self.keybindings.filter(&self.search_query);

            // Header information
            ui.label(format!("  Total: {} (showing: {})", 
                self.keybindings.entries.len(), 
                filtered.len()
            ));
            ui.add_space(5.0);

            // Scrollable table
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("keybinds_grid")
                    .striped(true)
                    .spacing([10.0, 5.0])
                    .show(ui, |ui| {
                        // Header row
                        ui.strong("  Modifiers");
                        ui.strong("  Key");
                        ui.strong("  Command");
                        ui.strong("  Description");
                        ui.end_row();

                        // Data rows
                        for entry in filtered {
                            // Display "-" if modifiers is empty
                            let modifiers = if entry.modifiers.is_empty() {
                                "  -"
                            } else {
                                &format!("  {}", entry.modifiers)
                            };
                            ui.label(modifiers);

                            // Key with icon
                            let key_display = format!("  {}", get_key_icon(&entry.key));
                            ui.label(key_display);

                            // Command (truncate if too long)
                            let command_display = if entry.command.len() > 50 {
                                format!("  {}...", &entry.command[..47])
                            } else {
                                format!("  {}", &entry.command)
                            };
                            ui.label(command_display);

                            // Description (display "-" if empty)
                            let description = if entry.description.is_empty() {
                                "  -"
                            } else {
                                &format!("  {}", entry.description)
                            };
                            ui.label(description);

                            ui.end_row();
                        }
                    });
            });
        });
    }
}

/// Get Nerd Font icon for common keys
fn get_key_icon(key: &str) -> String {
    match key {
        "Return" => format!("\u{f2f6} {}", key),        // nf-md-keyboard_return
        "Escape" => format!("\u{f12b} {}", key),        // nf-md-keyboard_esc
        "Tab" => format!("\u{f0331} {}", key),          // nf-md-keyboard_tab
        "space" => format!("\u{f0332} {}", key),        // nf-md-keyboard_space
        s if s.starts_with("XF86Audio") => format!("\u{f028} {}", s),  // nf-fa-volume_up
        s if s.starts_with("XF86MonBrightness") => format!("\u{f185} {}", s), // nf-fa-sun_o
        s if s.starts_with("mouse_down") => format!("\u{f1550} {}", s),
        s if s.starts_with("mouse_up") => format!("\u{f1551} {}", s),
        s if s.starts_with("mouse:272") => format!("\u{f0a54} {}", s),
        s if s.starts_with("mouse:273") => format!("\u{f02c7} {}", s),
        _ => key.to_string(),
    }
}
