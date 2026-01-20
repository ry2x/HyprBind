#[cfg(test)]
mod table_tests {
    use crate::ui::table::is_nerd_font_icon;

    #[test]
    fn test_is_nerd_font_icon() {
        // Test with NerdFonts
        let nerd_fonts: [&str; 21] = [
            "", "󰘶", "󰌑", "󰜱", "󰜴", "󰜷", "󰜮", "󱕐", "󱕑", "󰍽", "󰍽", "", "", "", "󰍭", "󰃠", "󰃞",
            "󰙡", "", "", "󰙣",
        ];

        let non_nerd_fonts: [&str; 5] = [";", "A", "DEL", "TAB", "1"];

        for icon in &nerd_fonts {
            assert!(is_nerd_font_icon(icon));
        }

        for text in &non_nerd_fonts {
            assert!(!is_nerd_font_icon(text));
        }
    }
}
