mod camera;
mod components;
mod map;
mod map_builder;
mod score_counter;
mod spawner;
mod systems;

mod prelude {
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::score_counter::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;

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
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng, false);
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(ScoreCounter::new());

        spawn_player(&mut ecs, map_builder.player_start);

        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| {
                spawn_monster(&mut ecs, &mut rng, pos);
                spawn_treasure(&mut ecs, &mut rng, pos - Point::new(1, 0));
            });

        Self {
            ecs,
            resources,
            systems: build_scheduler(),
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

        // Exit when Esc is pressed
        if let Some(VirtualKeyCode::Escape) = ctx.key {
            ctx.quitting = true;
        }

        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);
        render_draw_buffer(ctx).expect("Render error");
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
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(width, height, "dungeonfont.png")
        .with_simple_console_no_bg(width, height, "dungeonfont.png")
        .with_simple_console_no_bg(width, height, "terminal8x8.png")
        .build()?;

    let state = State::new();

    main_loop(context, state)
}
