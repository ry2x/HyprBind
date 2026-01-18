/// Get icon for common keys and modifiers using Nerd Font
pub fn get_icon(key: &str) -> String {
    match key {
        // Modifiers
        "SUPER" => "".to_string(),   // Apple Command key icon
        "SHIFT" => " 󰘶 ".to_string(), // Shift key icon

        // Keys
        k if k.eq_ignore_ascii_case("RETURN") || k.eq_ignore_ascii_case("ENTER") => "󰌑".to_string(),
        k if k.eq_ignore_ascii_case("SEMICOLON") => ";".to_string(),
        k if k.eq_ignore_ascii_case("DELETE") => "DEL".to_string(),
        k if k.eq_ignore_ascii_case("TAB") => "TAB".to_string(),
        k if k.eq_ignore_ascii_case("LEFT") => "󰜱".to_string(),
        k if k.eq_ignore_ascii_case("RIGHT") => "󰜴".to_string(),
        k if k.eq_ignore_ascii_case("UP") => "󰜷".to_string(),
        k if k.eq_ignore_ascii_case("DOWN") => "󰜮".to_string(),
        k if k.eq_ignore_ascii_case("mouse_down") => "󱕐".to_string(),
        k if k.eq_ignore_ascii_case("mouse_up") => "󱕑".to_string(),
        k if k.eq_ignore_ascii_case("mouse:272") => "󰍽".to_string(),
        k if k.eq_ignore_ascii_case("mouse:273") => "󰍽".to_string(),
        k if k.eq_ignore_ascii_case("XF86AudioRaiseVolume") => "".to_string(),
        k if k.eq_ignore_ascii_case("XF86AudioLowerVolume") => "".to_string(),
        k if k.eq_ignore_ascii_case("XF86AudioMute") => "".to_string(),
        k if k.eq_ignore_ascii_case("XF86AudioMicMute") => "󰍭".to_string(),
        k if k.eq_ignore_ascii_case("XF86MonBrightnessUp") => "󰃠".to_string(),
        k if k.eq_ignore_ascii_case("XF86MonBrightnessDown") => "󰃞".to_string(),
        k if k.eq_ignore_ascii_case("XF86AudioNext") => "󰙡".to_string(),
        k if k.eq_ignore_ascii_case("XF86AudioPause") => "".to_string(),
        k if k.eq_ignore_ascii_case("XF86AudioPlay") => "".to_string(),
        k if k.eq_ignore_ascii_case("XF86AudioPrev") => "󰙣".to_string(),
        _ => key.to_string(),
    }
}
