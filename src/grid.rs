
pub struct MinesweeperGrid {
    grid_width: u32,
    grid_height: u32,
}

impl Default for MinesweeperGrid {
    fn default() -> Self {
        MinesweeperGrid::new(9, 9)
    }
}

impl MinesweeperGrid {
    pub fn new(width: u32, height: u32) -> Self {
        MinesweeperGrid {
            grid_width: width,
            grid_height: height,
        }
    }

    pub fn width(&self) -> u32 {
        self.grid_width
    }

    pub fn height(&self) -> u32 {
        self.grid_height
    }
}
