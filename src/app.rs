use eframe::egui;
use crate::models::{KeyBindings, SearchOptions};
use crate::parser::parse_hyprctl_binds;
use crate::ui::{SortColumn, SortState, Theme, ColumnVisibility};
use std::cmp::Ordering;

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
    logo_texture: Option<egui::TextureHandle>,
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
                logo_texture: None,
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
                logo_texture: None,
            },
        }
    }

    fn handle_sort_click(&mut self, column: SortColumn) {
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
    
    fn get_filtered_and_sorted_entries(&self) -> Vec<crate::models::KeyBindEntry> {
        let mut filtered: Vec<_> = self.keybindings
            .filter(&self.search_query, &self.search_options)
            .into_iter()
            .cloned()
            .collect();
        
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
        
        filtered
    }
}

impl eframe::App for KeybindsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Load logo texture if not already loaded
        if self.logo_texture.is_none() {
            let logo_bytes = include_bytes!("../assets/logo_hyprbind.png");
            if let Ok(image) = image::load_from_memory(logo_bytes) {
                let size = [image.width() as usize, image.height() as usize];
                let image_buffer = image.to_rgba8();
                let pixels = image_buffer.as_flat_samples();
                let color_image = egui::ColorImage::from_rgba_unmultiplied(
                    size,
                    pixels.as_slice(),
                );
                self.logo_texture = Some(ctx.load_texture(
                    "logo",
                    color_image,
                    egui::TextureOptions::LINEAR,
                ));
            }
        }

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
            crate::ui::options::render_options_window(
                ctx,
                &mut self.show_options_window,
                &mut self.theme,
                &mut self.column_visibility,
                &mut self.search_options,
            );
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Render header
            crate::ui::header::render_header(ui, &mut self.show_options_window, &self.error_message, self.logo_texture.as_ref());
            
            // Render search bar
            crate::ui::header::render_search_bar(ui, &mut self.search_query);

            // Get filtered and sorted keybindings
            let filtered = self.get_filtered_and_sorted_entries();
            
            // Render stats bar
            crate::ui::header::render_stats_bar(ui, self.keybindings.entries.len(), filtered.len());

            // Render table
            if let Some(clicked_column) = crate::ui::table::render_table(
                ui,
                &filtered,
                &self.column_visibility,
                self.sort_column,
                self.sort_state,
            ) {
                self.handle_sort_click(clicked_column);
            }
        });
    }
}
