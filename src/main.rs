mod map;
mod map_builder;
mod player;

mod prelude {
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
    pub use bracket_lib::prelude::*;

    pub const WORLD_WIDTH: i32 = 80;
    pub const WORLD_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = WORLD_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = WORLD_HEIGHT / 2;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
    debug: bool,
}

impl State {
    fn new(debug: bool) -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng, false);
        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
            debug,
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);

        if self.debug {
            let mp = ctx.mouse_point();

            ctx.print(
                1,
                1,
                format!("{}x{}: {}", mp.x, mp.y, self.map.traversable(mp)),
            );

            if ctx.left_click {
                self.map.tiles[map_idx_point(mp)] = TileType::Wall;
            }
        }

        if let Some(VirtualKeyCode::Escape) = ctx.key {
            ctx.quitting = true
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crow")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;

    main_loop(context, State::new(false))
}
