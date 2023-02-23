
use rand::prelude::SliceRandom;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Expert,
    Custom { width: u32, height: u32, mines: usize },
}

impl Difficulty {
    fn size(self) -> (u32, u32) {
        match self {
            Difficulty::Beginner => (9, 9),
            Difficulty::Intermediate => (16, 16),
            Difficulty::Expert => (30, 16),
            Difficulty::Custom { width, height, .. } => (width, height),
        }
    }

    fn mines(self) -> usize {
        match self {
            Difficulty::Beginner => 10,
            Difficulty::Intermediate => 40,
            Difficulty::Expert => 99,
            Difficulty::Custom { mines, .. } => mines,
        }

    }

    fn total_size(self) -> usize {
        let (w, h) = self.size();
        (w * h) as usize
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameState {
    Reset,
    Playing,
    Completed,
    Dead,
}

impl GameState {
    pub fn game_over(self) -> bool {
        match self {
            GameState::Reset | GameState::Playing => false,
            GameState::Completed | GameState::Dead => true,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
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
    difficulty: Difficulty,
    state: GameState,
    mines_remaining: i32,
    grid: Vec<GridState>,
    flagged: Vec<bool>,
    revealed: Vec<bool>,
    revealed_count: usize,
}

impl Default for MinesweeperGame {
    fn default() -> Self {
        Self::new(Difficulty::Beginner)
    }
}

impl MinesweeperGame {
    pub fn new(difficulty: Difficulty) -> Self {
        let mines = generate_mines(difficulty.mines(), difficulty.total_size());
        MinesweeperGame::with_mines(difficulty, &mines)
    }

    pub fn with_mines(difficulty: Difficulty, mine_positions: &[usize]) -> Self {
        assert!(difficulty.mines() <= difficulty.total_size(), "Too many mines for grid size");
        assert_eq!(difficulty.mines(), mine_positions.len(), "Explicit mine_positions must match the number of mines for the difficulty");

        let grid = initialize_grid(difficulty, mine_positions);
        let flagged = vec![false; grid.len()];
        let revealed = vec![false; grid.len()];

        MinesweeperGame {
            difficulty,
            state: GameState::Reset,
            mines_remaining: mine_positions.len() as i32,
            grid,
            flagged,
            revealed,
            revealed_count: 0,
        }
    }

    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    pub fn total_size(&self) -> usize {
        self.difficulty.total_size()
    }

    pub fn size(&self) -> (u32, u32) {
        self.difficulty.size()
    }

    pub fn width(&self) -> u32 {
        self.size().0
    }

    pub fn height(&self) -> u32 {
        self.size().1
    }

    pub fn mines_remaining(&self) -> i32 {
        self.mines_remaining
    }

    pub fn peek_at(&self, x: u32, y: u32, show_actual: bool) -> GridState {
        let i = pos_to_index(x, y, self.width());
        let (state, revealed, flagged) = (self.grid[i], self.revealed[i], self.flagged[i]);
        let game_over = self.state.game_over();

        if !revealed && !show_actual {
            if flagged {
                if game_over && state != GridState::Mine {
                    GridState::MineIncorrect
                } else {
                    GridState::Flagged
                }
            } else if game_over && state == GridState::Mine {
                GridState::Mine
            } else {
                GridState::Unrevealed
            }
        } else if flagged {
            match state {
                GridState::Empty => GridState::MineIncorrect,
                GridState::Count(_) => GridState::MineIncorrect,
                GridState::Mine => GridState::Mine,
                _ => { panic!("Invalid state in grid"); },
            }
        } else {
            match state {
                GridState::Empty => GridState::Empty,
                GridState::Count(count) => GridState::Count(count),
                GridState::Mine => { if revealed { GridState::MineHighlighted } else { GridState::Mine } },
                _ => { panic!("Invalid state in grid"); },
            }
        }
    }

    pub fn reveal(&mut self, x: u32, y: u32) -> bool {
        if self.state.game_over() { return false; }
        self.state = GameState::Playing;

        let i = pos_to_index(x, y, self.width());
        if self.flagged[i] || self.revealed[i] { return false; }

        self.revealed[i] = true;
        self.revealed_count += 1;

        match self.grid[i] {
            GridState::Empty => {
                // Also reveal neighbors
                // Naively, we'd like to look at [x-1, x, x+1], but because x/y are u32, we can't actually calculate x-1 when x is 0.
                // So use a little trick to check bounds+1 on [x, x+1, x+2], and then subtract 1 when in bounds.
                for y2 in y..=y+2 {
                    if y2 > 0 && y2 <= self.height() {
                        for x2 in x..=x+2 {
                            if x2 > 0 && x2 <= self.width() {
                                self.reveal(x2-1, y2-1);
                            }
                        }
                    }
                }
            }
            GridState::Count(_) => {},
            GridState::Mine => {
                self.state = GameState::Dead;
            },
            _ => { panic!("Invalid state in grid"); },
        }

        if !self.state.game_over() {
            // Check if revealing this completes the game
            if self.revealed_count == self.total_size() - self.difficulty.mines() {
                self.state = GameState::Completed;
                // Flag all mines when game ends successfully
                for j in 0..self.grid.len() {
                    if self.grid[j] == GridState::Mine {
                        self.flagged[j] = true;
                    }
                }
            }
        }

        true
    }

    pub fn toggle_flag(&mut self, x: u32, y: u32) -> bool {
        if self.state.game_over() { return false; }
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
    if mines >= size { return result }

    let mut rng = rand::thread_rng();
    result.shuffle(&mut rng);

    result.into_iter().take(mines).collect()
}

fn initialize_grid(difficulty: Difficulty, mine_positions: &[usize]) -> Vec<GridState> {
    let size = difficulty.total_size();
    let mut grid = vec![GridState::Empty; size];

    for i in mine_positions {
        grid[*i] = GridState::Mine;
    }

    let (w, h) = difficulty.size();
    for x in 0..w {
        for y in 0..h {
            let i = pos_to_index(x, y, w);
            if grid[i] == GridState::Empty {
                let mut count = 0;

                if x > 0 {
                    if grid[i-1] == GridState::Mine { count += 1 };         // left
                }
                if x < w-1 {
                    if grid[i+1] == GridState::Mine { count += 1 };         // right
                }

                if y > 0 {
                    let j = pos_to_index(x, y-1, w);
                    if grid[j] == GridState::Mine { count += 1 };           // up
                    if x > 0 {
                        if grid[j-1] == GridState::Mine { count += 1 };     // up-left
                    }
                    if x < w-1 {
                        if grid[j+1] == GridState::Mine { count += 1 };     // up-right
                    }
                }
                if y < h-1 {
                    let j = pos_to_index(x, y+1, w);
                    if grid[j] == GridState::Mine { count += 1 };           // down
                    if x > 0 {
                        if grid[j-1] == GridState::Mine { count += 1 };     // down-left
                    }
                    if x < w-1 {
                        if grid[j+1] == GridState::Mine { count += 1 };     // down-right
                    }
                }

                if count > 0 {
                    grid[i] = GridState::Count(count);
                } else {
                    grid[i] = GridState::Empty;
                }
            }
        }
    }

    grid
}
