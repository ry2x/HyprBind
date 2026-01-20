use crate::config::UserConfig;
use crate::hyprland::{KeyBindings, SearchOptions, fetch_hyprctl_binds, parse_binds_output};
use crate::ui::{ColumnVisibility, SortColumn, SortState, Theme};
use eframe::egui;

#[allow(clippy::struct_excessive_bools)]
#[derive(Default)]
pub struct AppFlags {
    pub show_options_window: bool,
    pub zen_mode: bool,
    pub show_zen_info_modal: bool,
    pub export_request: bool,
}

pub struct AppState {
    pub keybindings: KeyBindings,
    pub search_query: String,
    pub error_message: Option<String>,
    pub search_options: SearchOptions,
    pub sort_column: SortColumn,
    pub sort_state: SortState,
    pub flags: AppFlags,
    pub theme: Theme,
    pub column_visibility: ColumnVisibility,
    pub logo_texture: Option<egui::TextureHandle>,
    pub selected_row: Option<usize>,
    pub export_modal_path: Option<String>,
    pub last_css_mtime: Option<std::time::SystemTime>,
}

impl AppState {
    pub fn new() -> Self {
        let (keybindings, error_message) = match fetch_hyprctl_binds() {
            Ok(raw_output) => (parse_binds_output(&raw_output), None),
            Err(e) => (
                KeyBindings::new(),
                Some(format!("Failed to load keybindings: {e}")),
            ),
        };

        let mut state = Self {
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

        state.load_config();
        state
    }

    fn load_config(&mut self) {
        if let Some(cfg) = crate::config::load() {
            self.theme = cfg.theme;
            self.column_visibility = cfg.column_visibility;
            self.search_options = cfg.search_options;
            self.flags.zen_mode = cfg.zen_mode;
        }
    }

    pub fn save_config(&self) {
        let cfg = UserConfig {
            theme: self.theme,
            column_visibility: self.column_visibility.clone(),
            search_options: self.search_options.clone(),
            zen_mode: self.flags.zen_mode,
        };
        let _ = crate::config::save(&cfg);
    }
}
