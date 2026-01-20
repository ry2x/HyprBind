use clap::Parser;
use std::process;

/// A GUI to display Hyprland keybindings
// Allow this to avoid make another struct
#[allow(clippy::struct_excessive_bools)]
#[derive(Parser)]
#[command(name = "HyprBind")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Write default CSS theme file
    #[arg(long)]
    pub write_default_css: bool,

    /// Overwrite existing CSS file (use with --write-default-css)
    #[arg(long, requires = "write_default_css")]
    pub force: bool,

    /// Output keybindings as JSON
    #[arg(short, long)]
    pub json: bool,

    /// Output keybindings in dmenu-compatible format
    #[arg(short, long)]
    pub dmenu: bool,
}

pub enum CliAction {
    RunGui,
    WriteDefaultCss { force: bool },
    OutputJson,
    OutputDmenu,
}

pub fn parse_args() -> CliAction {
    let cli = Cli::parse();

    if cli.write_default_css {
        return CliAction::WriteDefaultCss { force: cli.force };
    }
    if cli.json {
        return CliAction::OutputJson;
    }
    if cli.dmenu {
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
