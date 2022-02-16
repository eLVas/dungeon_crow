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

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;

        if self.in_bounds(destination) && self.traversable(destination) {
            Some(self.point2d_to_index(destination))
        } else {
            None
        }
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        for y in -1..=1 {
            for x in -1..=1 {
                if let Some(idx) = self.valid_exit(location, Point::new(x, y)) {
                    exits.push((idx, 1.0))
                }
            }
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(WORLD_WIDTH, WORLD_HEIGHT)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        self.in_bounds(pos)
    }
}
