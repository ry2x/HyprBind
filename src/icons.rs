use std::collections::HashMap;
use std::sync::OnceLock;

/// Get icon for common keys and modifiers using Nerd Font
pub fn get_icon(key: &str) -> String {
    // Static table for direct key-icon mapping (case-insensitive)
    fn icon_table() -> &'static HashMap<&'static str, &'static str> {
        static TABLE: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

        TABLE.get_or_init(|| {
            let mut m: HashMap<&str, &str> = HashMap::new();
            m.insert("super", "");
            m.insert("shift", " 󰘶 ");
            m.insert("return", "󰌑");
            m.insert("enter", "󰌑");
            m.insert("semicolon", ";");
            m.insert("delete", "DEL");
            m.insert("tab", "TAB");
            m.insert("left", "󰜱");
            m.insert("right", "󰜴");
            m.insert("up", "󰜷");
            m.insert("down", "󰜮");
            m.insert("mouse_down", "󱕐");
            m.insert("mouse_up", "󱕑");
            m.insert("mouse:272", "󰍽");
            m.insert("mouse:273", "󰍽");
            m.insert("xf86audioraisevolume", "");
            m.insert("xf86audiolowervolume", "");
            m.insert("xf86audiomute", "");
            m.insert("xf86audiomicmute", "󰍭");
            m.insert("xf86monbrightnessup", "󰃠");
            m.insert("xf86monbrightnessdown", "󰃞");
            m.insert("xf86audionext", "󰙡");
            m.insert("xf86audiopause", "");
            m.insert("xf86audioplay", "");
            m.insert("xf86audioprev", "󰙣");
            m
        })
    }

    let key_lower: String = key.to_ascii_lowercase();

    // Table lookup (case-insensitive)
    if let Some(&icon) = icon_table().get(key_lower.as_str()) {
        return icon.to_string();
    }

    // Fallback: return the key itself
    key.to_string()
}

#[cfg(test)]
mod tests {
    use super::get_icon;

    #[test]
    fn test_get_icon() {
        let cases: [(&str, &str); 28] = [
            ("SUPER", ""),
            ("SHIFT", " 󰘶 "),
            ("RETURN", "󰌑"),
            ("ENTER", "󰌑"),
            ("SEMICOLON", ";"),
            ("DELETE", "DEL"),
            ("TAB", "TAB"),
            ("LEFT", "󰜱"),
            ("RIGHT", "󰜴"),
            ("UP", "󰜷"),
            ("DOWN", "󰜮"),
            ("mouse_down", "󱕐"),
            ("mouse_up", "󱕑"),
            ("mouse:272", "󰍽"),
            ("mouse:273", "󰍽"),
            ("XF86AudioRaiseVolume", ""),
            ("XF86AudioLowerVolume", ""),
            ("XF86AudioMute", ""),
            ("XF86AudioMicMute", "󰍭"),
            ("XF86MonBrightnessUp", "󰃠"),
            ("XF86MonBrightnessDown", "󰃞"),
            ("XF86AudioNext", "󰙡"),
            ("XF86AudioPause", ""),
            ("XF86AudioPlay", ""),
            ("XF86AudioPrev", "󰙣"),
            ("UNKNOWN_KEY", "UNKNOWN_KEY"),
            ("A", "A"),
            ("123", "123"),
        ];

        for (input, expected) in &cases {
            assert_eq!(get_icon(input), *expected);
        }
    }
}
