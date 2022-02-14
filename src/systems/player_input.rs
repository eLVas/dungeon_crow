use crate::prelude::*;

#[system]
#[read_component(Player)]
#[write_component(Point)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 => Point::new(-1, 0),
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 => Point::new(1, 0),
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 => Point::new(0, -1),
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 => Point::new(0, 1),
            VirtualKeyCode::Numpad7 => Point::new(-1, -1),
            VirtualKeyCode::Numpad9 => Point::new(1, -1),
            VirtualKeyCode::Numpad1 => Point::new(-1, 1),
            VirtualKeyCode::Numpad3 => Point::new(1, 1),
            _ => Point::new(0, 0),
        };

        let (player, destination) = <(Entity, &Point)>::query()
            .filter(component::<Player>())
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();

        let mut did_something = false;
        if delta.x != 0 || delta.y != 0 {
            did_something = true;
            let mut hit_something = false;

            <(Entity, &Point)>::query()
                .filter(component::<Enemy>())
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    did_something = true;

                    commands.push((
                        (),
                        AttackIntention {
                            attacker: player,
                            target: *entity,
                        },
                    ));
                });

            if !hit_something {
                commands.push((
                    (),
                    MovementIntention {
                        entity: player,
                        destination,
                    },
                ));
            }
        }

        if !did_something {
            if let Ok(mut health) = ecs.entry_mut(player).unwrap().get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + 1);
            }
        }

        *turn_state = TurnState::PlayerTurn;
    }
}
