mod chase;
mod combat;
mod end_turn;
mod entity_render;
mod fov;
mod hud_render;
mod map_render;
mod monster_ai;
mod movement;
mod player_input;
mod tooltips;
mod wander;

use crate::prelude::*;

pub fn build_common_systems() -> Schedule {
    Schedule::builder()
        .add_system(fov::fov_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud_render::hud_render_system())
        .add_system(tooltips::tooltips_system())
        .build()
}

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(monster_ai::behaviour_system())
        .flush()
        .add_system(wander::random_movement_system())
        .add_system(chase::chase_system())
        .flush()
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(end_turn::end_turn_system())
        .build()
}
