use crate::prelude::*;

#[system]
#[read_component(AttackIntention)]
#[read_component(Player)]
#[read_component(Damage)]
#[read_component(Carried)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, cmd: &mut CommandBuffer) {
    let targets: Vec<(Entity, Entity, Entity)> = <(Entity, &AttackIntention)>::query()
        .iter(ecs)
        .map(|(entity, attack_intention)| {
            (*entity, attack_intention.attacker, attack_intention.target)
        })
        .collect();

    targets.iter().for_each(|(msg, attacker, target)| {
        let is_player = ecs
            .entry_ref(*target)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        let base_damage = if let Ok(e) = ecs.entry_ref(*attacker) {
            if let Ok(dmg) = e.get_component::<Damage>() {
                dmg.0
            } else {
                0
            }
        } else {
            0
        };

        let weapon_damage: i32 = <(&Carried, &Damage)>::query()
            .iter(ecs)
            .filter(|(carried, _)| carried.0 == *attacker)
            .map(|(_, dmg)| dmg.0)
            .sum();

        let final_damage = base_damage + weapon_damage;

        if let Ok(mut health) = ecs
            .entry_mut(*target)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= final_damage;

            if health.current < 1 && !is_player {
                cmd.remove(*target);
            }
        }
        cmd.remove(*msg);
    })
}
