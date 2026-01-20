use eframe::egui;

fn render_gradient_text(ui: &mut egui::Ui, text: &str, font_size: f32) {
    // Gradient colors derived from theme accent
    let accent = ui.visuals().hyperlink_color;
    let start_color = accent; // use accent as start
    let end_color = egui::Color32::from_rgb(
        u8::try_from(u16::midpoint(u16::from(accent.r()), 255)).unwrap_or(255),
        u8::try_from(u16::midpoint(u16::from(accent.g()), 255)).unwrap_or(255),
        u8::try_from(u16::midpoint(u16::from(accent.b()), 255)).unwrap_or(255),
    );

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let to_byte = |v: f32| -> u8 {
        // Clamp to [0, 255] and round to nearest before narrowing
        v.clamp(0.0, 255.0).round() as u8
    };

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0; // No spacing between characters

        #[allow(clippy::cast_precision_loss)]
        let char_count = (text.len().saturating_sub(1)).max(1) as f32;

        for (i, ch) in text.chars().enumerate() {
            #[allow(clippy::cast_precision_loss)]
            let t = i as f32 / char_count;

            // Interpolate color
            let r =
                to_byte(f32::from(start_color.r()).mul_add(1.0 - t, f32::from(end_color.r()) * t));
            let g =
                to_byte(f32::from(start_color.g()).mul_add(1.0 - t, f32::from(end_color.g()) * t));
            let b =
                to_byte(f32::from(start_color.b()).mul_add(1.0 - t, f32::from(end_color.b()) * t));
            let color = egui::Color32::from_rgb(r, g, b);

            ui.label(
                egui::RichText::new(ch.to_string())
                    .size(font_size)
                    .strong()
                    .color(color),
            );
        }
    });
}

pub fn render_header(
    ui: &mut egui::Ui,
    show_options_window: &mut bool,
    error_message: Option<&String>,
    logo_texture: Option<&egui::TextureHandle>,
) {
    // Modern header with background
    let header_rect =
        egui::Rect::from_min_size(ui.min_rect().min, egui::vec2(ui.available_width(), 120.0));
    let header_bg = ui.visuals().panel_fill;
    ui.painter().rect_filled(header_rect, 0.0, header_bg);

    ui.add_space(15.0);

    // Title section
    ui.horizontal(|ui| {
        ui.add_space(20.0);

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;

            // Display logo if available
            if let Some(texture) = logo_texture {
                let logo_size = 32.0;
                ui.add(
                    egui::Image::new(texture)
                        .max_width(logo_size)
                        .max_height(logo_size),
                );
            } else {
                ui.label(egui::RichText::new("").size(28.0));
            }

            // Add vertical offset to move text up
            ui.add_space(-2.0);

            // Render gradient text "HyprBind" tinted by accent color
            ui.vertical(|ui| {
                ui.add_space(-1.0); // Negative space to move text up
                render_gradient_text(ui, "HyprBind", 24.0);
            });
        });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(20.0);
            let options_button = egui::Button::new(egui::RichText::new("").size(18.0))
                .fill(egui::Color32::TRANSPARENT)
                .stroke(egui::Stroke::NONE);
            if ui.add(options_button).on_hover_text("Options").clicked() {
                *show_options_window = !*show_options_window;
            }
        });
    });

    ui.add_space(12.0);

    // Display error message if any
    if let Some(error) = error_message {
        ui.horizontal(|ui| {
            ui.add_space(20.0);
            ui.colored_label(egui::Color32::RED, format!("⚠ {error}"));
        });
        ui.add_space(8.0);
    }
}

pub fn render_search_bar(ui: &mut egui::Ui, search_query: &mut String) {
    ui.horizontal(|ui| {
        ui.add_space(20.0);

        // Search icon and input
        ui.label(egui::RichText::new("").size(16.0).weak());
        ui.add_space(5.0);

        let search_bar = egui::TextEdit::singleline(search_query)
            .id(egui::Id::new("search_bar"))
            .hint_text("\u{e68f}  Search keybinds... ( / )")
            .desired_width(ui.available_width() - 140.0);
        ui.add(search_bar);

        ui.add_space(1.0);

        let clear_button = egui::Button::new(egui::RichText::new("\u{eabf} ").size(13.0))
            .fill(ui.visuals().widgets.inactive.weak_bg_fill)
            .stroke(ui.visuals().widgets.inactive.bg_stroke);
        if ui.add(clear_button).clicked() {
            search_query.clear();
        }
    });

    ui.add_space(12.0);
}

pub fn render_stats_bar(ui: &mut egui::Ui, total: usize, showing: usize) {
    ui.horizontal(|ui| {
        ui.add_space(20.0);
        ui.label(
            egui::RichText::new(format!(" Total: {total}"))
                .weak()
                .size(12.0),
        );
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new(format!(" Showing: {showing}"))
                .weak()
                .size(12.0),
        );
    });
    ui.add_space(8.0);

    ui.separator();
}
