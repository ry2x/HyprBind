use crate::models::{KeyBindings, SearchOptions};
use crate::parser::parse_hyprctl_binds;
use crate::ui::{ColumnVisibility, SortColumn, SortState, Theme};
use eframe::egui;
use std::cmp::Ordering;

#[allow(clippy::struct_excessive_bools)]
#[derive(Default)]
pub struct AppFlags {
    pub show_options_window: bool,
    pub zen_mode: bool,
    pub show_zen_info_modal: bool,
    pub export_request: bool,
}


pub struct KeybindsApp {
    keybindings: KeyBindings,
    search_query: String,
    error_message: Option<String>,
    search_options: SearchOptions,
    sort_column: SortColumn,
    sort_state: SortState,
    flags: AppFlags,
    theme: Theme,
    column_visibility: ColumnVisibility,
    logo_texture: Option<egui::TextureHandle>,
    selected_row: Option<usize>,
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
            flags: AppFlags::default(),
            theme: Theme::Dark,
            column_visibility: ColumnVisibility::default(),
            logo_texture: None,
            selected_row: None,
            export_modal_path: None,
            last_css_mtime: None,
        };
        if let Some(cfg) = crate::config::load() {
            app.theme = cfg.theme;
            app.column_visibility = cfg.column_visibility;
            app.search_options = cfg.search_options;
            app.flags.zen_mode = cfg.zen_mode;
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

impl KeybindsApp {
    fn load_logo_texture_if_needed(&mut self, ctx: &egui::Context) {
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
    }

    fn save_config(&self) {
        let cfg = crate::config::UserConfig {
            theme: self.theme,
            column_visibility: self.column_visibility.clone(),
            search_options: self.search_options.clone(),
            zen_mode: self.flags.zen_mode,
        };
        let _ = crate::config::save(&cfg);
    }

    fn handle_zen_mode_shortcuts(&mut self, ctx: &egui::Context) {
        let prev_zen = self.flags.zen_mode;
        crate::ui::zen::handle_zen_keyboard_shortcuts(ctx, &mut self.flags.zen_mode);
        if self.flags.zen_mode != prev_zen {
            self.save_config();
        }
    }

    fn handle_search_bar_focus(&self, ctx: &egui::Context) {
        let search_bar_focused = ctx.memory(|m| m.focused() == Some(egui::Id::new("search_bar")));
        if !self.flags.zen_mode && !search_bar_focused && ctx.input(|i| i.key_pressed(egui::Key::Slash)) {
            ctx.memory_mut(|m| m.request_focus(egui::Id::new("search_bar")));
            ctx.input_mut(|i| {
                i.events
                    .retain(|e| !matches!(e, egui::Event::Text(s) if s == "/"));
            });
        }
    }

    fn apply_theme_or_css(&mut self, ctx: &egui::Context) {
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
    }

    fn handle_zen_info_modal(&mut self, ctx: &egui::Context) {
        if self.flags.show_zen_info_modal {
            crate::ui::zen::render_zen_info_modal(
                ctx,
                &mut self.flags.show_zen_info_modal,
                &mut self.flags.show_options_window,
            );
            if ctx.input(|i| i.key_pressed(egui::Key::Z)) {
                self.flags.show_zen_info_modal = false;
            }
        }
    }

    fn handle_options_window(&mut self, ctx: &egui::Context) {
        if self.flags.show_options_window && ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.flags.show_options_window = false;
        }

        let options_viewport_id = egui::ViewportId::from_hash_of("options");
        if self.flags.show_options_window {
            ctx.show_viewport_immediate(
                options_viewport_id,
                egui::ViewportBuilder::default()
                    .with_title("HyprBind – Options")
                    .with_resizable(true)
                    .with_min_inner_size([400.0, 420.0])
                    .with_inner_size([520.0, 560.0]),
                |vctx, _class| {
                    if vctx.input(|i| i.viewport().close_requested()) {
                        self.flags.show_options_window = false;
                    }
                    egui::CentralPanel::default().show(vctx, |ui| {
                        let prev_zen = self.flags.zen_mode;
                        let mut state = crate::ui::options::OptionsState {
                            theme: &mut self.theme,
                            column_visibility: &mut self.column_visibility,
                            search_options: &mut self.search_options,
                            zen_mode: &mut self.flags.zen_mode,
                            show_zen_info_modal: &mut self.flags.show_zen_info_modal,
                            export_request: &mut self.flags.export_request,
                        };
                        crate::ui::options::render_options_contents(vctx, ui, &mut state);
                        if !prev_zen && self.flags.zen_mode {
                            self.flags.show_options_window = false;
                        }
                    });
                },
            );
        }
    }

    fn handle_export_request(&mut self) {
        if self.flags.export_request {
            self.flags.export_request = false;
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
    }

    fn handle_export_modal(&mut self, ctx: &egui::Context) {
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
    }

    fn handle_keyboard_navigation(&mut self, ctx: &egui::Context, filtered_len: usize) {
        let search_bar_focused = ctx.memory(|m| m.focused() == Some(egui::Id::new("search_bar")));
        if !search_bar_focused && !self.flags.show_options_window && filtered_len > 0 {
            let mut sel = self.selected_row.unwrap_or(0);
            let changed = if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                sel = (sel + 1).min(filtered_len - 1);
                true
            } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                sel = sel.saturating_sub(1);
                true
            } else if ctx.input(|i| i.key_pressed(egui::Key::PageDown)) {
                sel = (sel + 10).min(filtered_len - 1);
                true
            } else if ctx.input(|i| i.key_pressed(egui::Key::PageUp)) {
                sel = sel.saturating_sub(10);
                true
            } else if ctx.input(|i| i.key_pressed(egui::Key::Home)) {
                sel = 0;
                true
            } else if ctx.input(|i| i.key_pressed(egui::Key::End)) {
                sel = filtered_len - 1;
                true
            } else {
                false
            };
            if changed {
                self.selected_row = Some(sel);
            }
        } else if filtered_len == 0 {
            self.selected_row = None;
        }
    }

    fn render_main_ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.flags.zen_mode {
                crate::ui::header::render_header(
                    ui,
                    &mut self.flags.show_options_window,
                    self.error_message.as_ref(),
                    self.logo_texture.as_ref(),
                );

                crate::ui::header::render_search_bar(ui, &mut self.search_query);

                let filtered = self.get_filtered_and_sorted_entries();
                crate::ui::header::render_stats_bar(
                    ui,
                    self.keybindings.entries.len(),
                    filtered.len(),
                );
            }

            let filtered = self.get_filtered_and_sorted_entries();
            self.handle_keyboard_navigation(ctx, filtered.len());

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

impl eframe::App for KeybindsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.load_logo_texture_if_needed(ctx);
        self.handle_zen_mode_shortcuts(ctx);
        self.handle_search_bar_focus(ctx);
        self.apply_theme_or_css(ctx);
        self.handle_zen_info_modal(ctx);
        self.handle_options_window(ctx);
        self.handle_export_request();
        self.handle_export_modal(ctx);
        self.render_main_ui(ctx);
    }
}

