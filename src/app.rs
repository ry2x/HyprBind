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

#[derive(PartialEq, Eq, Clone, Copy)]
enum Theme {
    Dark,
    Light,
}

pub struct ColumnVisibility {
    pub keybind: bool,
    pub description: bool,
    pub command: bool,
}

impl Default for ColumnVisibility {
    fn default() -> Self {
        Self {
            keybind: true,
            description: true,
            command: false,
        }
    }
}

pub struct KeybindsApp {
    keybindings: KeyBindings,
    search_query: String,
    error_message: Option<String>,
    search_options: SearchOptions,
    sort_column: SortColumn,
    sort_state: SortState,
    show_options_window: bool,
    theme: Theme,
    column_visibility: ColumnVisibility,
}

impl KeybindsApp {
    pub fn new() -> Self {
        match parse_hyprctl_binds() {
            Ok(keybindings) => Self {
                keybindings,
                search_query: String::new(),
                error_message: None,
                search_options: SearchOptions::default(),
                sort_column: SortColumn::Keybind,
                sort_state: SortState::None,
                show_options_window: false,
                theme: Theme::Dark,
                column_visibility: ColumnVisibility::default(),
            },
            Err(e) => Self {
                keybindings: KeyBindings::new(),
                search_query: String::new(),
                error_message: Some(format!("Failed to load keybindings: {}", e)),
                search_options: SearchOptions::default(),
                sort_column: SortColumn::Keybind,
                sort_state: SortState::None,
                show_options_window: false,
                theme: Theme::Dark,
                column_visibility: ColumnVisibility::default(),
            },
        }
    }

