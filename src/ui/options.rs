use super::types::{ColumnVisibility, Theme};
use crate::hyprland::SearchOptions;
use eframe::egui;

pub struct OptionsState<'a> {
    pub theme: &'a mut Theme,
    pub column_visibility: &'a mut ColumnVisibility,
    pub search_options: &'a mut SearchOptions,
    pub zen_mode: &'a mut bool,
    pub show_zen_info_modal: &'a mut bool,
    pub export_request: &'a mut bool,
}

fn save_config(
    theme: Theme,
    column_visibility: &ColumnVisibility,
    search_options: &SearchOptions,
    zen_mode: bool,
) {
    let cfg = crate::config::UserConfig {
        theme,
        column_visibility: column_visibility.clone(),
        search_options: search_options.clone(),
        zen_mode,
    };
    let _ = crate::config::save(&cfg);
}

fn render_theme_section(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    theme: &mut Theme,
    column_visibility: &ColumnVisibility,
    search_options: &SearchOptions,
    zen_mode: bool,
) {
    ui.heading("\u{f050e}  Theme");
    ui.add_space(5.0);
    ui.horizontal(|ui| {
        // Dark label with icon
        let dark_text = egui::RichText::new("\u{f0594} Dark").size(13.0);
        ui.label(dark_text);

        ui.add_space(8.0);

        // Toggle switch (smaller)
        let mut is_light = matches!(theme, Theme::Light);
        let desired_size = egui::vec2(36.0, 18.0);
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        if response.clicked() {
            is_light = !is_light;
            *theme = if is_light { Theme::Light } else { Theme::Dark };
            // autosave on theme change
            save_config(*theme, column_visibility, search_options, zen_mode);
        }

        let bg_color = if is_light {
            egui::Color32::from_rgb(100, 149, 237)
        } else {
            egui::Color32::from_rgb(60, 60, 60)
        };

        let knob_offset = if is_light {
            rect.width() - rect.height()
        } else {
            0.0
        };

        let painter = ui.painter();
        painter.rect_filled(rect, rect.height() / 2.0, bg_color);

        let knob_radius = rect.height() / 2.0 - 2.5;
        let knob_center = egui::pos2(
            rect.min.x + rect.height() / 2.0 + knob_offset,
            rect.center().y,
        );
        painter.circle_filled(knob_center, knob_radius, egui::Color32::WHITE);

        ui.add_space(8.0);

        // Light label with icon
        let light_text = egui::RichText::new("\u{e30d} Light").size(13.0);
        ui.label(light_text);
    });
    ui.add_space(8.0);
    let tip = format!(
        "Re-apply {}",
        crate::ui::styling::css::default_css_path().to_string_lossy()
    );
    if ui.button("Reload CSS").on_hover_text(tip).clicked() {
        crate::ui::styling::css::apply_default_if_exists(ctx);
    }
    ui.add_space(10.0);
}

fn render_column_visibility_section(
    ui: &mut egui::Ui,
    theme: Theme,
    column_visibility: &mut ColumnVisibility,
    search_options: &SearchOptions,
    zen_mode: bool,
) {
    ui.separator();
    ui.add_space(10.0);

    ui.heading("\u{f0db}  Visible Columns");
    ui.add_space(5.0);
    let r1 = ui.checkbox(&mut column_visibility.keybind, "\u{ea65}  Keybind");
    let r2 = ui.checkbox(&mut column_visibility.description, "\u{f29e}  Description");
    let r3 = ui.checkbox(&mut column_visibility.command, "\u{ebc4}  Command");
    if r1.changed() || r2.changed() || r3.changed() {
        save_config(theme, column_visibility, search_options, zen_mode);
    }
    ui.add_space(10.0);
}

fn render_search_options_section(
    ui: &mut egui::Ui,
    theme: Theme,
    column_visibility: &ColumnVisibility,
    search_options: &mut SearchOptions,
    zen_mode: bool,
) {
    ui.separator();
    ui.add_space(10.0);

    ui.heading("\u{e68f}  Search Options");
    ui.add_space(5.0);
    ui.label("Search in:");
    let s1 = ui.checkbox(&mut search_options.keybind, "\u{ea65}  Keybind");
    let s2 = ui.checkbox(&mut search_options.description, "\u{f29e}  Description");
    let s3 = ui.checkbox(&mut search_options.command, "\u{ebc4}  Command");
    if s1.changed() || s2.changed() || s3.changed() {
        save_config(theme, column_visibility, search_options, zen_mode);
    }
    ui.add_space(10.0);
}

fn render_zen_mode_section(
    ui: &mut egui::Ui,
    theme: Theme,
    column_visibility: &ColumnVisibility,
    search_options: &SearchOptions,
    zen_mode: &mut bool,
    show_zen_info_modal: &mut bool,
) {
    ui.separator();
    ui.add_space(10.0);

    ui.heading("\u{f06e}  ZEN Mode");
    ui.add_space(5.0);
    ui.label("Hide all distractions and focus on keybindings.");
    ui.add_space(5.0);
    if ui
        .button(egui::RichText::new("Enable ZEN Mode").size(14.0))
        .clicked()
    {
        *zen_mode = true;
        *show_zen_info_modal = true;
        save_config(theme, column_visibility, search_options, *zen_mode);
    }

    ui.add_space(10.0);
}

fn render_export_section(ui: &mut egui::Ui, export_request: &mut bool) {
    ui.separator();
    ui.add_space(10.0);

    ui.heading("\u{ebc4}  Export");
    ui.add_space(5.0);
    if ui
        .button(egui::RichText::new("Export JSON").size(14.0))
        .clicked()
    {
        *export_request = true;
    }
}

pub fn render_options_contents(ctx: &egui::Context, ui: &mut egui::Ui, state: &mut OptionsState) {
    render_theme_section(
        ctx,
        ui,
        state.theme,
        state.column_visibility,
        state.search_options,
        *state.zen_mode,
    );
    render_column_visibility_section(
        ui,
        *state.theme,
        state.column_visibility,
        state.search_options,
        *state.zen_mode,
    );
    render_search_options_section(
        ui,
        *state.theme,
        state.column_visibility,
        state.search_options,
        *state.zen_mode,
    );
    render_zen_mode_section(
        ui,
        *state.theme,
        state.column_visibility,
        state.search_options,
        state.zen_mode,
        state.show_zen_info_modal,
    );
    render_export_section(ui, state.export_request);
}
