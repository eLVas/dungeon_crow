use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
#[read_component(Health)]
#[read_component(Player)]
pub fn random_movement(ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();
    movers.iter(ecs).for_each(|(entity, pos, _)| {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 8) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            3 => Point::new(0, 1),
            4 => Point::new(-1, -1),
            5 => Point::new(1, -1),
            6 => Point::new(-1, 1),
            _ => Point::new(1, 1),
        } + *pos;

        let mut attacked = false;
        positions
            .iter(ecs)
            .filter(|(_, target_pos, _)| **target_pos == destination)
            .for_each(|(target, _, _)| {
                if ecs
                    .entry_ref(*target)
                    .unwrap()
                    .get_component::<Player>()
                    .is_ok()
                {
                    commands.push((
                        (),
                        AttackIntention {
                            attacker: *entity,
                            target: *target,
                        },
                    ));
                }
                attacked = true;
            });

        if !attacked {
            commands.push((
                (),
                MovementIntention {
                    entity: *entity,
                    destination,
                },
            ));
        }
    });
}
