/// Get icon for common keys using Nerd Font
pub fn get_key_icon(key: &str) -> String {
    match key {
        k if k.eq_ignore_ascii_case("RETURN") || k.eq_ignore_ascii_case("ENTER") => format!("\u{f0311}"),
        k if k.eq_ignore_ascii_case("SEMICOLON") => format!(";"),
        k if k.eq_ignore_ascii_case("DELETE") => format!("DEL"),
        k if k.eq_ignore_ascii_case("TAB") => format!("TAB"),
        k if k.eq_ignore_ascii_case("LEFT") => format!("\u{f0731}"),
        k if k.eq_ignore_ascii_case("RIGHT") => format!("\u{f0734}"),
        k if k.eq_ignore_ascii_case("UP") => format!("\u{f0737}"),
        k if k.eq_ignore_ascii_case("DOWN") => format!("\u{f072e}"),
        k if k.eq_ignore_ascii_case("mouse_down") => format!("\u{f1550}"),
        k if k.eq_ignore_ascii_case("mouse_up") => format!("\u{f1551}"),
        k if k.eq_ignore_ascii_case("mouse:272") => format!("\u{f522}\u{f037d}"),
        k if k.eq_ignore_ascii_case("mouse:273") => format!("\u{f037d}\u{f522}"),
        k if k.eq_ignore_ascii_case("XF86AudioRaiseVolume") => format!("\u{f028}"),
        k if k.eq_ignore_ascii_case("XF86AudioLowerVolume") => format!("\u{f027}"),
        k if k.eq_ignore_ascii_case("XF86AudioMute") => format!("\u{eee8}"),
        k if k.eq_ignore_ascii_case("XF86AudioMicMute") => format!("\u{f036d}"),
        k if k.eq_ignore_ascii_case("XF86MonBrightnessUp") => format!("\u{f00e0}"),
        k if k.eq_ignore_ascii_case("XF86MonBrightnessDown") => format!("\u{f00de}"),
        k if k.eq_ignore_ascii_case("XF86AudioNext") => format!("\u{f0661}"),
        k if k.eq_ignore_ascii_case("XF86AudioPause") => format!("\u{f04c}"),
        k if k.eq_ignore_ascii_case("XF86AudioPlay") => format!("\u{f04b}"),
        k if k.eq_ignore_ascii_case("XF86AudioPrev") => format!("\u{f0663}"),
        _ => key.to_string(),
    }
}
