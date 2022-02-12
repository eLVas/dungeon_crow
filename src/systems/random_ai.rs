use crate::prelude::*;

#[system(for_each)]
#[filter(component::<MovingRandomly>())]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_movement(
    entity: &Entity,
    position: &Point,
    commands: &mut CommandBuffer,
    #[resource] rng: &mut RandomNumberGenerator,
) {
    let move_direction = match rng.range(0, 8) {
        0 => Point::new(-1, 0),
        1 => Point::new(1, 0),
        2 => Point::new(0, -1),
        3 => Point::new(0, 1),
        4 => Point::new(-1, -1),
        5 => Point::new(1, -1),
        6 => Point::new(-1, 1),
        _ => Point::new(1, 1),
    };

    let destination = *position + move_direction;

    commands.push((
        (),
        MovementIntention {
            entity: *entity,
            destination,
        },
    ));
}
