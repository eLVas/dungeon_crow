use crate::prelude::*;
const NUM_TILES: usize = (WORLD_WIDTH * WORLD_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * WORLD_WIDTH) + x) as usize
}

pub fn map_idx_point(point: Point) -> usize {
    map_idx(point.x, point.y)
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < WORLD_WIDTH && point.y >= 0 && point.y < WORLD_HEIGHT
    }

    pub fn traversable(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx_point(point)] == TileType::Floor
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx_point(point))
        }
    }

    pub fn render(&self, ctx: &mut BTerm, cam: &Camera) {
        // Use map layer for rendering
        ctx.set_active_console(0);

        for y in cam.top_y..cam.bottom_y {
            for x in cam.left_x..cam.right_x {
                if let Some(idx) = self.try_idx(Point::new(x, y)) {
                    let x_display = x - cam.left_x;
                    let y_display = y - cam.top_y;

                    match self.tiles[idx] {
                        TileType::Floor => {
                            ctx.set(x_display, y_display, GRAY, BLACK, to_cp437('.'));
                        }
                        TileType::Wall => {
                            ctx.set(x_display, y_display, DARK_GREY, BLACK, to_cp437('#'));
                        }
                    }
                }
            }
        }
    }
}
