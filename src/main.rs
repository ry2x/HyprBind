mod app;
mod cli;
mod config;
mod hyprland;
mod ui;

#[cfg(test)]
mod tests;

use app::KeybindsApp;
use cli::CliAction;
use eframe::egui;
use ui::styling::fonts::setup_custom_fonts;

fn main() -> Result<(), eframe::Error> {
    match cli::parse_args() {
        CliAction::WriteDefaultCss { force } => {
            cli::handle_write_css(force);
            Ok(())
        }
        CliAction::OutputJson => {
            cli::handle_json_output();
            Ok(())
        }
        CliAction::OutputDmenu => {
            cli::handle_dmenu_output();
            Ok(())
        }
        CliAction::RunGui => run_gui(),
    }
}

fn run_gui() -> Result<(), eframe::Error> {
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
            ui::styling::css::apply_default_if_exists(&cc.egui_ctx);
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
