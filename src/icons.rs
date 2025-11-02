/// Get icon for common keys and modifiers using Nerd Font
pub fn get_icon(key: &str) -> String {
    match key {
        // Modifiers
        "SUPER" => "\u{f17a} ".to_string(), // Apple Command key icon
        "SHIFT" => "\u{f0636} ".to_string(), // Shift key icon

        // Keys
        k if k.eq_ignore_ascii_case("RETURN") || k.eq_ignore_ascii_case("ENTER") => "\u{f0311} ".to_string(),
        k if k.eq_ignore_ascii_case("SEMICOLON") => ";".to_string(),
        k if k.eq_ignore_ascii_case("DELETE") => "DEL".to_string(),
        k if k.eq_ignore_ascii_case("TAB") => "TAB".to_string(),
        k if k.eq_ignore_ascii_case("LEFT") => "\u{f0731} ".to_string(),
        k if k.eq_ignore_ascii_case("RIGHT") => "\u{f0734} ".to_string(),
        k if k.eq_ignore_ascii_case("UP") => "\u{f0737} ".to_string(),
        k if k.eq_ignore_ascii_case("DOWN") => "\u{f072e} ".to_string(),
        k if k.eq_ignore_ascii_case("mouse_down") => "\u{f1550} ".to_string(),
        k if k.eq_ignore_ascii_case("mouse_up") => "\u{f1551} ".to_string(),
        k if k.eq_ignore_ascii_case("mouse:272") => "\u{f522}\u{f037d} ".to_string(),
        k if k.eq_ignore_ascii_case("mouse:273") => "\u{f037d}\u{f522} ".to_string(),
        k if k.eq_ignore_ascii_case("XF86AudioRaiseVolume") => "\u{f028} ".to_string(),
        k if k.eq_ignore_ascii_case("XF86AudioLowerVolume") => "\u{f027} ".to_string(),
        k if k.eq_ignore_ascii_case("XF86AudioMute") => "\u{eee8} ".to_string(),
        k if k.eq_ignore_ascii_case("XF86AudioMicMute") => "\u{f036d} ".to_string(),
        k if k.eq_ignore_ascii_case("XF86MonBrightnessUp") => "\u{f00e0} ".to_string(),
        k if k.eq_ignore_ascii_case("XF86MonBrightnessDown") => "\u{f00de} ".to_string(),
        k if k.eq_ignore_ascii_case("XF86AudioNext") => "\u{f0661} ".to_string(),
        k if k.eq_ignore_ascii_case("XF86AudioPause") => "\u{f04c} ".to_string(),
        k if k.eq_ignore_ascii_case("XF86AudioPlay") => "\u{f04b} ".to_string(),
        k if k.eq_ignore_ascii_case("XF86AudioPrev") => "\u{f0663} ".to_string(),
        _ => key.to_string(),
    }
}
