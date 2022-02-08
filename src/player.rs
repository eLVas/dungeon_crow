use crate::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map, cam: &mut Camera) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                _ => Point::zero(),
            };

            let new_position = self.position + delta;
            if map.traversable(new_position) {
                self.position = new_position;
                cam.center_on_player(new_position);
            }
        }
    }

    pub fn render(&self, ctx: &mut BTerm, cam: &Camera) {
        // Use entities layer for rendering
        ctx.set_active_console(1);

        ctx.set(
            self.position.x - cam.left_x,
            self.position.y - cam.top_y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }
}
