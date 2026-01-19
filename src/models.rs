use crate::icons::get_icon;
use serde::{Deserialize, Serialize};

/// Options for searching keybindings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOptions {
    pub keybind: bool,
    pub command: bool,
    pub description: bool,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            keybind: true,
            command: true,
            description: true,
        }
    }
}

/// Keybind entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBindEntry {
    /// Modifier keys (e.g., "SUPER", "SUPER SHIFT")
    pub modifiers: String,
    /// Key name (e.g., "A", "F", "Return")
    pub key: String,
    /// Command (e.g., "exec kitty", "killactive")
    pub command: String,
    /// Description (obtained from bind line comment)
    pub description: String,
}

impl KeyBindEntry {
    pub fn new(modifiers: String, key: String, command: String, description: String) -> Self {
        Self {
            modifiers,
            key,
            command,
            description,
        }
    }

    /// Check if this entry matches the search query
    pub fn matches(&self, query: &str, options: &SearchOptions) -> bool {
        let query_lower = query.to_lowercase();
        let keybind_match = options.keybind
            && (self.modifiers.to_lowercase().contains(&query_lower)
                || self.key.to_lowercase().contains(&query_lower));
        let command_match = options.command && self.command.to_lowercase().contains(&query_lower);
        let description_match =
            options.description && self.description.to_lowercase().contains(&query_lower);

        keybind_match || command_match || description_match
    }
}

/// Structure to hold parsing results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBindings {
    /// All keybind entries
    pub entries: Vec<KeyBindEntry>,
}

impl KeyBindings {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: KeyBindEntry) {
        self.entries.push(entry);
    }

    /// Filter entries by search query
    pub fn filter(&self, query: &str, options: &SearchOptions) -> Vec<&KeyBindEntry> {
        if query.is_empty() {
            self.entries.iter().collect()
        } else {
            self.entries
                .iter()
                .filter(|e| e.matches(query, options))
                .collect()
        }
    }

    /// Export as JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Export as dmenu-compatible format with NERD FONT icons
    pub fn to_dmenu(&self) -> String {
        self.entries
            .iter()
            .map(|entry| {
                let keybind = if entry.modifiers.is_empty() {
                    get_icon(&entry.key)
                } else {
                    let modifiers: Vec<&str> = entry.modifiers.split('+').collect();
                    let modifier_icons: Vec<String> =
                        modifiers.iter().map(|m| get_icon(m)).collect();
                    let key_icon = get_icon(&entry.key);
                    format!("{} + {}", modifier_icons.join(" + "), key_icon)
                };

                let display_text = if !entry.description.is_empty() {
                    entry.description.clone()
                } else {
                    entry.command.clone()
                };

                format!("{} :{}", keybind, display_text)
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_dmenu() {
        // 1. No modifier, with description
        let entry1 = KeyBindEntry::new(
            "".to_string(),
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
            "CTRL+ALT".to_string(),
            "F1".to_string(),
            "exec firefox".to_string(),
            "".to_string(),
        );

        let kb = KeyBindings {
            entries: vec![entry1, entry2, entry3],
        };

        let dmenu = kb.to_dmenu();
        let lines: Vec<&str> = dmenu.lines().collect();
        // 1. No modifier, icon only
        assert!(lines[0].contains("󰌑")); // Return icon
        assert!(lines[0].contains(":Terminal"));
        // 2. Modifiers, icons
        assert!(lines[1].contains("")); // SUPER icon
        assert!(lines[1].contains("󰘶")); // SHIFT icon
        assert!(lines[1].contains(":Kill window"));
        // 3. Modifiers, fallback to key text if not in icon table
        assert!(lines[2].contains("CTRL"));
        assert!(lines[2].contains("ALT"));
        assert!(lines[2].contains("F1"));
        assert!(lines[2].contains(":exec firefox"));
    }
}
