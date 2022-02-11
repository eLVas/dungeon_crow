use crate::prelude::*;

// TODO: Use system(for_each)

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_movement(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] rng: &mut RandomNumberGenerator,
) {
    <(Entity, &mut Point)>::query()
        .filter(component::<MovingRandomly>())
        .iter_mut(ecs)
        .for_each(|(entity, pos)| {
            let move_direction = match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            };

            let destination = *pos + move_direction;

            commands.push((
                (),
                MovementIntention {
                    entity: *entity,
                    destination,
                },
            ));
        });
}
