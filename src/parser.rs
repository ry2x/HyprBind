use crate::models::{KeyBindEntry, KeyBindings};
use std::collections::HashMap;
use std::io;
use std::process::Command;

/// Parse the output of hyprctl binds
pub fn parse_hyprctl_binds() -> io::Result<KeyBindings> {
    let output = Command::new("hyprctl").arg("binds").output()?;

    if !output.status.success() {
        return Err(io::Error::other(
            "hyprctl binds command failed",
        ));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let keybindings = parse_binds_output(&output_str);

    Ok(keybindings)
}

/// Parse the output text from hyprctl binds
fn parse_binds_output(output: &str) -> KeyBindings {
    let mut keybindings = KeyBindings::new();
    let blocks: Vec<&str> = output.split("\n\n").collect();

    for block in blocks {
        if block.trim().is_empty() {
            continue;
        }

        if let Some(entry) = parse_bind_block(block) {
            keybindings.add_entry(entry);
        }
    }

    keybindings
}

/// Parse a single bind block
fn parse_bind_block(block: &str) -> Option<KeyBindEntry> {
    let mut fields = HashMap::new();

    for line in block.lines() {
        let line = line.trim();
        if let Some(colon_pos) = line.find(':') {
            let key = line[..colon_pos].trim();
            let value = line[colon_pos + 1..].trim();
            fields.insert(key, value);
        }
    }

    let modmask = fields.get("modmask")?.parse::<u32>().ok()?;
    let key = fields.get("key")?.to_string();
    let dispatcher = fields.get("dispatcher")?.to_string();
    let arg = fields.get("arg").unwrap_or(&"").to_string();
    let description = fields.get("description").unwrap_or(&"").to_string();

    let modifiers = modmask_to_string(modmask);
    let command = if arg.is_empty() {
        dispatcher
    } else {
        format!("{dispatcher} {arg}")
    };

    Some(KeyBindEntry::new(modifiers, key, command, description))
}

/// Convert modmask (bitmask) to human-readable string
fn modmask_to_string(modmask: u32) -> String {
    let mut mods = Vec::new();

    if modmask & 0x40 != 0 {
        mods.push("SUPER");
    }
    if modmask & 0x08 != 0 {
        mods.push("ALT");
    }
    if modmask & 0x04 != 0 {
        mods.push("CTRL");
    }
    if modmask & 0x01 != 0 {
        mods.push("SHIFT");
    }

    if mods.is_empty() {
        String::new()
    } else {
        mods.join("+")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modmask_to_string() {
        assert_eq!(modmask_to_string(64), "SUPER");
        assert_eq!(modmask_to_string(65), "SUPER+SHIFT");
        assert_eq!(modmask_to_string(72), "SUPER+ALT");
        assert_eq!(modmask_to_string(0), "");
    }

    #[test]
    fn test_parse_bind_block() {
        let block = r"bind
	modmask: 64
	submap: 
	key: Return
	keycode: 0
	catchall: false
	description: Terminal
	dispatcher: exec
	arg: kitty";

        let entry = parse_bind_block(block).expect("Failed to parse test bind block");
        assert_eq!(entry.modifiers, "SUPER");
        assert_eq!(entry.key, "Return");
        assert_eq!(entry.command, "exec kitty");
        assert_eq!(entry.description, "Terminal");
    }
}
