use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    intended_move: &MovementIntention,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.traversable(intended_move.destination) {
        commands.add_component(intended_move.entity, intended_move.destination);

        if let Ok(entry) = ecs.entry_ref(intended_move.entity) {
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                commands.add_component(intended_move.entity, fov.clone_dirty());

                if entry.get_component::<Player>().is_ok() {
                    camera.center_on_player(intended_move.destination);

                    let visible_tiles: Vec<Point> = fov.visible_tiles.iter().copied().collect();

                    map.reveal_tiles(&visible_tiles);
                }
            }
        }
    }

    commands.remove(*entity);
}
