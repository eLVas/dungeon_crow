use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    intended_move: &MovementIntention,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.traversable(intended_move.destination) {
        commands.add_component(intended_move.entity, intended_move.destination);

        if ecs
            .entry_ref(intended_move.entity)
            .unwrap()
            .get_component::<Player>()
            .is_ok()
        {
            camera.center_on_player(intended_move.destination);
        }
    }

    commands.remove(*entity);
}
