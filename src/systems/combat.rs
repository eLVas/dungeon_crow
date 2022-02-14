use crate::prelude::*;

#[system]
#[read_component(AttackIntention)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, cmd: &mut CommandBuffer) {
    let targets: Vec<(Entity, Entity)> = <(Entity, &AttackIntention)>::query()
        .iter(ecs)
        .map(|(entity, attack_intention)| (*entity, attack_intention.target))
        .collect();

    targets.iter().for_each(|(msg, target)| {
        if let Ok(mut health) = ecs
            .entry_mut(*target)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack: {}", health.current);
            health.current -= 1;

            if health.current < 1 {
                cmd.remove(*target);
            }

            println!("Health after attack: {}", health.current);
        }
        cmd.remove(*msg);
    })
}
