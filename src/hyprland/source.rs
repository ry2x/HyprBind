use std::io;
use std::process::Command;

/// Fetch raw output from hyprctl binds command
pub fn fetch_hyprctl_binds() -> io::Result<String> {
    let output = Command::new("hyprctl").arg("binds").output()?;

    if !output.status.success() {
        return Err(io::Error::other("hyprctl binds command failed"));
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}
