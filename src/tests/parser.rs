#[cfg(test)]
mod parser_tests {
    use crate::hyprland::parser::parse_binds_output;

    /// Tests modmask bitmask to human-readable string conversion
    #[test]
    fn test_modmask_conversion() {
        let sample = r"bind
	modmask: 64
	submap: 
	key: A
	keycode: 0
	catchall: false
	description: 
	dispatcher: exec
	arg: echo super";

        let kb = parse_binds_output(sample);
        assert_eq!(kb.entries.len(), 1);
        assert_eq!(kb.entries[0].modifiers, "SUPER");
    }

    /// Validates parsing of binds with multiple modifier keys
    #[test]
    fn test_multiple_modifiers() {
        let sample = r"bind
	modmask: 65
	submap: 
	key: Q
	keycode: 0
	catchall: false
	description: Kill window
	dispatcher: killactive
	arg: ";

        let kb = parse_binds_output(sample);
        assert_eq!(kb.entries.len(), 1);
        assert_eq!(kb.entries[0].modifiers, "SUPER+SHIFT");
        assert_eq!(kb.entries[0].description, "Kill window");
    }

    /// Tests complete parsing of a single bind block with all fields
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

        let kb = parse_binds_output(block);
        assert_eq!(kb.entries.len(), 1);
        assert_eq!(kb.entries[0].modifiers, "SUPER");
        assert_eq!(kb.entries[0].key, "Return");
        assert_eq!(kb.entries[0].command, "exec kitty");
        assert_eq!(kb.entries[0].description, "Terminal");
    }
}
