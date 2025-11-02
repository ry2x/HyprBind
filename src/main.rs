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
        Box::new(|_cc| Ok(Box::new(KeybindsApp::new()))),
    )
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
            ui.heading("Hyprland Keybinds Viewer");
            ui.add_space(10.0);

            // Display error message if any
            if let Some(error) = &self.error_message {
                ui.colored_label(egui::Color32::RED, error);
                ui.add_space(10.0);
            }

            // Search bar
            ui.horizontal(|ui| {
                ui.label("Search:");
                ui.text_edit_singleline(&mut self.search_query);
                if ui.button("Clear").clicked() {
                    self.search_query.clear();
                }
            });
            ui.add_space(10.0);

            // Get filtered keybindings
            let filtered = self.keybindings.filter(&self.search_query);

            // Header information
            ui.label(format!("Total keybindings: {} (showing: {})", 
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
                        ui.strong("Modifiers");
                        ui.strong("Key");
                        ui.strong("Command");
                        ui.strong("Description");
                        ui.end_row();

                        // Data rows
                        for entry in filtered {
                            // Display "-" if modifiers is empty
                            let modifiers = if entry.modifiers.is_empty() {
                                "-"
                            } else {
                                &entry.modifiers
                            };
                            ui.label(modifiers);

                            // Key
                            ui.label(&entry.key);

                            // Command (truncate if too long)
                            if entry.command.len() > 50 {
                                ui.label(format!("{}...", &entry.command[..47]));
                            } else {
                                ui.label(&entry.command);
                            }

                            // Description (display "-" if empty)
                            let description = if entry.description.is_empty() {
                                "-"
                            } else {
                                &entry.description
                            };
                            ui.label(description);

                            ui.end_row();
                        }
                    });
            });
        });
    }
}
