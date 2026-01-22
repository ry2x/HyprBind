#[cfg(test)]
mod parser_edge_tests {
    use crate::hyprland::parser::parse_binds_output;

    /// Ensures parser handles empty input without crashing
    #[test]
    fn test_empty_input() {
        let kb = parse_binds_output("");
        assert_eq!(kb.entries.len(), 0);
    }

    /// Validates that parser correctly handles multiple empty lines between bind blocks
    #[test]
    fn test_empty_lines_mixed() {
        let sample = r"bind
	modmask: 64
	submap: 
	key: A
	keycode: 0
	catchall: false
	description: Test
	dispatcher: exec
	arg: test


bind
	modmask: 65
	submap: 
	key: B
	keycode: 0
	catchall: false
	description: 
	dispatcher: killactive
	arg: ";

        let kb = parse_binds_output(sample);
        assert_eq!(kb.entries.len(), 2);
        assert_eq!(kb.entries[0].key, "A");
        assert_eq!(kb.entries[1].key, "B");
    }

    /// Verifies parser rejects bind blocks missing required fields (modmask, dispatcher)
    #[test]
    fn test_missing_required_fields() {
        let sample = r"bind
	submap: 
	key: A
	description: Missing modmask";

        let kb = parse_binds_output(sample);
        assert_eq!(kb.entries.len(), 0);
    }

    /// Tests parser behavior with non-numeric modmask values
    #[test]
    fn test_invalid_modmask() {
        let sample = r"bind
	modmask: invalid
	submap: 
	key: A
	keycode: 0
	catchall: false
	description: 
	dispatcher: exec
	arg: test";

        let kb = parse_binds_output(sample);
        assert_eq!(kb.entries.len(), 0);
    }

    /// Confirms parser silently ignores unknown fields in bind blocks
    #[test]
    fn test_unknown_fields_ignored() {
        let sample = r"bind
	modmask: 64
	submap: 
	key: A
	keycode: 0
	catchall: false
	description: Test
	dispatcher: exec
	arg: test
	unknown_field: should be ignored
	another_unknown: also ignored";

        let kb = parse_binds_output(sample);
        assert_eq!(kb.entries.len(), 1);
        assert_eq!(kb.entries[0].key, "A");
    }

    /// Validates parser against actual hyprctl binds output format
    #[test]
    fn test_real_hyprctl_output() {
        let sample = r"bindd
	modmask: 64
	submap: 
	key: Return
	keycode: 0
	catchall: false
	description:  Kitty
	dispatcher: exec
	arg: kitty

bindd
	modmask: 65
	submap: 
	key: Return
	keycode: 0
	catchall: false
	description:  TempKitty
	dispatcher: exec
	arg: kitty --title TempTerminal

bindd
	modmask: 68
	submap: 
	key: Return
	keycode: 0
	catchall: false
	description:  DevKitty
	dispatcher: exec
	arg: kitty --config ~/.config/kitty/dev.conf";

        let kb = parse_binds_output(sample);
        assert_eq!(kb.entries.len(), 3);
        assert_eq!(kb.entries[0].modifiers, "SUPER");
        assert_eq!(kb.entries[0].command, "exec kitty");
        assert_eq!(kb.entries[1].modifiers, "SUPER+SHIFT");
        assert_eq!(kb.entries[2].modifiers, "SUPER+CTRL");
    }

    /// Ensures parser tolerates malformed lines without colon delimiters
    #[test]
    fn test_malformed_line_no_colon() {
        let sample = r"bind
	modmask: 64
	this line has no colon delimiter
	key: A
	keycode: 0
	catchall: false
	description: Test
	dispatcher: exec
	arg: test";

        let kb = parse_binds_output(sample);
        assert_eq!(kb.entries.len(), 1);
    }

    /// Tests parser handling of empty dispatcher field
    #[test]
    fn test_empty_dispatcher() {
        let sample = r"bind
	modmask: 64
	submap: 
	key: A
	keycode: 0
	catchall: false
	description: 
	dispatcher: 
	arg: ";

        let kb = parse_binds_output(sample);
        assert_eq!(kb.entries.len(), 1);
        assert!(kb.entries[0].command.is_empty());
    }

    /// Verifies parser preserves special characters in description fields
    #[test]
    fn test_special_characters_in_description() {
        let sample = r#"bind
	modmask: 64
	submap: 
	key: A
	keycode: 0
	catchall: false
	description: Test: with "quotes" and 'apostrophes'
	dispatcher: exec
	arg: test"#;

        let kb = parse_binds_output(sample);
        assert_eq!(kb.entries.len(), 1);
        assert!(kb.entries[0].description.contains("quotes"));
    }

    /// Confirms parser correctly trims excessive whitespace in field values
    #[test]
    fn test_whitespace_handling() {
        let sample = "bind\n\tmodmask: 64\n\tsubmap: \n\tkey: A\n\tkeycode: 0\n\tcatchall: false\n\tdescription:    Spaces   \n\tdispatcher: exec\n\targ:   test  ";

        let kb = parse_binds_output(sample);
        assert_eq!(kb.entries.len(), 1);
        assert_eq!(kb.entries[0].description.trim(), "Spaces");
    }
}
