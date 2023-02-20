use egui::{vec2, Ui, Align, Direction};
use crate::grid::MinesweeperGrid;
use crate::sprites::{Sprites, SpriteType};

pub struct MinesweeperApp {
    sprites: Sprites,
    grid: MinesweeperGrid,
}

impl Default for MinesweeperApp {
    fn default() -> Self {
        Self {
            sprites: Sprites::default(),
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
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let top_height = 42.0;

        egui::TopBottomPanel::top("top").exact_height(top_height).show_separator_line(false).show(ctx, |ui| {
            ui.spacing_mut().item_spacing = vec2(2.0, 0.0);

            ui.columns(3, |columns| {
                columns[0].with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                    self.sprites.digits(ui, 123, Direction::LeftToRight, 1.5);
                });

                columns[1].with_layout(egui::Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                    self.sprites.image(ui, SpriteType::FaceSmileyUp, 1.5);
                });

                columns[2].with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    self.sprites.digits(ui, 123, Direction::RightToLeft, 1.5);
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // panel with countdown

            // minesweeper grid
            let clicked_pos = minesweeper_grid(ui, &self.sprites, self.grid.width(), self.grid.height());
            //let clicked_pos = self.grid.ui(ui);

            if let Some((x, y)) = clicked_pos {
                println!("Clicked {x},{y}");

                if x < 3 {
                    self.grid = MinesweeperGrid::new(9, 9);
                }
                else if x > 7 {
                    self.grid = MinesweeperGrid::new(16, 16);
                }
            }
        });

        let window_size = vec2(33.0 * self.grid.width() as f32, 33.0 * self.grid.height() as f32 + top_height);

        frame.set_window_size(window_size);
    }
}

fn minesweeper_grid(ui: &mut Ui, sprites: &Sprites, w: u32, h: u32) -> Option<(u32, u32)> {
    let mut result = None;

    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);

    let mut i = 0;
    ui.vertical(|ui| {
        for y in 0..h {
            ui.horizontal(|ui| {
                for x in 0..w {
                    let btn = sprites.button(ui, SpriteType::BlockEmptyUp, 2.0);
                    let clicked = btn.clicked();

                    if clicked {
                        result = Some((x, y))
                    }

                    i += 1;
                }
            });
        }
    });

    result
}
