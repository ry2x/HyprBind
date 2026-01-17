mod app;
mod config;
mod css;
mod font;
mod icons;
mod models;
mod parser;
mod ui;

use app::KeybindsApp;
use eframe::egui;
use font::setup_custom_fonts;

fn main() -> Result<(), eframe::Error> {
    // JSON output mode: `--json` or `-j`
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--write-default-css") {
        match css::write_default_css(false) {
            Ok(path) => {
                println!("Default CSS written to {}", path.to_string_lossy());
                return Ok(());
            }
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
    if args.iter().any(|a| a == "--force-write-default-css") {
        match css::write_default_css(true) {
            Ok(path) => {
                println!(
                    "Default CSS written (overwritten) to {}",
                    path.to_string_lossy()
                );
                return Ok(());
            }
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
    if args.iter().any(|a| a == "--json" || a == "-j") {
        match parser::parse_hyprctl_binds() {
            Ok(kb) => match kb.to_json() {
                Ok(s) => {
                    println!("{}", s);
                    return Ok(());
                }
                Err(e) => {
                    eprintln!("Failed to serialize JSON: {}", e);
                    std::process::exit(1);
                }
            },
            Err(e) => {
                eprintln!("Failed to load keybindings: {}", e);
                std::process::exit(1);
            }
        }
    }

    let icon_data = load_icon();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_title("HyprBind")
            .with_icon(icon_data),
        ..Default::default()
    };

    eframe::run_native(
        "HyprBind",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            css::apply_default_if_exists(&cc.egui_ctx);
            Ok(Box::new(KeybindsApp::new()))
        }),
    )
}

fn load_icon() -> egui::IconData {
    let icon_bytes = include_bytes!("../assets/logo_hyprbind.png");
    let image = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon")
        .into_rgba8();
    let (width, height) = image.dimensions();

    egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}
