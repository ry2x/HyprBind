use std::process;

pub enum CliAction {
    RunGui,
    WriteDefaultCss { force: bool },
    OutputJson,
    OutputDmenu,
}

pub fn parse_args() -> CliAction {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|a| a == "--write-default-css") {
        return CliAction::WriteDefaultCss { force: false };
    }
    if args.iter().any(|a| a == "--force-write-default-css") {
        return CliAction::WriteDefaultCss { force: true };
    }
    if args.iter().any(|a| a == "--json" || a == "-j") {
        return CliAction::OutputJson;
    }
    if args.iter().any(|a| a == "--dmenu" || a == "-d") {
        return CliAction::OutputDmenu;
    }

    CliAction::RunGui
}

pub fn handle_write_css(force: bool) {
    match crate::ui::styling::css::write_default_css(force) {
        Ok(path) => {
            let msg = if force {
                format!(
                    "Default CSS written (overwritten) to {}",
                    path.to_string_lossy()
                )
            } else {
                format!("Default CSS written to {}", path.to_string_lossy())
            };
            println!("{msg}");
        }
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    }
}

pub fn handle_json_output() {
    match crate::hyprland::fetch_hyprctl_binds() {
        Ok(raw_output) => {
            let kb = crate::hyprland::parse_binds_output(&raw_output);
            match kb.to_json() {
                Ok(s) => println!("{s}"),
                Err(e) => {
                    eprintln!("Failed to serialize JSON: {e}");
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to load keybindings: {e}");
            process::exit(1);
        }
    }
}

pub fn handle_dmenu_output() {
    match crate::hyprland::fetch_hyprctl_binds() {
        Ok(raw_output) => {
            let kb = crate::hyprland::parse_binds_output(&raw_output);
            println!("{}", kb.to_dmenu());
        }
        Err(e) => {
            eprintln!("Failed to load keybindings: {e}");
            process::exit(1);
        }
    }
}
