use eframe::egui;
use std::fs;
use std::path::PathBuf;

fn parse_hex_color(s: &str) -> Option<egui::Color32> {
    let s = s.trim();
    let hex = s.strip_prefix('#').unwrap_or(s);
    let (r, g, b, a) = match hex.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            (r, g, b, 255)
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            (r, g, b, a)
        }
        _ => return None,
    };
    Some(egui::Color32::from_rgba_unmultiplied(r, g, b, a))
}

fn parse_number(s: &str) -> Option<f32> {
    s.trim().parse::<f32>().ok()
}

fn extract_vars(contents: &str) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    for line in contents.lines() {
        let line = line.trim();
        if line.starts_with("--") {
            if let Some(colon) = line.find(':') {
                let key = &line[..colon].trim();
                let mut value = &line[colon + 1..];
                if let Some(semi) = value.find(';') { value = &value[..semi]; }
                map.insert(key.trim().trim_start_matches('-').to_string(), value.trim().to_string());
            }
        }
        // Also handle lines within :root { ... }
        if let Some(start) = line.find("--") {
            let rest = &line[start..];
            if let Some(colon) = rest.find(':') {
                let key = &rest[..colon].trim();
                let mut value = &rest[colon + 1..];
                if let Some(semi) = value.find(';') { value = &value[..semi]; }
                if key.starts_with("--") {
                    map.insert(key.trim().trim_start_matches('-').to_string(), value.trim().to_string());
                }
            }
        }
    }
    map
}

pub fn apply_from_path(ctx: &egui::Context, path: &str) -> Result<(), String> {
    let contents = fs::read_to_string(path).map_err(|e| format!("Failed to read CSS: {}", e))?;
    let vars = extract_vars(&contents);

    // Expected variables (all optional):
    // bg, fg, panel, accent, stroke, selection, radius, spacing
    let bg = vars.get("bg").and_then(|v| parse_hex_color(v));
    let fg = vars.get("fg").and_then(|v| parse_hex_color(v));
    let panel = vars.get("panel").and_then(|v| parse_hex_color(v));
    let accent = vars.get("accent").and_then(|v| parse_hex_color(v));
    let stroke = vars.get("stroke").and_then(|v| parse_hex_color(v));
    let selection = vars.get("selection").and_then(|v| parse_hex_color(v));
    let radius = vars.get("radius").and_then(|v| parse_number(v));
    let spacing = vars.get("spacing").and_then(|v| parse_number(v));

    let mut style = (*ctx.style()).clone();
    let mut visuals = style.visuals.clone();

    if let Some(bg) = bg { visuals.extreme_bg_color = bg; }
    if let Some(panel) = panel { visuals.panel_fill = panel; }
    if let Some(accent) = accent { visuals.hyperlink_color = accent; }
    if let Some(selection_bg) = selection { visuals.selection.bg_fill = selection_bg; }
    if let Some(stroke_c) = stroke { visuals.selection.stroke.color = stroke_c; }

    // Widgets styling
    let apply_widget = |w: &mut egui::style::WidgetVisuals| {
        if let Some(fg) = fg { w.fg_stroke.color = fg; }
        if let Some(bg) = bg { w.weak_bg_fill = bg.gamma_multiply(0.6); }
        if let Some(panel) = panel { w.bg_fill = panel; }
        if let Some(stroke_c) = stroke { w.bg_stroke.color = stroke_c; }
        if let Some(r) = radius { w.corner_radius = egui::CornerRadius::same(r as u8); }
    };
    apply_widget(&mut visuals.widgets.noninteractive);
    apply_widget(&mut visuals.widgets.inactive);
    apply_widget(&mut visuals.widgets.hovered);
    apply_widget(&mut visuals.widgets.active);
    apply_widget(&mut visuals.widgets.open);

    if let Some(sp) = spacing {
        style.spacing.item_spacing = egui::vec2(sp, sp);
        style.spacing.button_padding = egui::vec2(sp, sp);
        style.spacing.menu_margin = egui::Margin::same(sp as i8);
        style.spacing.window_margin = egui::Margin::same(sp as i8);
    }

    style.visuals = visuals;
    ctx.set_style(style);

    Ok(())
}

pub fn default_css_path() -> PathBuf {
    // ~/.config/hyprbind/hyprbind-theme.css
    #[allow(deprecated)]
    let mut dir = if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        PathBuf::from(xdg)
    } else if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".config")
    } else {
        PathBuf::from(".config")
    };
    dir.push("hyprbind");
    dir.push("hyprbind-theme.css");
    dir
}

pub fn apply_default_if_exists(ctx: &egui::Context) {
    let path = default_css_path();
    if path.exists() {
        let _ = apply_from_path(ctx, &path.to_string_lossy());
    }
}

pub fn has_custom_theme() -> bool {
    default_css_path().exists()
}

pub fn write_default_css(overwrite: bool) -> Result<PathBuf, String> {
    let dir = crate::config::config_dir();
    fs::create_dir_all(&dir).map_err(|e| format!("Failed to create config dir: {}", e))?;
    let path = default_css_path();
    if path.exists() && !overwrite {
        return Err(format!("CSS already exists at {} (use --force-write-default-css to overwrite)", path.to_string_lossy()));
    }

    let epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let css = format!(
        "/* HyprBind default CSS generated at epoch {} */\n:root {{\n  --bg: #0f1117;\n  --fg: #d4d7dc;\n  --panel: #151922;\n  --accent: #7aa2f7;\n  --stroke: #3b4261;\n  --selection: #283457;\n  --radius: 6;\n  --spacing: 6;\n}}\n",
        epoch
    );

    fs::write(&path, css).map_err(|e| format!("Failed to write CSS: {}", e))?;
    Ok(path)
}
