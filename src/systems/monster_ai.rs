use crate::prelude::*;

#[system[for_each]]
#[filter[component::<Enemy>()]]
#[filter[component::<MovingRandomly>()]]
#[read_component(Point)]
#[read_component(FieldOfView)]
#[read_component(MovingRandomly)]
pub fn behaviour(entity: &Entity, fov: &FieldOfView, ecs: &SubWorld, cmd: &mut CommandBuffer) {
    let player_pos = <&Point>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .unwrap();

    if fov.visible_tiles.contains(player_pos) {
        cmd.add_component(*entity, ChasingPlayer);
        cmd.remove_component::<MovingRandomly>(*entity);
    }
}
