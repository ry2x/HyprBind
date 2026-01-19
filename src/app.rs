use crate::models::{KeyBindings, SearchOptions};
use crate::parser::parse_hyprctl_binds;
use crate::ui::{ColumnVisibility, SortColumn, SortState, Theme};
use eframe::egui;
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
    zen_mode: bool,
    show_zen_info_modal: bool,
    selected_row: Option<usize>,
    export_request: bool,
    export_modal_path: Option<String>,
    last_css_mtime: Option<std::time::SystemTime>,
}

impl KeybindsApp {
    pub fn new() -> Self {
        let (keybindings, error_message) = match parse_hyprctl_binds() {
            Ok(keybindings) => (keybindings, None),
            Err(e) => (
                KeyBindings::new(),
                Some(format!("Failed to load keybindings: {e}")),
            ),
        };
        let mut app = Self {
            keybindings,
            search_query: String::new(),
            error_message,
            search_options: SearchOptions::default(),
            sort_column: SortColumn::Keybind,
            sort_state: SortState::None,
            show_options_window: false,
            theme: Theme::Dark,
            column_visibility: ColumnVisibility::default(),
            logo_texture: None,
            zen_mode: false,
            show_zen_info_modal: false,
            selected_row: None,
            export_request: false,
            export_modal_path: None,
            last_css_mtime: None,
        };
        if let Some(cfg) = crate::config::load() {
            app.theme = cfg.theme;
            app.column_visibility = cfg.column_visibility;
            app.search_options = cfg.search_options;
            app.zen_mode = cfg.zen_mode;
        }
        app
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
        let mut filtered: Vec<_> = self
            .keybindings
            .filter(&self.search_query, &self.search_options)
            .into_iter()
            .cloned()
            .collect();

        if self.sort_state != SortState::None {
            match self.sort_column {
                SortColumn::Description => {
                    filtered.sort_by(|a, b| a.description.cmp(&b.description));
                }
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
                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                self.logo_texture =
                    Some(ctx.load_texture("logo", color_image, egui::TextureOptions::LINEAR));
            }
        }

        // ZEN mode keyboard shortcuts with autosave
        let prev_zen = self.zen_mode;
        crate::ui::zen::handle_zen_keyboard_shortcuts(ctx, &mut self.zen_mode);
        if self.zen_mode != prev_zen {
            let cfg = crate::config::UserConfig {
                theme: self.theme,
                column_visibility: self.column_visibility.clone(),
                search_options: self.search_options.clone(),
                zen_mode: self.zen_mode,
            };
            let _ = crate::config::save(&cfg);
        }

        // Keyboard shortcuts
        let search_bar_focused = ctx.memory(|m| m.focused() == Some(egui::Id::new("search_bar")));

        // Slash key to focus search bar (only when not in ZEN mode)
        if !self.zen_mode && !search_bar_focused && ctx.input(|i| i.key_pressed(egui::Key::Slash)) {
            ctx.memory_mut(|m| m.request_focus(egui::Id::new("search_bar")));
            // Consume the slash event so it doesn't get typed
            ctx.input_mut(|i| {
                i.events
                    .retain(|e| !matches!(e, egui::Event::Text(s) if s == "/"));
            });
        }

        // Apply theme or auto-reload CSS when present
        if crate::css::has_custom_theme() {
            let path = crate::css::default_css_path();
            if let Ok(meta) = std::fs::metadata(&path)
                && let Ok(modified) = meta.modified()
            {
                let changed = self.last_css_mtime.is_none_or(|prev| modified > prev);
                if changed {
                    let _ = crate::css::apply_from_path(ctx, &path.to_string_lossy());
                    self.last_css_mtime = Some(modified);
                }
            }
        } else {
            match self.theme {
                Theme::Dark => ctx.set_visuals(egui::Visuals::dark()),
                Theme::Light => ctx.set_visuals(egui::Visuals::light()),
            }
        }

        // ZEN mode info modal
        if self.show_zen_info_modal {
            crate::ui::zen::render_zen_info_modal(
                ctx,
                &mut self.show_zen_info_modal,
                &mut self.show_options_window,
            );
            if ctx.input(|i| i.key_pressed(egui::Key::Z)) {
                self.show_zen_info_modal = false;
            }
        }

        // Close options with ESC
        if self.show_options_window && ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.show_options_window = false;
        }

