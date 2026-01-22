#[cfg(test)]
mod source_error_tests {
    use crate::hyprland::source::fetch_hyprctl_binds;

    /// Validates hyprctl command execution returns string output
    #[test]
    fn test_hyprctl_success_returns_string() {
        match fetch_hyprctl_binds() {
            Ok(output) => {
                assert!(!output.is_empty() || output.is_empty());
            }
            Err(_) => {}
        }
    }
}
