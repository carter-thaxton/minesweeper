use crate::game::{GameConfig, GameState, GridState, MinesweeperGame};
use crate::solver::get_next_move;
use crate::sprites::{SpriteType, Sprites};
use egui::{Align, CentralPanel, Direction, Key, Layout, Panel, Ui, ViewportCommand, vec2};
use std::time::Duration;

#[derive(Default)]
pub struct MinesweeperApp {
    sprites: Sprites,
    game: MinesweeperGame,
}

impl eframe::App for MinesweeperApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        let top_height = 42.0;
        let bottom_height = 42.0;

        // every frame, run solver for one move if S key is pressed
        let run_solver = ui.input(|i| i.key_pressed(Key::S));
        if run_solver {
            let m = get_next_move(&self.game);
            self.game.make_move(m);
        }

        // top panel, with numbers and faces
        Panel::top("top")
            .exact_size(top_height)
            .show_separator_line(false)
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing = vec2(2.0, 0.0);

                ui.columns(3, |columns| {
                    columns[0].with_layout(Layout::left_to_right(Align::Center), |ui| {
                        let mines_remaining =
                            self.game.mines_remaining().max(0).try_into().unwrap();
                        self.sprites
                            .digits(ui, mines_remaining, Direction::LeftToRight, 1.5);
                    });

                    columns[1].with_layout(
                        Layout::centered_and_justified(Direction::LeftToRight),
                        |ui| {
                            let face = sprite_for_game_state(self.game.state());
                            let reset = self.sprites.button(ui, face, 1.5).clicked();
                            let reset = reset || ui.input(|i| i.key_pressed(Key::R));
                            if reset {
                                self.game = MinesweeperGame::new(self.game.config());
                            }
                        },
                    );

                    columns[2].with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let timer_secs = self.game.timer_elapsed().as_secs().try_into().unwrap();
                        self.sprites
                            .digits(ui, timer_secs, Direction::RightToLeft, 1.5);
                    });
                });
            });

        // bottom panel, with options to change game size
        Panel::bottom("bottom")
            .exact_size(bottom_height)
            .show_separator_line(false)
            .show(ui, |ui| {
                let mut config = self.game.config();

                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.radio_value(&mut config, GameConfig::BEGINNER, "Beginner");
                    ui.radio_value(&mut config, GameConfig::INTERMEDIATE, "Intermediate");
                    ui.radio_value(&mut config, GameConfig::EXPERT, "Expert");
                });

                if config != self.game.config() {
                    self.game = MinesweeperGame::new(config);
                }
            });

        // central panel, with minesweeper grid
        CentralPanel::default().show(ui, |ui| {
            let hint = ui.input(|i| i.key_down(Key::H) && i.modifiers.shift_only());
            let clicked_pos = minesweeper_grid(ui, &self.sprites, &self.game, hint);

            if let Some((x, y, right)) = clicked_pos {
                if right {
                    self.game.toggle_flag(x, y);
                } else {
                    self.game.reveal(x, y);
                }
            }
        });

        // resize window to match contents
        let window_size = vec2(
            32. * self.game.width() as f32 + 10.,
            32. * self.game.height() as f32 + 10. + top_height + bottom_height,
        );
        ui.send_viewport_cmd(ViewportCommand::InnerSize(window_size));

        // ensure the timer increments while playing, even if no user interaction
        if self.game.state() == GameState::Playing {
            ui.request_repaint_after(Duration::from_millis(100));
        }
    }
}

/// Draw minesweeper grid.
///
/// Uses sprites to draw each block.
///
/// Return the grid position of a user click, or None.
fn minesweeper_grid(
    ui: &mut Ui,
    sprites: &Sprites,
    game: &MinesweeperGame,
    show_all: bool,
) -> Option<(u32, u32, bool)> {
    let mut result = None;

    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);

    ui.vertical(|ui| {
        for y in 0..game.height() {
            ui.horizontal(|ui| {
                for x in 0..game.width() {
                    let state = game.peek_at(x, y, show_all);
                    let sprite = sprite_for_grid(state);
                    let btn = sprites.button(ui, sprite, 2.0);
                    let clicked = btn.clicked();
                    let right_clicked = btn.secondary_clicked();

                    if clicked || right_clicked {
                        result = Some((x, y, right_clicked))
                    }
                }
            });
        }
    });

    result
}

fn sprite_for_game_state(state: GameState) -> SpriteType {
    match state {
        GameState::Reset | GameState::Playing => SpriteType::FaceSmileyUp,
        GameState::Completed => SpriteType::FaceCool,
        GameState::Dead => SpriteType::FaceXXX,
    }
}

fn sprite_for_grid(state: GridState) -> SpriteType {
    match state {
        GridState::Empty => SpriteType::BlockEmptyDown,
        GridState::Count(count) => SpriteType::block_digit(count.into()),
        GridState::Mine => SpriteType::BlockMine,
        GridState::Unrevealed => SpriteType::BlockEmptyUp,
        GridState::Flagged => SpriteType::BlockFlag,
        GridState::MineHighlighted => SpriteType::BlockMineRed,
        GridState::MineIncorrect => SpriteType::BlockMineX,
    }
}
