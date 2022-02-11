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

    pub fn in_inner_space(&self, point: Point) -> bool {
        let mut inner_space = true;

        for y in point.y - 1..=point.y + 1 {
            for x in point.x - 1..=point.x + 1 {
                if self.in_bounds(Point::new(x, y)) && self.tiles[map_idx(x, y)] == TileType::Floor
                {
                    inner_space = false;
                }
            }
        }

        inner_space
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
}
