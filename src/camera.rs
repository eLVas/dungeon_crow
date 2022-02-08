use crate::prelude::*;

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    pub fn new(player_position: Point) -> Self {
        let mut cam = Self {
            left_x: 0,
            right_x: DISPLAY_WIDTH,
            top_y: 0,
            bottom_y: DISPLAY_HEIGHT,
        };
        cam.center_on_player(player_position);
        cam
    }

    #[allow(dead_code)]
    pub fn debug_mode() -> Self {
        Self {
            left_x: 0,
            right_x: WORLD_WIDTH,
            top_y: 0,
            bottom_y: WORLD_HEIGHT,
        }
    }

    pub fn center_on_player(&mut self, player_position: Point) {
        self.left_x = player_position.x - DISPLAY_WIDTH / 2;
        self.right_x = player_position.x + DISPLAY_WIDTH / 2;
        self.top_y = player_position.y - DISPLAY_HEIGHT / 2;
        self.bottom_y = player_position.y + DISPLAY_HEIGHT / 2;
    }
}
