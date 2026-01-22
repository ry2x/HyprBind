#[cfg(test)]
mod models_tests {
    use crate::hyprland::{KeyBindEntry, KeyBindings};

    /// Validates dmenu format export with icon mapping
    #[test]
    fn test_to_dmenu() {
        // 1. No modifier, with description
        let entry1 = KeyBindEntry::new(
            String::new(),
            "Return".to_string(),
            "exec kitty".to_string(),
            "Terminal".to_string(),
        );
        // 2. With modifiers, with description
        let entry2 = KeyBindEntry::new(
            "SUPER+SHIFT".to_string(),
            "Q".to_string(),
            "killactive".to_string(),
            "Kill window".to_string(),
        );
        // 3. With modifiers, no description
        let entry3 = KeyBindEntry::new(
            "SUPER+ALT".to_string(),
            "F1".to_string(),
            "exec firefox".to_string(),
            String::new(),
        );
        // 4. With modifiers, no description, no command
        let entry4 = KeyBindEntry::new(
            "CTRL+SHIFT".to_string(),
            "F2".to_string(),
            String::new(),
            String::new(),
        );

        let kb = KeyBindings {
            entries: vec![entry1, entry2, entry3, entry4],
        };

        let dmenu = kb.to_dmenu();
        let lines: Vec<&str> = dmenu.lines().collect();

        // 1. No modifier, icon only
        assert_eq!(lines[0], "󰌑 : Terminal");

        // 2. Modifiers, icons
        assert_eq!(lines[1], " +  󰘶  + Q : Kill window");

        // 3. Modifiers, fallback to key text if not in icon table
        assert_eq!(lines[2], " + ALT + F1 : exec firefox");

        // 4. Modifiers, no description, no command
        assert_eq!(lines[3], "CTRL +  󰘶  + F2 : ");
    }
}
