use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_movement(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] rng: &mut RandomNumberGenerator,
) {
    <&mut Point>::query()
        .filter(component::<MovingRandomly>())
        .iter_mut(ecs)
        .for_each(|pos| {
            let move_direction = match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            };

            let destination = *pos + move_direction;

            if map.traversable(destination) {
                *pos = destination;
            }
        });
}
