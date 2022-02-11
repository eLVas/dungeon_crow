use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left | VirtualKeyCode::Key4 => Point::new(-1, 0),
            VirtualKeyCode::Right | VirtualKeyCode::Key6 => Point::new(1, 0),
            VirtualKeyCode::Up | VirtualKeyCode::Key8 => Point::new(0, -1),
            VirtualKeyCode::Down | VirtualKeyCode::Key2 => Point::new(0, 1),
            VirtualKeyCode::Key7 => Point::new(-1, -1),
            VirtualKeyCode::Key9 => Point::new(1, -1),
            VirtualKeyCode::Key1 => Point::new(-1, 1),
            VirtualKeyCode::Key3 => Point::new(1, 1),
            _ => Point::new(0, 0),
        };

        if delta.x != 0 || delta.y != 0 {
            <(Entity, &Point)>::query()
                .filter(component::<Player>())
                .iter(ecs)
                .for_each(|(entity, pos)| {
                    let destination = *pos + delta;

                    commands.push((
                        (),
                        MovementIntention {
                            entity: *entity,
                            destination,
                        },
                    ));
                });
            *turn_state = TurnState::PlayerTurn;
        }
    }
}
