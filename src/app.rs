use egui::{vec2, Ui, Align, Direction};
use crate::game::{Difficulty, GridState, MinesweeperGame};
use crate::sprites::{Sprites, SpriteType};

pub struct MinesweeperApp {
    sprites: Sprites,
    game: MinesweeperGame,
}

impl Default for MinesweeperApp {
    fn default() -> Self {
        Self {
            sprites: Sprites::default(),
            game: MinesweeperGame::default(),
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
        let bottom_height = 42.0;

        // top panel, with numbers and faces
        egui::TopBottomPanel::top("top").exact_height(top_height).show_separator_line(false).show(ctx, |ui| {
            ui.spacing_mut().item_spacing = vec2(2.0, 0.0);

            ui.columns(3, |columns| {
                columns[0].with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                    self.sprites.digits(ui, 123, Direction::LeftToRight, 1.5);
                });

                columns[1].with_layout(egui::Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                    let reset = self.sprites.button(ui, SpriteType::FaceSmileyUp, 1.5).clicked();
                    if reset {
                        self.game = MinesweeperGame::new(self.game.difficulty());
                    }
                });

                columns[2].with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    self.sprites.digits(ui, 123, Direction::RightToLeft, 1.5);
                });
            });
        });

        // central panel, with minesweeper grid
        egui::CentralPanel::default().show(ctx, |ui| {
            let clicked_pos = minesweeper_grid(ui, &self.sprites, &self.game);

            if let Some((x, y)) = clicked_pos {
                println!("Clicked {x},{y}");
            }
        });

        // bottom panel, with options to change game size
        egui::TopBottomPanel::bottom("bottom").exact_height(bottom_height).show_separator_line(false).show(ctx, |ui| {
            let mut difficulty = self.game.difficulty();

            ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                ui.horizontal(|ui| {
                    ui.radio_value(&mut difficulty, Difficulty::Beginner, "Beginner");
                    ui.radio_value(&mut difficulty, Difficulty::Intermediate, "Intermediate");
                    ui.radio_value(&mut difficulty, Difficulty::Expert, "Expert");
                });
            });

            if difficulty != self.game.difficulty() {
                self.game = MinesweeperGame::new(difficulty);
            }
        });

        // resize window to match contents
        let window_size = vec2(32. * self.game.width() as f32 + 10., 32. * self.game.height() as f32  + 10. + top_height + bottom_height);
        frame.set_window_size(window_size);
    }
}

/// Draw minesweeper grid.
///
/// Uses sprites to draw each block.
///
/// Return the grid position of a user click, or None.
fn minesweeper_grid(ui: &mut Ui, sprites: &Sprites, game: &MinesweeperGame) -> Option<(u32, u32)> {
    let mut result = None;

    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);

    let mut i = 0;
    ui.vertical(|ui| {
        for y in 0..game.height() {
            ui.horizontal(|ui| {
                for x in 0..game.width() {
                    let (state, revealed) = game.state_and_revealed_at(x, y);
                    let sprite = sprite_for_state(state, revealed || true);
                    let btn = sprites.button(ui, sprite, 2.0);
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

fn sprite_for_state(state: GridState, revealed: bool) -> SpriteType {
    if !revealed {
        SpriteType::BlockEmptyUp
    } else {
        match state {
            GridState::Empty => SpriteType::BlockEmptyDown,
            GridState::Mine => SpriteType::BlockBomb,
            GridState::Count(count) => SpriteType::block_digit(count),
        }
    }
}
