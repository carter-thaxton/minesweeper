use egui::{pos2, vec2, ImageButton, Rect, Ui, Vec2};
use egui_extras::RetainedImage;

pub struct MinesweeperGrid {
    grid_width: u32,
    grid_height: u32,
    image_size: Vec2,
    texture_atlas: RetainedImage,
}

impl Default for MinesweeperGrid {
    fn default() -> Self {
        MinesweeperGrid::new(30, 20)
    }
}

impl MinesweeperGrid {
    pub fn new(width: u32, height: u32) -> Self {
        MinesweeperGrid {
            grid_width: width,
            grid_height: height,

            image_size: vec2(32.0, 32.0),

            texture_atlas: RetainedImage::from_image_bytes(
                "minesweeper_texture_atlas.png",
                include_bytes!("../assets/minesweeper_texture_atlas.png"),
            )
            .unwrap(),
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) -> Option<(u32, u32)> {
        let texture_id = self.texture_atlas.texture_id(ui.ctx());

        let mut result = None;

        ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
        ui.spacing_mut().button_padding = vec2(-0.5, 0.0);

        let mut i = 0;
        ui.vertical(|ui| {
            for y in 0..self.grid_height {
                ui.horizontal(|ui| {
                    for x in 0..self.grid_width {
                        let rect = rect_for_index(i);
                        let btn = ImageButton::new(texture_id, self.image_size).uv(rect);

                        let clicked = ui.add(btn).clicked();
                        if clicked {
                            result = Some((x, y))
                        }

                        i += 1;
                        if i >= ATLAS_SIZE {
                            i = 0;
                        }
                    }
                });
            }
        });

        result
    }
}

const ATLAS_WIDTH: u32 = 4;
const ATLAS_HEIGHT: u32 = 4;
const ATLAS_SIZE: u32 = 14;

fn rect_for_index(i: u32) -> Rect {
    if i >= ATLAS_SIZE { panic!{"index out of bounds for texture atlas: {i}"}; }

    let x = (i % ATLAS_WIDTH) as f32;
    let y = (i / ATLAS_HEIGHT) as f32;
    let w = 1.0 / ATLAS_WIDTH as f32;
    let h = 1.0 / ATLAS_HEIGHT as f32;

    let min = pos2(x * w, y * h);
    let max = pos2((x + 1.0) * w, (y + 1.0) * h);
    [min, max].into()
}
