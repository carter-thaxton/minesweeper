use egui::{pos2, vec2, Direction, Image, ImageButton, ImageSource, Rect, Response, Ui, Vec2};

pub enum SpriteType {
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,

    FaceSmileyUp,
    FaceSmileyDown,
    FaceRuhRoh,
    FaceCool,
    FaceXXX,

    BlockEmptyUp,
    BlockEmptyDown,
    BlockFlag,
    BlockQuestionUp,
    BlockQuestionDown,
    BlockMine,
    BlockMineRed,
    BlockMineX,

    Block1,
    Block2,
    Block3,
    Block4,
    Block5,
    Block6,
    Block7,
    Block8,
}

impl SpriteType {
    pub fn digit(digit: u32) -> Self {
        match digit {
            0 => SpriteType::Digit0,
            1 => SpriteType::Digit1,
            2 => SpriteType::Digit2,
            3 => SpriteType::Digit3,
            4 => SpriteType::Digit4,
            5 => SpriteType::Digit5,
            6 => SpriteType::Digit6,
            7 => SpriteType::Digit7,
            8 => SpriteType::Digit8,
            9 => SpriteType::Digit9,
            _ => {
                panic!("digit out of bounds");
            }
        }
    }

    pub fn block_digit(digit: u32) -> Self {
        match digit {
            0 => SpriteType::BlockEmptyDown,
            1 => SpriteType::Block1,
            2 => SpriteType::Block2,
            3 => SpriteType::Block3,
            4 => SpriteType::Block4,
            5 => SpriteType::Block5,
            6 => SpriteType::Block6,
            7 => SpriteType::Block7,
            8 => SpriteType::Block8,
            _ => {
                panic!("digit out of bounds");
            }
        }
    }
}

impl SpriteType {
    const SPRITESHEET_WIDTH: f32 = 139.;
    const SPRITESHEET_HEIGHT: f32 = 84.;

    #[rustfmt::skip]
    fn pixels(&self) -> (u32, u32, u32, u32) {
        match self {
            SpriteType::Digit1 => (0,   0, 13, 23),
            SpriteType::Digit2 => (14,  0, 13, 23),
            SpriteType::Digit3 => (28,  0, 13, 23),
            SpriteType::Digit4 => (42,  0, 13, 23),
            SpriteType::Digit5 => (56,  0, 13, 23),
            SpriteType::Digit6 => (70,  0, 13, 23),
            SpriteType::Digit7 => (84,  0, 13, 23),
            SpriteType::Digit8 => (98,  0, 13, 23),
            SpriteType::Digit9 => (112, 0, 13, 23),
            SpriteType::Digit0 => (126, 0, 13, 23),

            SpriteType::FaceSmileyUp        => (0,   24, 26, 26),
            SpriteType::FaceSmileyDown      => (27,  24, 26, 26),
            SpriteType::FaceRuhRoh          => (54,  24, 26, 26),
            SpriteType::FaceCool            => (81,  24, 26, 26),
            SpriteType::FaceXXX             => (108, 24, 26, 26),

            SpriteType::BlockEmptyUp        => (0,   51, 16, 16),
            SpriteType::BlockEmptyDown      => (17,  51, 16, 16),
            SpriteType::BlockFlag           => (34,  51, 16, 16),
            SpriteType::BlockQuestionUp     => (51,  51, 16, 16),
            SpriteType::BlockQuestionDown   => (68,  51, 16, 16),
            SpriteType::BlockMine           => (85,  51, 16, 16),
            SpriteType::BlockMineRed        => (102, 51, 16, 16),
            SpriteType::BlockMineX          => (119, 51, 16, 16),

            SpriteType::Block1 => (0,   68, 16, 16),
            SpriteType::Block2 => (17,  68, 16, 16),
            SpriteType::Block3 => (34,  68, 16, 16),
            SpriteType::Block4 => (51,  68, 16, 16),
            SpriteType::Block5 => (68,  68, 16, 16),
            SpriteType::Block6 => (85,  68, 16, 16),
            SpriteType::Block7 => (102, 68, 16, 16),
            SpriteType::Block8 => (119, 68, 16, 16),
        }
    }

    fn rect(&self) -> Rect {
        let (x, y, w, h) = self.pixels();
        [
            pos2(
                x as f32 / Self::SPRITESHEET_WIDTH,
                y as f32 / Self::SPRITESHEET_HEIGHT,
            ),
            pos2(
                (x + w) as f32 / Self::SPRITESHEET_WIDTH,
                (y + h) as f32 / Self::SPRITESHEET_HEIGHT,
            ),
        ]
        .into()
    }

    fn size(&self) -> Vec2 {
        let (_x, _y, w, h) = self.pixels();
        vec2(w as f32, h as f32)
    }
}

pub struct Sprites {
    spritesheet: ImageSource<'static>,
}

impl Default for Sprites {
    fn default() -> Self {
        Sprites {
            spritesheet: egui::include_image!("../assets/minesweeper_spritesheet.png"),
        }
    }
}

impl Sprites {
    pub fn button(&self, ui: &mut Ui, sprite: SpriteType, zoom: f32) -> Response {
        let image = self.image_helper(sprite, zoom);
        let button = ImageButton::new(image);

        ui.spacing_mut().button_padding = vec2(0.0, 0.0);
        ui.add(button)
    }

    pub fn image(&self, ui: &mut Ui, sprite: SpriteType, zoom: f32) -> Response {
        let image = self.image_helper(sprite, zoom);
        ui.add(image)
    }

    fn image_helper(&self, sprite: SpriteType, zoom: f32) -> Image<'_> {
        let size = sprite.size() * zoom;
        let rect = sprite.rect();

        Image::new(self.spritesheet.clone())
            .maintain_aspect_ratio(false)
            .fit_to_exact_size(size)
            .uv(rect)
    }

    pub fn digit(&self, ui: &mut Ui, digit: u32, zoom: f32) {
        let sprite = SpriteType::digit(digit);
        self.image(ui, sprite, zoom);
    }

    pub fn digits(&self, ui: &mut Ui, value: u32, dir: Direction, zoom: f32) {
        if value > 999 {
            panic!("value out of bounds - must be 0-999");
        }

        let d1 = value % 10;
        let value = value / 10;
        let d2 = value % 10;
        let d3 = value / 10;

        match dir {
            Direction::LeftToRight | Direction::TopDown => {
                self.digit(ui, d3, zoom);
                self.digit(ui, d2, zoom);
                self.digit(ui, d1, zoom);
            }
            Direction::RightToLeft | Direction::BottomUp => {
                self.digit(ui, d1, zoom);
                self.digit(ui, d2, zoom);
                self.digit(ui, d3, zoom);
            }
        }
    }
}