    fn sort_button(&mut self, ui: &mut egui::Ui, label: &str, column: SortColumn) {
        let mut button_text = label.to_string();
        let sort_indicator = if self.sort_column == column {
            match self.sort_state {
                SortState::Ascending => " ▲",
                SortState::Descending => " ▼",
                SortState::None => "",
            }
        } else {
            ""
        };
        button_text.push_str(sort_indicator);
        
        // Navbar-style button with hover effect
        let is_active = self.sort_column == column && self.sort_state != SortState::None;
        let button = egui::Button::new(egui::RichText::new(button_text).strong().size(14.0))
            .fill(if is_active {
                ui.visuals().widgets.active.bg_fill
            } else {
                egui::Color32::TRANSPARENT
            })
            .stroke(egui::Stroke::NONE);
        
        let response = ui.add(button);
        
        if response.clicked() {
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
        let search_bar_focused = ctx.memory(|m| m.focused() == Some(egui::Id::new("search_bar")));
        
        if !search_bar_focused && ctx.input(|i| i.key_pressed(egui::Key::Slash)) {
            ctx.memory_mut(|m| m.request_focus(egui::Id::new("search_bar")));
            // Consume the slash event so it doesn't get typed
            ctx.input_mut(|i| {
                i.events.retain(|e| !matches!(e, egui::Event::Text(s) if s == "/"));
            });
        }

        // Apply theme
        match self.theme {
            Theme::Dark => ctx.set_visuals(egui::Visuals::dark()),
            Theme::Light => ctx.set_visuals(egui::Visuals::light()),
        }

        // Options window
        if self.show_options_window {
            egui::Window::new("Options")
                .open(&mut self.show_options_window)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.heading("Theme");
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.radio_value(&mut self.theme, Theme::Dark, "Dark");
                        ui.radio_value(&mut self.theme, Theme::Light, "Light");
                    });
                    ui.add_space(10.0);
                    
                    ui.separator();
                    ui.add_space(10.0);
                    
                    ui.heading("Visible Columns");
                    ui.add_space(5.0);
                    ui.checkbox(&mut self.column_visibility.keybind, "Keybind");
                    ui.checkbox(&mut self.column_visibility.description, "Description");
                    ui.checkbox(&mut self.column_visibility.command, "Command");
                    ui.add_space(10.0);
                    
                    ui.separator();
                    ui.add_space(10.0);
                    
                    ui.heading("Search Options");
                    ui.add_space(5.0);
                    ui.label("Search in:");
                    ui.checkbox(&mut self.search_options.description, "Description");
                    ui.checkbox(&mut self.search_options.keybind, "Keybind");
                    ui.checkbox(&mut self.search_options.command, "Command");
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Modern header with background
            let header_rect = egui::Rect::from_min_size(
                ui.min_rect().min,
                egui::vec2(ui.available_width(), 120.0)
            );
            let header_bg = if ui.visuals().dark_mode {
                egui::Color32::from_rgb(25, 25, 30)
            } else {
                egui::Color32::from_rgb(250, 250, 255)
            };
            ui.painter().rect_filled(header_rect, 0.0, header_bg);
            
            ui.add_space(15.0);
            
            // Title section
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                ui.label(egui::RichText::new("").size(28.0));
                ui.add_space(8.0);
                ui.label(egui::RichText::new("Hyprland Keybinds").size(24.0).strong());
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(20.0);
                    let options_button = egui::Button::new(egui::RichText::new("⚙").size(18.0))
                        .fill(egui::Color32::TRANSPARENT)
                        .stroke(egui::Stroke::NONE);
                    if ui.add(options_button).on_hover_text("Options").clicked() {
                        self.show_options_window = !self.show_options_window;
                    }
                });
            });
            
            ui.add_space(12.0);

            // Display error message if any
            if let Some(error) = &self.error_message {
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    ui.colored_label(egui::Color32::RED, format!("⚠ {}", error));
                });
                ui.add_space(8.0);
            }

            // Modern search bar
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                
                // Search icon and input
                ui.label(egui::RichText::new("").size(16.0).weak());
                ui.add_space(5.0);
                
                let search_bar = egui::TextEdit::singleline(&mut self.search_query)
                    .id(egui::Id::new("search_bar"))
                    .hint_text("Search keybinds... (press /)")
                    .desired_width(ui.available_width() - 140.0);
                ui.add(search_bar);
                
                ui.add_space(8.0);
                
                let clear_button = egui::Button::new(egui::RichText::new("Clear").size(13.0))
                    .fill(if ui.visuals().dark_mode {
                        egui::Color32::from_rgb(40, 40, 50)
                    } else {
                        egui::Color32::from_rgb(230, 230, 240)
                    });
                if ui.add(clear_button).clicked() {
                    self.search_query.clear();
                }
            });
            
            ui.add_space(12.0);

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

            // Stats bar
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                ui.label(egui::RichText::new(format!("📊 Total: {}", self.keybindings.entries.len())).weak().size(12.0));
                ui.add_space(10.0);
                ui.label(egui::RichText::new(format!("👁 Showing: {}", filtered.len())).weak().size(12.0));
            });
            ui.add_space(8.0);
            
            ui.separator();

            // Scrollable table
            use egui_extras::{TableBuilder, Column};
            
            let mut table = TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center));
            
            // Count visible columns
            let visible_count = [
                self.column_visibility.keybind,
                self.column_visibility.description,
                self.column_visibility.command,
            ].iter().filter(|&&v| v).count();
            
            // Add columns based on visibility with remainder for last column
            let mut col_index = 0;
            
            if self.column_visibility.keybind {
                col_index += 1;
                if col_index == visible_count {
                    table = table.column(Column::remainder().at_least(150.0).resizable(true).clip(true));
                } else {
                    table = table.column(Column::exact(200.0).resizable(true).clip(true));
                }
            }
            if self.column_visibility.description {
                col_index += 1;
                if col_index == visible_count {
                    table = table.column(Column::remainder().at_least(200.0).resizable(true).clip(true));
                } else {
                    table = table.column(Column::exact(300.0).resizable(true).clip(true));
                }
            }
            if self.column_visibility.command {
                col_index += 1;
                if col_index == visible_count {
                    table = table.column(Column::remainder().at_least(200.0).resizable(true).clip(true));
                } else {
                    table = table.column(Column::exact(300.0).resizable(true).clip(true));
                }
            }
            
            table
                .header(35.0, |mut header| {
                    if self.column_visibility.keybind {
                        header.col(|ui| {
                            // Navbar-style header background
                            let rect = ui.max_rect();
                            let bg_color = if ui.visuals().dark_mode {
                                egui::Color32::from_rgb(30, 30, 35)
                            } else {
                                egui::Color32::from_rgb(245, 245, 250)
                            };
                            ui.painter().rect_filled(rect, 0.0, bg_color);
                            
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center).with_main_justify(true), |ui| {
                                ui.add_space(8.0);
                                self.sort_button(ui, "Keybind", SortColumn::Keybind);
                                ui.add_space(8.0);
                            });
                        });
                    }
                    if self.column_visibility.description {
                        header.col(|ui| {
                            let rect = ui.max_rect();
                            let bg_color = if ui.visuals().dark_mode {
                                egui::Color32::from_rgb(30, 30, 35)
                            } else {
                                egui::Color32::from_rgb(245, 245, 250)
                            };
                            ui.painter().rect_filled(rect, 0.0, bg_color);
                            
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center).with_main_justify(true), |ui| {
                                ui.add_space(8.0);
                                self.sort_button(ui, "Description", SortColumn::Description);
                                ui.add_space(8.0);
                            });
                        });
                    }
                    if self.column_visibility.command {
                        header.col(|ui| {
                            let rect = ui.max_rect();
                            let bg_color = if ui.visuals().dark_mode {
                                egui::Color32::from_rgb(30, 30, 35)
                            } else {
                                egui::Color32::from_rgb(245, 245, 250)
                            };
                            ui.painter().rect_filled(rect, 0.0, bg_color);
                            
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center).with_main_justify(true), |ui| {
                                ui.add_space(8.0);
                                self.sort_button(ui, "Command", SortColumn::Command);
                                ui.add_space(8.0);
                            });
                        });
                    }
                })
                .body(|mut body| {
                    for entry in filtered {
                        body.row(32.0, |mut row| {
                            if self.column_visibility.keybind {
                                row.col(|ui| {
                                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                        ui.add_space(8.0);
                                        
                                        // Modern key badge style
                                        let key_frame = egui::Frame::new()
                                            .inner_margin(egui::Margin::symmetric(8, 4))
                                            .corner_radius(6.0)
                                            .fill(if ui.visuals().dark_mode {
                                                egui::Color32::from_rgb(45, 45, 55)
                                            } else {
                                                egui::Color32::from_rgb(235, 235, 240)
                                            })
                                            .stroke(egui::Stroke::new(1.5, if ui.visuals().dark_mode {
                                                egui::Color32::from_rgb(70, 70, 80)
                                            } else {
                                                egui::Color32::from_rgb(200, 200, 210)
                                            }));

                                        // Display modifier icons in modern badges
                                        if !entry.modifiers.is_empty() {
                                            let modifiers: Vec<&str> = entry.modifiers.split('+').collect();
                                            for (i, modifier_str) in modifiers.iter().enumerate() {
                                                key_frame.show(ui, |ui| {
                                                    ui.label(egui::RichText::new(get_icon(modifier_str)).size(13.0));
                                                });
                                                if i < modifiers.len() - 1 {
                                                    ui.label(egui::RichText::new("+").size(12.0).weak());
                                                }
                                            }
                                            ui.label(egui::RichText::new("+").size(12.0).weak());
                                        }

                                        // Key with modern badge
                                        key_frame.show(ui, |ui| {
                                            ui.label(egui::RichText::new(get_icon(&entry.key)).size(13.0));
                                        });
                                    });
                                });
                            }
                            
                            if self.column_visibility.description {
                                row.col(|ui| {
                                    ui.add_space(8.0);
                                    let description = if entry.description.is_empty() {
                                        egui::RichText::new("-").weak()
                                    } else {
                                        egui::RichText::new(&entry.description)
                                    };
                                    ui.label(description);
                                });
                            }

                            if self.column_visibility.command {
                                row.col(|ui| {
                                    ui.add_space(8.0);
                                    let command_display = if entry.command.len() > 50 {
                                        format!("{}...", &entry.command[..47])
                                    } else {
                                        entry.command.clone()
                                    };
                                    ui.label(egui::RichText::new(command_display).size(12.0).color(
                                        if ui.visuals().dark_mode {
                                            egui::Color32::from_rgb(180, 180, 190)
                                        } else {
                                            egui::Color32::from_rgb(80, 80, 90)
                                        }
                                    ));
                                });
                            }
                        });
                    }
                });
        });
    }
}
