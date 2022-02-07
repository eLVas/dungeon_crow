mod map;
mod player;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::player::*;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        Self {
            map: Map::new(),
            player: Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);

        let mp = ctx.mouse_point();

        ctx.print(
            1,
            1,
            format!("{}x{}: {}", mp.x, mp.y, self.map.traversable(mp)),
        );

        if ctx.left_click {
            self.map.tiles[map_idx_point(mp)] = TileType::Wall;
        }

        if let Some(VirtualKeyCode::Escape) = ctx.key {
            ctx.quitting = true
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crow")
        //.with_fullscreen(true)
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())
}
