use rand::prelude::SliceRandom;
use std::time::{Duration, Instant};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GameConfig {
    width: u32,
    height: u32,
    mines: usize,
}

impl GameConfig {
    pub const BEGINNER: GameConfig = GameConfig {
        width: 9,
        height: 9,
        mines: 10,
    };
    pub const INTERMEDIATE: GameConfig = GameConfig {
        width: 16,
        height: 16,
        mines: 40,
    };
    pub const EXPERT: GameConfig = GameConfig {
        width: 32,
        height: 16,
        mines: 99,
    };

    pub fn new(width: u32, height: u32, mines: usize) -> Self {
        Self {
            width,
            height,
            mines,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn mines(&self) -> usize {
        self.mines
    }

    pub fn total_size(&self) -> usize {
        (self.width * self.height) as usize
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum GameMove {
    NoOp,
    Reveal(u32, u32),
    Flag(u32, u32),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum GameState {
    Reset,
    Playing,
    Completed,
    Dead,
}

impl GameState {
    pub fn game_over(self) -> bool {
        use GameState::*;
        match self {
            Reset | Playing => false,
            Completed | Dead => true,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum GridState {
    // Actual states of grid internally
    Empty,
    Count(u8),
    Mine,

    // Additional states that may be shown to player
    Unrevealed,
    Flagged,
    MineHighlighted,
    MineIncorrect,
}

pub struct MinesweeperGame {
    config: GameConfig,
    state: GameState,
    mines_remaining: i32,
    grid: Vec<GridState>,
    flagged: Vec<bool>,
    revealed: Vec<bool>,
    revealed_count: usize,
    timer: Timer,
}

impl Default for MinesweeperGame {
    fn default() -> Self {
        Self::new(GameConfig::BEGINNER)
    }
}

impl MinesweeperGame {
    pub fn new(config: GameConfig) -> Self {
        let mines = generate_mines(config.mines(), config.total_size());
        MinesweeperGame::with_mines(config, &mines)
    }

    pub fn with_mines(config: GameConfig, mine_positions: &[usize]) -> Self {
        assert!(
            config.mines() <= config.total_size(),
            "Too many mines for grid size"
        );
        assert_eq!(
            config.mines(),
            mine_positions.len(),
            "Explicit mine_positions must match the number of mines for the config"
        );

        let grid = initialize_grid(&config, mine_positions);
        let flagged = vec![false; grid.len()];
        let revealed = vec![false; grid.len()];

        MinesweeperGame {
            config,
            state: GameState::Reset,
            mines_remaining: mine_positions.len() as i32,
            grid,
            flagged,
            revealed,
            revealed_count: 0,
            timer: Timer::default(),
        }
    }

    pub fn config(&self) -> GameConfig {
        self.config
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    pub fn game_over(&self) -> bool {
        self.state.game_over()
    }

    pub fn total_size(&self) -> usize {
        self.config.total_size()
    }

    pub fn width(&self) -> u32 {
        self.config.width()
    }

    pub fn height(&self) -> u32 {
        self.config.height()
    }

    pub fn mines_remaining(&self) -> i32 {
        self.mines_remaining
    }

    pub fn timer_elapsed(&self) -> Duration {
        self.timer.elapsed_duration()
    }

    pub fn revealed_count(&self) -> usize {
        self.revealed_count
    }

    pub fn neighbors(&self, x: u32, y: u32) -> Vec<(u32, u32)> {
        // Naively, we'd like to look at [x-1, x, x+1], but because x/y are u32, we can't actually calculate x-1 when x is 0.
        // So use a little trick to check bounds+1 on [x, x+1, x+2], and then subtract 1 when in bounds.
        let mut result = Vec::new();
        for ny in y..=y + 2 {
            if ny > 0 && ny <= self.height() {
                for nx in x..=x + 2 {
                    if nx > 0 && nx <= self.width() {
                        result.push((nx - 1, ny - 1));
                    }
                }
            }
        }
        result
    }

    pub fn peek_at(&self, x: u32, y: u32, show_actual: bool) -> GridState {
        use GridState::*;

        let i = pos_to_index(x, y, self.width());
        let (state, revealed, flagged) = (self.grid[i], self.revealed[i], self.flagged[i]);
        let game_over = self.game_over();

        if !revealed && !show_actual {
            if flagged {
                if game_over && state != Mine {
                    MineIncorrect
                } else {
                    Flagged
                }
            } else if game_over && state == Mine {
                Mine
            } else {
                Unrevealed
            }
        } else if flagged {
            match state {
                Empty => MineIncorrect,
                Count(_) => MineIncorrect,
                Mine => Mine,
                _ => {
                    panic!("Invalid state in grid");
                }
            }
        } else {
            match state {
                Empty => Empty,
                Count(count) => Count(count),
                Mine => {
                    if revealed {
                        MineHighlighted
                    } else {
                        Mine
                    }
                }
                _ => {
                    panic!("Invalid state in grid");
                }
            }
        }
    }

    pub fn make_move(&mut self, m: GameMove) -> bool {
        use GameMove::*;
        match m {
            NoOp => false,
            Reveal(x, y) => self.reveal(x, y),
            Flag(x, y) => self.toggle_flag(x, y),
        }
    }

    pub fn reveal(&mut self, x: u32, y: u32) -> bool {
        use GridState::*;

        if self.game_over() {
            return false;
        }
        if !self.timer.is_started() {
            self.timer.start();
        }
        self.state = GameState::Playing;

        let i = pos_to_index(x, y, self.width());
        if self.flagged[i] || self.revealed[i] {
            return false;
        }

        self.revealed[i] = true;
        self.revealed_count += 1;

        match self.grid[i] {
            Empty => {
                // Also reveal neighbors
                for (nx, ny) in self.neighbors(x, y) {
                    self.reveal(nx, ny);
                }
            }
            Count(_) => {}
            Mine => {
                self.state = GameState::Dead;
            }
            _ => {
                panic!("Invalid state in grid");
            }
        }

        if !self.game_over() {
            // Check if revealing this completes the game
            if self.revealed_count == self.total_size() - self.config.mines() {
                self.state = GameState::Completed;
                self.timer.end();
                // Flag all mines when game ends successfully
                for j in 0..self.grid.len() {
                    if self.grid[j] == Mine {
                        self.flagged[j] = true;
                    }
                }
            }
        }

        true
    }

    pub fn toggle_flag(&mut self, x: u32, y: u32) -> bool {
        if self.game_over() {
            return false;
        }
        if !self.timer.is_started() {
            self.timer.start();
        }
        self.state = GameState::Playing;

        let i = pos_to_index(x, y, self.width());
        if !self.revealed[i] {
            if self.flagged[i] {
                self.flagged[i] = false;
                self.mines_remaining += 1;
            } else {
                self.flagged[i] = true;
                self.mines_remaining -= 1;
            }
        }
        self.flagged[i]
    }
}

fn pos_to_index(x: u32, y: u32, width: u32) -> usize {
    (x + y * width) as usize
}

fn index_to_pos(i: usize, width: u32) -> (u32, u32) {
    (i as u32 % width, i as u32 / width)
}

fn generate_mines(mines: usize, size: usize) -> Vec<usize> {
    let mut result: Vec<usize> = (0..size).collect();

    // shortcut to handle 100% density
    if mines >= size {
        return result;
    }

    let mut rng = rand::thread_rng();
    result.shuffle(&mut rng);

    result.into_iter().take(mines).collect()
}

fn initialize_grid(config: &GameConfig, mine_positions: &[usize]) -> Vec<GridState> {
    use GridState::*;

    let size = config.total_size();
    let mut grid = vec![GridState::Empty; size];

    for i in mine_positions {
        grid[*i] = Mine;
    }

    let w = config.width();
    let h = config.height();
    for x in 0..w {
        for y in 0..h {
            let i = pos_to_index(x, y, w);
            if grid[i] == Empty {
                let mut count = 0;

                if x > 0 {
                    if grid[i - 1] == Mine {
                        count += 1
                    }; // left
                }
                if x < w - 1 {
                    if grid[i + 1] == Mine {
                        count += 1
                    }; // right
                }

                if y > 0 {
                    let j = pos_to_index(x, y - 1, w);
                    if grid[j] == Mine {
                        count += 1
                    }; // up
                    if x > 0 {
                        if grid[j - 1] == Mine {
                            count += 1
                        }; // up-left
                    }
                    if x < w - 1 {
                        if grid[j + 1] == Mine {
                            count += 1
                        }; // up-right
                    }
                }
                if y < h - 1 {
                    let j = pos_to_index(x, y + 1, w);
                    if grid[j] == Mine {
                        count += 1
                    }; // down
                    if x > 0 {
                        if grid[j - 1] == Mine {
                            count += 1
                        }; // down-left
                    }
                    if x < w - 1 {
                        if grid[j + 1] == Mine {
                            count += 1
                        }; // down-right
                    }
                }

                if count > 0 {
                    grid[i] = Count(count);
                } else {
                    grid[i] = Empty;
                }
            }
        }
    }

    grid
}

struct Timer {
    start_time: Option<Instant>,
    end_time: Option<Instant>,
}

impl Default for Timer {
    fn default() -> Self {
        Timer {
            start_time: None,
            end_time: None,
        }
    }
}

impl Timer {
    fn reset(&mut self) {
        self.start_time = None;
        self.end_time = None;
    }

    fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.end_time = None;
    }

    fn end(&mut self) {
        assert!(self.start_time.is_some(), "Timer end called before start");
        self.end_time = Some(Instant::now());
    }

    fn is_started(&self) -> bool {
        self.start_time.is_some()
    }

    fn is_ended(&self) -> bool {
        self.end_time.is_some()
    }

    fn elapsed_duration(&self) -> Duration {
        match (self.start_time, self.end_time) {
            (Some(start_time), None) => start_time.elapsed(),
            (Some(start_time), Some(end_time)) => end_time.duration_since(start_time),
            (None, _) => Duration::ZERO,
        }
    }
}
