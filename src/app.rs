use eframe::egui;
use crate::models::KeyBindings;
use crate::parser::parse_hyprctl_binds;
use crate::icons::get_icon;

pub struct KeybindsApp {
    keybindings: KeyBindings,
    search_query: String,
    error_message: Option<String>,
}

impl KeybindsApp {
    pub fn new() -> Self {
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
                        ui.strong("  Description");
                        ui.strong("  Keybind");
                        ui.strong("  Command");
                        ui.end_row();

                        // Data rows
                        for entry in filtered {
                            // Description (display "-" if empty)
                            let description = if entry.description.is_empty() {
                                "  -".to_string()
                            } else {
                                format!("  {}", entry.description)
                            };
                            ui.label(description);

                            ui.horizontal(|ui| {
                                ui.add_space(4.0);
                                let frame = egui::Frame::new()
                                    .inner_margin(egui::Margin::symmetric(4, 1))
                                    .corner_radius(3.0)
                                    .stroke(egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.fg_stroke.color));

                                // Display modifier icons in frames
                                if !entry.modifiers.is_empty() {
                                    let modifiers: Vec<&str> = entry.modifiers.split('+').collect();
                                    for (i, modifier_str) in modifiers.iter().enumerate() {
                                        frame.show(ui, |ui| {
                                            ui.label(get_icon(modifier_str));
                                        });
                                        if i < modifiers.len() - 1 {
                                            ui.label("+");
                                        }
                                    }
                                    ui.label("+");
                                }

                                // Key with frame
                                frame.show(ui, |ui| {
                                    ui.label(get_icon(&entry.key));
                                });
                            });

                            // Command (truncate if too long)
                            let command_display = if entry.command.len() > 50 {
                                format!("  {}...", &entry.command[..47])
                            } else {
                                format!("  {}", &entry.command)
                            };
                            ui.label(command_display);

                            ui.end_row();
                        }
                    });
            });
        });
    }
}
