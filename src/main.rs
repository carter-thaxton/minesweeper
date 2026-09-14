#![warn(clippy::all)]
#![allow(clippy::collapsible_if, dead_code)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod game;
mod solver;
mod sprites;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "Minesweeper",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(app::MinesweeperApp::default()))
        }),
    )
}
