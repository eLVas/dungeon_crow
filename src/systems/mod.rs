mod collision;
mod entity_render;
mod map_render;
mod player_input;
mod radom_movement;
mod ui_render;

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(collision::collision_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(ui_render::ui_render_system())
        .add_system(radom_movement::random_movement_system())
        .build()
}
