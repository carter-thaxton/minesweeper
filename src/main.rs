#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(dead_code)]

mod app;
mod game;
mod solver;
mod sprites;

fn main() -> eframe::Result<()> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    //tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions { resizable: false, ..Default::default() };

    eframe::run_native(
        "Minesweeper",
        native_options,
        Box::new(|cc| Box::new(app::MinesweeperApp::new(cc))),
    )
}