        // Options window as separate OS window (egui viewport)
        let options_viewport_id = egui::ViewportId::from_hash_of("options");
        if self.show_options_window {
            ctx.show_viewport_immediate(
                options_viewport_id,
                egui::ViewportBuilder::default()
                    .with_title("HyprBind – Options")
                    .with_resizable(true)
                    .with_min_inner_size([400.0, 420.0])
                    .with_inner_size([520.0, 560.0]),
                |vctx, _class| {
                    if vctx.input(|i| i.viewport().close_requested()) {
                        self.show_options_window = false;
                    }
                    egui::CentralPanel::default().show(vctx, |ui| {
                        let prev_zen = self.zen_mode;
                        crate::ui::options::render_options_contents(
                            vctx,
                            ui,
                            &mut self.theme,
                            &mut self.column_visibility,
                            &mut self.search_options,
                            &mut self.zen_mode,
                            &mut self.show_zen_info_modal,
                            &mut self.export_request,
                        );
                        if !prev_zen && self.zen_mode {
                            self.show_options_window = false;
                        }
                    });
                },
            );
        } else {
            // No options viewport when flag is false
        }

        // Handle export request and show result modal
        if self.export_request {
            self.export_request = false;
            if let Ok(json) = self.keybindings.to_json() {
                let dir = crate::config::export_dir();
                let _ = std::fs::create_dir_all(&dir);
                let epoch = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                let file_name = format!("keybindings_{epoch}.json");
                let path = dir.join(file_name);
                if std::fs::write(&path, json).is_ok() {
                    self.export_modal_path = Some(path.to_string_lossy().to_string());
                }
            }
        }
        if let Some(ref path) = self.export_modal_path.clone() {
            egui::Window::new("Exported")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label("JSON has been exported to:");
                        ui.monospace(path);
                        ui.add_space(10.0);
                        if ui.button("OK").clicked()
                            || ctx.input(|i| {
                                i.key_pressed(egui::Key::Enter) || i.key_pressed(egui::Key::Escape)
                            })
                        {
                            self.export_modal_path = None;
                        }
                    });
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Only render header, search bar, and stats in non-ZEN mode
            if !self.zen_mode {
                // Render header
                crate::ui::header::render_header(
                    ui,
                    &mut self.show_options_window,
                    self.error_message.as_ref(),
                    self.logo_texture.as_ref(),
                );

                // Render search bar
                crate::ui::header::render_search_bar(ui, &mut self.search_query);

                // Render stats bar
                let filtered = self.get_filtered_and_sorted_entries();
                crate::ui::header::render_stats_bar(
                    ui,
                    self.keybindings.entries.len(),
                    filtered.len(),
                );
            }

            // Get filtered and sorted keybindings
            let filtered = self.get_filtered_and_sorted_entries();

            // Keyboard navigation for table (when search/options not focused)
            let search_bar_focused =
                ctx.memory(|m| m.focused() == Some(egui::Id::new("search_bar")));
            if !search_bar_focused && !self.show_options_window {
                let len = filtered.len();
                if len > 0 {
                    let mut sel = self.selected_row.unwrap_or(0);
                    let changed = if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                        sel = (sel + 1).min(len - 1);
                        true
                    } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                        sel = sel.saturating_sub(1);
                        true
                    } else if ctx.input(|i| i.key_pressed(egui::Key::PageDown)) {
                        sel = (sel + 10).min(len - 1);
                        true
                    } else if ctx.input(|i| i.key_pressed(egui::Key::PageUp)) {
                        sel = sel.saturating_sub(10);
                        true
                    } else if ctx.input(|i| i.key_pressed(egui::Key::Home)) {
                        sel = 0;
                        true
                    } else if ctx.input(|i| i.key_pressed(egui::Key::End)) {
                        sel = len - 1;
                        true
                    } else {
                        false
                    };
                    if changed {
                        self.selected_row = Some(sel);
                    }
                } else {
                    self.selected_row = None;
                }
            }

            // Render table
            if let Some(clicked_column) = crate::ui::table::render_table(
                ui,
                &filtered,
                &self.column_visibility,
                self.sort_column,
                self.sort_state,
                self.selected_row,
            ) {
                self.handle_sort_click(clicked_column);
            }
        });
    }
}
