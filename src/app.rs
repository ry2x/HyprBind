use eframe::egui;
use crate::models::{KeyBindings, SearchOptions};
use crate::parser::parse_hyprctl_binds;
use crate::icons::get_icon;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Clone, Copy)]
enum SortColumn {
    Keybind,
    Description,
    Command,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum SortState {
    Ascending,
    Descending,
    None,
}

pub struct KeybindsApp {
    keybindings: KeyBindings,
    search_query: String,
    error_message: Option<String>,
    search_options: SearchOptions,
    sort_column: SortColumn,
    sort_state: SortState,
}

impl KeybindsApp {
    pub fn new() -> Self {
        match parse_hyprctl_binds() {
            Ok(keybindings) => Self {
                keybindings,
                search_query: String::new(),
                error_message: None,
                search_options: SearchOptions::default(),
                sort_column: SortColumn::Keybind, // Default column, doesn't matter much
                sort_state: SortState::None,
            },
            Err(e) => Self {
                keybindings: KeyBindings::new(),
                search_query: String::new(),
                error_message: Some(format!("Failed to load keybindings: {}", e)),
                search_options: SearchOptions::default(),
                sort_column: SortColumn::Keybind,
                sort_state: SortState::None,
            },
        }
    }

    fn sort_button(&mut self, ui: &mut egui::Ui, label: &str, column: SortColumn) {
        let mut button_text = label.to_string();
        if self.sort_column == column {
            match self.sort_state {
                SortState::Ascending => button_text.push('▲'),
                SortState::Descending => button_text.push('▼'),
                SortState::None => (),
            }
        }
        if ui.button(button_text).clicked() {
            if self.sort_column == column {
                self.sort_state = match self.sort_state {
                    SortState::Ascending => SortState::Descending,
                    SortState::Descending => SortState::None,
                    SortState::None => SortState::Ascending,
                };
            } else {
                self.sort_column = column;
                self.sort_state = SortState::Ascending;
            }
        }
    }
}

impl eframe::App for KeybindsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Keyboard shortcut for focusing search bar
        if ctx.input(|i| i.key_pressed(egui::Key::Slash)) {
            ctx.memory_mut(|m| m.request_focus(egui::Id::new("search_bar")));
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("  Hyprland Keybinds Viewer");
            ui.add_space(10.0);

            // Display error message if any
            if let Some(error) = &self.error_message {
                ui.colored_label(egui::Color32::RED, format!("  {}", error));
                ui.add_space(10.0);
            }

            // Search bar and options
            ui.horizontal(|ui| {
                ui.label("  Search (/):");
                let search_bar = egui::TextEdit::singleline(&mut self.search_query).id(egui::Id::new("search_bar"));
                ui.add(search_bar);
                if ui.button("Clear").clicked() {
                    self.search_query.clear();
                }
            });

            ui.horizontal(|ui| {
                ui.add_space(4.0);
                ui.label("Search in:");
                ui.checkbox(&mut self.search_options.description, "Description");
                ui.checkbox(&mut self.search_options.keybind, "Keybind");
                ui.checkbox(&mut self.search_options.command, "Command");
            });
            ui.add_space(10.0);

            // Get filtered and sorted keybindings
            let mut filtered: Vec<_> = self.keybindings.filter(&self.search_query, &self.search_options).into_iter().cloned().collect();
            
            if self.sort_state != SortState::None {
                match self.sort_column {
                    SortColumn::Description => filtered.sort_by(|a, b| a.description.cmp(&b.description)),
                    SortColumn::Keybind => filtered.sort_by(|a, b| {
                        let mod_cmp = a.modifiers.cmp(&b.modifiers);
                        if mod_cmp == Ordering::Equal {
                            a.key.cmp(&b.key)
                        } else {
                            mod_cmp
                        }
                    }),
                    SortColumn::Command => filtered.sort_by(|a, b| a.command.cmp(&b.command)),
                }
                if self.sort_state == SortState::Descending {
                    filtered.reverse();
                }
            }

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
                        self.sort_button(ui, "  Keybind", SortColumn::Keybind);
                        self.sort_button(ui, "  Description", SortColumn::Description);
                        self.sort_button(ui, "  Command", SortColumn::Command);
                        ui.end_row();

                        // Data rows
                        for entry in filtered {
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
                            
                            // Description (display "-" if empty)
                            let description = if entry.description.is_empty() {
                                "  -".to_string()
                            } else {
                                format!("  {}", entry.description)
                            };
                            ui.label(description);

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
