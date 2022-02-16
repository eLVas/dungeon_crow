use crate::prelude::*;

#[system]
#[read_component(AttackIntention)]
#[read_component(Player)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, cmd: &mut CommandBuffer) {
    let targets: Vec<(Entity, Entity)> = <(Entity, &AttackIntention)>::query()
        .iter(ecs)
        .map(|(entity, attack_intention)| (*entity, attack_intention.target))
        .collect();

    targets.iter().for_each(|(msg, target)| {
        let is_player = ecs
            .entry_ref(*target)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        if let Ok(mut health) = ecs
            .entry_mut(*target)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= 1;

            if health.current < 1 && !is_player {
                cmd.remove(*target);
            }
        }
        cmd.remove(*msg);
    })
}
