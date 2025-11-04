use eframe::egui;
use egui_extras::{TableBuilder, Column};
use crate::models::KeyBindEntry;
use crate::icons::get_icon;
use super::types::{SortColumn, SortState, ColumnVisibility};

pub fn render_sort_button(
    ui: &mut egui::Ui,
    label: &str,
    column: SortColumn,
    sort_column: SortColumn,
    sort_state: SortState,
) -> bool {
    let mut button_text = label.to_string();
    let sort_indicator = if sort_column == column {
        match sort_state {
            SortState::Ascending => " ▲",
            SortState::Descending => " ▼",
            SortState::None => "",
        }
    } else {
        ""
    };
    button_text.push_str(sort_indicator);
    
    let is_active = sort_column == column && sort_state != SortState::None;
    let button = egui::Button::new(egui::RichText::new(button_text).strong().size(14.0))
        .fill(if is_active {
            ui.visuals().widgets.active.bg_fill
        } else {
            egui::Color32::TRANSPARENT
        })
        .stroke(egui::Stroke::NONE);
    
    ui.add(button).clicked()
}

fn render_header_cell(ui: &mut egui::Ui, label: &str, column: SortColumn, sort_column: SortColumn, sort_state: SortState) -> bool {
    let rect = ui.max_rect();
    let bg_color = if ui.visuals().dark_mode {
        egui::Color32::from_rgb(30, 30, 35)
    } else {
        egui::Color32::from_rgb(245, 245, 250)
    };
    ui.painter().rect_filled(rect, 0.0, bg_color);
    
    let mut clicked = false;
    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center).with_main_justify(true), |ui| {
        ui.add_space(8.0);
        clicked = render_sort_button(ui, label, column, sort_column, sort_state);
        ui.add_space(8.0);
    });
    clicked
}

fn render_keybind_cell(ui: &mut egui::Ui, entry: &KeyBindEntry) {
    ui.add_space(8.0);
    
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

    key_frame.show(ui, |ui| {
        ui.label(egui::RichText::new(get_icon(&entry.key)).size(13.0));
    });

    ui.label(egui::RichText::new(" ").size(12.0));
}

fn render_description_cell(ui: &mut egui::Ui, entry: &KeyBindEntry) {
    ui.add_space(8.0);
    let description = if entry.description.is_empty() {
        egui::RichText::new("-").weak()
    } else {
        egui::RichText::new(&entry.description)
    };
    ui.label(description);
}

fn render_command_cell(ui: &mut egui::Ui, entry: &KeyBindEntry) {
    ui.add_space(8.0);
    ui.label(egui::RichText::new(&entry.command)
        .size(12.0)
        .color(
            if ui.visuals().dark_mode {
                egui::Color32::from_rgb(180, 180, 190)
            } else {
                egui::Color32::from_rgb(80, 80, 90)
            }
        ))
        .on_hover_text(&entry.command);
}

pub fn render_table(
    ui: &mut egui::Ui,
    filtered: &[KeyBindEntry],
    column_visibility: &ColumnVisibility,
    sort_column: SortColumn,
    sort_state: SortState,
    selected_row: Option<usize>,
) -> Option<SortColumn> {
    let visible_count = [
        column_visibility.keybind,
        column_visibility.description,
        column_visibility.command,
    ].iter().filter(|&&v| v).count();
    
    // Remove vertical lines by making separator invisible
    ui.style_mut().visuals.widgets.noninteractive.bg_stroke = egui::Stroke::NONE;
    ui.style_mut().visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;
    
    let mut table = TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center));
    
    let mut col_index = 0;
    
    if column_visibility.keybind {
        col_index += 1;
        if col_index == visible_count {
            table = table.column(Column::remainder().at_least(100.0).resizable(true).clip(true));
        } else {
            table = table.column(Column::initial(250.0).at_least(100.0).resizable(true).clip(true));
        }
    }
    if column_visibility.description {
        col_index += 1;
        if col_index == visible_count {
            table = table.column(Column::remainder().at_least(200.0).resizable(true).clip(true));
        } else {
            table = table.column(Column::initial(300.0).at_least(100.0).resizable(true).clip(true));
        }
    }
    if column_visibility.command {
        col_index += 1;
        if col_index == visible_count {
            table = table.column(Column::remainder().at_least(200.0).resizable(true).clip(true));
        } else {
            table = table.column(Column::initial(300.0).at_least(100.0).resizable(true).clip(true));
        }
    }
    
    let mut clicked_column = None;
    
    table
        .header(35.0, |mut header| {
            if column_visibility.keybind {
                header.col(|ui| {
                    if render_header_cell(ui, "Keybind", SortColumn::Keybind, sort_column, sort_state) {
                        clicked_column = Some(SortColumn::Keybind);
                    }
                });
            }
            if column_visibility.description {
                header.col(|ui| {
                    if render_header_cell(ui, "Description", SortColumn::Description, sort_column, sort_state) {
                        clicked_column = Some(SortColumn::Description);
                    }
                });
            }
            if column_visibility.command {
                header.col(|ui| {
                    if render_header_cell(ui, "Command", SortColumn::Command, sort_column, sort_state) {
                        clicked_column = Some(SortColumn::Command);
                    }
                });
            }
        })
        .body(|mut body| {
            for (idx, entry) in filtered.iter().enumerate() {
                body.row(32.0, |mut row| {
                    if column_visibility.keybind {
                        row.col(|ui| {
                            ui.set_min_height(32.0);
                            if let Some(sel) = selected_row {
                                if sel == idx {
                                    let rect = ui.max_rect();
                                    let hl = if ui.visuals().dark_mode { egui::Color32::from_rgb(50, 60, 80) } else { egui::Color32::from_rgb(210, 220, 245) };
                                    ui.painter().rect_filled(rect, 0.0, hl);
                                    ui.scroll_to_rect(rect, None);
                                }
                            }
                            render_keybind_cell(ui, entry);
                        });
                    }
                    if column_visibility.description {
                        row.col(|ui| {
                            ui.set_min_height(32.0);
                            if let Some(sel) = selected_row {
                                if sel == idx {
                                    let rect = ui.max_rect();
                                    let hl = if ui.visuals().dark_mode { egui::Color32::from_rgb(50, 60, 80) } else { egui::Color32::from_rgb(210, 220, 245) };
                                    ui.painter().rect_filled(rect, 0.0, hl);
                                }
                            }
                            render_description_cell(ui, entry);
                        });
                    }
                    if column_visibility.command {
                        row.col(|ui| {
                            ui.set_min_height(32.0);
                            if let Some(sel) = selected_row {
                                if sel == idx {
                                    let rect = ui.max_rect();
                                    let hl = if ui.visuals().dark_mode { egui::Color32::from_rgb(50, 60, 80) } else { egui::Color32::from_rgb(210, 220, 245) };
                                    ui.painter().rect_filled(rect, 0.0, hl);
                                }
                            }
                            render_command_cell(ui, entry);
                        });
                    }
                });
            }
        });
    
    clicked_column
}
