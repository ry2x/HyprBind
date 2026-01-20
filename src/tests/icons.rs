#[cfg(test)]
mod icons_tests {
    use crate::ui::styling::icons::get_icon;

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
