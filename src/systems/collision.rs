use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Treasure)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collision(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] score_counter: &mut ScoreCounter,
) {
    if let Some(player_pos) = <&Point>::query()
        .filter(component::<Player>())
        .iter(ecs)
        // Take a first value as there should be only one player
        .next()
    {
        // Kill enemies
        <(Entity, &Point)>::query()
            .filter(component::<Enemy>())
            .iter(ecs)
            .filter(|(_, pos)| **pos == *player_pos)
            .for_each(|(entity, _)| {
                commands.remove(*entity);
                score_counter.add(1);
            });

        // Collect loot
        <(Entity, &Treasure, &Point)>::query()
            .iter(ecs)
            .filter(|(_, _, pos)| **pos == *player_pos)
            .for_each(|(entity, treasure, _)| {
                commands.remove(*entity);
                score_counter.add(treasure.value);
            });
    }
}
