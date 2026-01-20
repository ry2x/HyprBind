pub mod sorting;
mod state;

pub use state::AppState;

use crate::ui::SortColumn;
use eframe::egui;

pub struct KeybindsApp {
    state: AppState,
}

impl KeybindsApp {
    pub fn new() -> Self {
        Self {
            state: AppState::new(),
        }
    }

    fn handle_sort_click(&mut self, column: SortColumn) {
        let (new_column, new_state) =
            sorting::next_sort_state(self.state.sort_column, column, self.state.sort_state);
        self.state.sort_column = new_column;
        self.state.sort_state = new_state;
    }

    fn get_filtered_and_sorted_entries(&self) -> Vec<crate::hyprland::KeyBindEntry> {
        sorting::filter_and_sort(
            &self.state.keybindings.entries,
            &self.state.search_query,
            &self.state.search_options,
            self.state.sort_column,
            self.state.sort_state,
        )
    }

    fn load_logo_texture_if_needed(&mut self, ctx: &egui::Context) {
        if self.state.logo_texture.is_none() {
            let logo_bytes = include_bytes!("../../assets/logo_hyprbind.png");
            if let Ok(image) = image::load_from_memory(logo_bytes) {
                let size = [image.width() as usize, image.height() as usize];
                let image_buffer = image.to_rgba8();
                let pixels = image_buffer.as_flat_samples();
                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                self.state.logo_texture =
                    Some(ctx.load_texture("logo", color_image, egui::TextureOptions::LINEAR));
            }
        }
    }

    fn handle_zen_mode_shortcuts(&mut self, ctx: &egui::Context) {
        let prev_zen = self.state.flags.zen_mode;
        crate::ui::zen::handle_zen_keyboard_shortcuts(ctx, &mut self.state.flags.zen_mode);
        if self.state.flags.zen_mode != prev_zen {
            self.state.save_config();
        }
    }

    fn handle_search_bar_focus(&self, ctx: &egui::Context) {
        let search_bar_focused = ctx.memory(|m| m.focused() == Some(egui::Id::new("search_bar")));
        if !self.state.flags.zen_mode
            && !search_bar_focused
            && ctx.input(|i| i.key_pressed(egui::Key::Slash))
        {
            ctx.memory_mut(|m| m.request_focus(egui::Id::new("search_bar")));
            ctx.input_mut(|i| {
                i.events
                    .retain(|e| !matches!(e, egui::Event::Text(s) if s == "/"));
            });
        }
    }

    fn apply_theme_or_css(&mut self, ctx: &egui::Context) {
        if crate::ui::styling::css::has_custom_theme() {
            let path = crate::ui::styling::css::default_css_path();
            if let Ok(meta) = std::fs::metadata(&path)
                && let Ok(modified) = meta.modified()
            {
                let changed = self.state.last_css_mtime.is_none_or(|prev| modified > prev);
                if changed {
                    let _ = crate::ui::styling::css::apply_from_path(ctx, &path.to_string_lossy());
                    self.state.last_css_mtime = Some(modified);
                }
            }
        } else {
            match self.state.theme {
                crate::ui::Theme::Dark => ctx.set_visuals(egui::Visuals::dark()),
                crate::ui::Theme::Light => ctx.set_visuals(egui::Visuals::light()),
            }
        }
    }

    fn handle_zen_info_modal(&mut self, ctx: &egui::Context) {
        if self.state.flags.show_zen_info_modal {
            crate::ui::zen::render_zen_info_modal(
                ctx,
                &mut self.state.flags.show_zen_info_modal,
                &mut self.state.flags.show_options_window,
            );
            if ctx.input(|i| i.key_pressed(egui::Key::Z)) {
                self.state.flags.show_zen_info_modal = false;
            }
        }
    }

    fn handle_options_window(&mut self, ctx: &egui::Context) {
        if self.state.flags.show_options_window && ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.state.flags.show_options_window = false;
        }

        let options_viewport_id = egui::ViewportId::from_hash_of("options");
        if self.state.flags.show_options_window {
            ctx.show_viewport_immediate(
                options_viewport_id,
                egui::ViewportBuilder::default()
                    .with_title("HyprBind – Options")
                    .with_resizable(true)
                    .with_min_inner_size([400.0, 420.0])
                    .with_inner_size([520.0, 560.0]),
                |vctx, _class| {
                    if vctx.input(|i| i.viewport().close_requested()) {
                        self.state.flags.show_options_window = false;
                    }
                    egui::CentralPanel::default().show(vctx, |ui| {
                        let prev_zen = self.state.flags.zen_mode;
                        let mut opts = crate::ui::options::OptionsState {
                            theme: &mut self.state.theme,
                            column_visibility: &mut self.state.column_visibility,
                            search_options: &mut self.state.search_options,
                            zen_mode: &mut self.state.flags.zen_mode,
                            show_zen_info_modal: &mut self.state.flags.show_zen_info_modal,
                            export_request: &mut self.state.flags.export_request,
                        };
                        crate::ui::options::render_options_contents(vctx, ui, &mut opts);
                        if !prev_zen && self.state.flags.zen_mode {
                            self.state.flags.show_options_window = false;
                        }
                    });
                },
            );
        }
    }

    fn handle_export_request(&mut self) {
        if self.state.flags.export_request {
            self.state.flags.export_request = false;
            if let Ok(json) = self.state.keybindings.to_json() {
                let dir = crate::config::export_dir();
                let _ = std::fs::create_dir_all(&dir);
                let epoch = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                let file_name = format!("keybindings_{epoch}.json");
                let path = dir.join(file_name);
                if std::fs::write(&path, json).is_ok() {
                    self.state.export_modal_path = Some(path.to_string_lossy().to_string());
                }
            }
        }
    }

    fn handle_export_modal(&mut self, ctx: &egui::Context) {
        if let Some(ref path) = self.state.export_modal_path.clone() {
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
                            self.state.export_modal_path = None;
                        }
                    });
                });
        }
    }

    fn handle_keyboard_navigation(&mut self, ctx: &egui::Context, filtered_len: usize) {
        let search_bar_focused = ctx.memory(|m| m.focused() == Some(egui::Id::new("search_bar")));
        if !search_bar_focused && !self.state.flags.show_options_window && filtered_len > 0 {
            let mut sel = self.state.selected_row.unwrap_or(0);
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
                self.state.selected_row = Some(sel);
            }
        } else if filtered_len == 0 {
            self.state.selected_row = None;
        }
    }

    fn render_main_ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.state.flags.zen_mode {
                crate::ui::header::render_header(
                    ui,
                    &mut self.state.flags.show_options_window,
                    self.state.error_message.as_ref(),
                    self.state.logo_texture.as_ref(),
                );

                crate::ui::header::render_search_bar(ui, &mut self.state.search_query);

                let filtered = self.get_filtered_and_sorted_entries();
                crate::ui::header::render_stats_bar(
                    ui,
                    self.state.keybindings.entries.len(),
                    filtered.len(),
                );
            }

            let filtered = self.get_filtered_and_sorted_entries();
            self.handle_keyboard_navigation(ctx, filtered.len());

            if let Some(clicked_column) = crate::ui::table::render_table(
                ui,
                &filtered,
                &self.state.column_visibility,
                self.state.sort_column,
                self.state.sort_state,
                self.state.selected_row,
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
