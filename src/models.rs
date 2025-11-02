use serde::{Deserialize, Serialize};

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
    pub fn matches(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();
        self.modifiers.to_lowercase().contains(&query_lower)
            || self.key.to_lowercase().contains(&query_lower)
            || self.command.to_lowercase().contains(&query_lower)
            || self.description.to_lowercase().contains(&query_lower)
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
    pub fn filter(&self, query: &str) -> Vec<&KeyBindEntry> {
        if query.is_empty() {
            self.entries.iter().collect()
        } else {
            self.entries.iter().filter(|e| e.matches(query)).collect()
        }
    }

    /// Export as JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self::new()
    }
}
