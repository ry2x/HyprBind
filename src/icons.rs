/// Get Nerd Font icon for common keys
pub fn get_key_icon(key: &str) -> String {
    match key {
        "Return" => format!("\u{f2f6} {}", key),        // nf-md-keyboard_return
        "Escape" => format!("\u{f12b} {}", key),        // nf-md-keyboard_esc
        "Tab" => format!("\u{f0331} {}", key),          // nf-md-keyboard_tab
        "space" => format!("\u{f0332} {}", key),        // nf-md-keyboard_space
        s if s.starts_with("XF86Audio") => format!("\u{f028} {}", s),  // nf-fa-volume_up
        s if s.starts_with("XF86MonBrightness") => format!("\u{f185} {}", s), // nf-fa-sun_o
        s if s.starts_with("mouse_down") => format!("\u{f35d} {}", s), // nf-md-arrow_down
        s if s.starts_with("mouse_up") => format!("\u{f35d} {}", s),   // nf-md-arrow_up
        s if s.starts_with("mouse:272") => format!("\u{f0a54} {}", s), // nf-md-mouse_left
        s if s.starts_with("mouse:273") => format!("\u{f02c7} {}", s), // nf-md-mouse_right
        s if s.starts_with("mouse") => format!("\u{f245} {}", s),      // nf-fa-mouse_pointer
        _ => key.to_string(),
    }
}
