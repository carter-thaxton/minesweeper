use crate::grid::MinesweeperGrid;

pub struct MinesweeperApp {
    grid: MinesweeperGrid,
}

impl Default for MinesweeperApp {
    fn default() -> Self {
        Self {
            grid: MinesweeperGrid::default(),
        }
    }
}

impl MinesweeperApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for MinesweeperApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.heading("eframe template");
            // ui.hyperlink("https://github.com/emilk/eframe_template");

            let clicked_pos = self.grid.ui(ui);

            match clicked_pos {
                Some((x, y)) => {
                    println!("Clicked {x},{y}");
                }
                None => {}
            }
        });
    }
}
