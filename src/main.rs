mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;

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
use std::collections::HashSet;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
    common_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        spawn_player(&mut ecs, map_builder.player_start);
        // spawn_amulet_of_yala(&mut ecs, map_builder.amulet_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;

        spawn_level(&mut ecs, &mut rng, 0, &map_builder.monster_spawns);

        resources.insert(rng);
        resources.insert(TurnState::AwaitingInput);
        resources.insert(map_builder.map);
        resources.insert(map_builder.theme);
        resources.insert(Camera::new(map_builder.player_start));

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
            common_systems: build_common_systems(),
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();

        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        spawn_player(&mut self.ecs, map_builder.player_start);
        // spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;

        spawn_level(&mut self.ecs, &mut rng, 0, &map_builder.monster_spawns);

        self.resources.insert(rng);
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.map);
        self.resources.insert(map_builder.theme);
        self.resources.insert(Camera::new(map_builder.player_start));
    }

    fn advance_level(&mut self) {
        let player_entity = *<Entity>::query()
            .filter(component::<Player>())
            .iter(&mut self.ecs)
            .nth(0)
            .unwrap();

        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);

        <(Entity, &Carried)>::query()
            .iter(&self.ecs)
            .filter(|(_, carry)| carry.0 == player_entity)
            .map(|(e, _)| *e)
            .for_each(|e| {
                entities_to_keep.insert(e);
            });

        let mut cb = CommandBuffer::new(&mut self.ecs);
        for e in Entity::query().iter(&self.ecs) {
            if !entities_to_keep.contains(e) {
                cb.remove(*e);
            }
        }

        cb.flush(&mut self.ecs);

        <&mut FieldOfView>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|fov| fov.is_dirty = true);

        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        let mut map_level = 0;
        <(&mut Player, &mut Point)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|(player, pos)| {
                player.map_level += 1;
                map_level = player.map_level;
                pos.x = map_builder.player_start.x;
                pos.y = map_builder.player_start.y;
            });

        if map_level == 2 {
            spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }

        spawn_level(
            &mut self.ecs,
            &mut rng,
            map_level as usize,
            &map_builder.monster_spawns,
        );

        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
        ctx.print_color_centered(
            18,
            WHITE,
            BLACK,
            "Slain by a monster, your hero's journey has come to a premature end.",
        );
        ctx.print_color_centered(
            19,
            WHITE,
            BLACK,
            "The Amulet of Yala remains unclaimed, and your home town is not saved.",
        );
        ctx.print_color_centered(
            30,
            YELLOW,
            BLACK,
            "Don't worry, you can always try again with a new hero.",
        );
        ctx.print_color_centered(34, GREEN, BLACK, "Press NumEnter to play again.");

        if let Some(VirtualKeyCode::NumpadEnter) = ctx.key {
            self.reset_game_state()
        }

        ctx.set_active_console(1);
        ctx.print_centered(11, 'o');
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, GREEN, BLACK, "You have won!");
        ctx.print_color_centered(
            18,
            WHITE,
            BLACK,
            "You put on the Amulet of Yala and feel its power course through \
your veins.",
        );
        ctx.print_color_centered(
            19,
            WHITE,
            BLACK,
            "Your town is saved, and you can return to your normal life.",
        );

        ctx.print_color_centered(34, GREEN, BLACK, "Press NumEnter to play again.");

        if let Some(VirtualKeyCode::NumpadEnter) = ctx.key {
            self.reset_game_state()
        }

        ctx.set_active_console(1);
        ctx.print_centered(11, '|');
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Clean screen
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();

        // Exit when Esc is pressed
        if let Some(VirtualKeyCode::Escape) = ctx.key {
            ctx.quitting = true;
        }

        // Register input
        self.resources.insert(ctx.key);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        // Run systems
        let current_state = self.resources.get::<TurnState>().unwrap().clone();

        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::NextLevel => self.advance_level(),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
        }

        if current_state != TurnState::GameOver && current_state != TurnState::Victory {
            self.common_systems
                .execute(&mut self.ecs, &mut self.resources);
        }

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
        //.with_fullscreen(true)
        .with_fps_cap(fps_cap)
        .with_dimensions(width, height)
        .with_tile_dimensions(tile_width, tile_height)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", tile_width, tile_height)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(width, height, "dungeonfont.png")
        .with_simple_console_no_bg(width, height, "dungeonfont.png")
        .with_simple_console_no_bg(width * 2, height * 2, "terminal8x8.png")
        .build()?;

    let state = State::new();

    main_loop(context, state)
}
