
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

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GridState {
    Empty,
    Mine,
    Count(u32),
}

pub struct MinesweeperGame {
    difficulty: Difficulty,
    state: GameState,
    mines_remaining: i32,
    grid: Vec<GridState>,
    revealed: Vec<bool>,
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
        let grid = initialize_grid(difficulty, mine_positions);
        let revealed = vec![false; grid.len()];

        MinesweeperGame {
            difficulty,
            state: GameState::Reset,
            mines_remaining: mine_positions.len() as i32,
            grid,
            revealed,
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

    pub fn state_at(&self, x: u32, y: u32) -> GridState {
        let i = pos_to_index(x, y, self.width());
        self.grid[i]
    }

    pub fn revealed_at(&self, x: u32, y: u32) -> bool {
        let i = pos_to_index(x, y, self.width());
        self.revealed[i]
    }

    pub fn state_and_revealed_at(&self, x: u32, y: u32) -> (GridState, bool) {
        let i = pos_to_index(x, y, self.width());
        (self.grid[i], self.revealed[i])
    }

    pub fn reveal(&mut self, x: u32, y: u32) -> bool {
        let _i = pos_to_index(x, y, self.width());
        false
    }

    pub fn flag(&mut self, x: u32, y: u32) -> bool {
        let _i = pos_to_index(x, y, self.width());
        false
    }

    pub fn unflag(&mut self, x: u32, y: u32) -> bool {
        let _i = pos_to_index(x, y, self.width());
        false
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
