#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod grid;

fn main() -> eframe::Result<()> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    //tracing_subscriber::fmt::init();

    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some([1000.0, 800.0].into());

    eframe::run_native(
        "Minesweeper",
        native_options,
        Box::new(|cc| Box::new(app::MinesweeperApp::new(cc))),
    )
}
