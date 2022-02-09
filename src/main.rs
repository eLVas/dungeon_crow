mod camera;
mod map;
mod map_builder;

mod prelude {
    pub use crate::camera::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;

    pub use bracket_lib::prelude::*;

    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;

    pub const WORLD_WIDTH: i32 = 80;
    pub const WORLD_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = WORLD_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = WORLD_HEIGHT / 2;
}

use prelude::*;

struct State {
    map: Map,
    camera: Camera,
    debug: bool,
}

impl State {
    fn new(debug: bool) -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng, false);
        Self {
            map: map_builder.map,
            camera: Camera::new(map_builder.player_start),
            debug,
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Clean screen
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();

        // TODO: Execute Systems
        // TODO: Render Draw Buffer
    }
}

fn main() -> BError {
    let width = DISPLAY_WIDTH;
    let height = DISPLAY_HEIGHT;

    let fps_cap = 30.0;

    let tile_width = 32;
    let tile_height = 32;

    let context = BTermBuilder::new()
        .with_title("Dungeon Crow")
        .with_fullscreen(true)
        .with_fps_cap(fps_cap)
        .with_dimensions(width, height)
        .with_tile_dimensions(tile_width, tile_height)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", tile_width, tile_height)
        .with_simple_console(width, height, "dungeonfont.png")
        .with_simple_console_no_bg(width, height, "dungeonfont.png")
        .build()?;

    let state = State::new(false);

    main_loop(context, state)
}
