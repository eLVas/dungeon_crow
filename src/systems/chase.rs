use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(FieldOfView)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chase(#[resource] map: &Map, ecs: &SubWorld, cmd: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &ChasingPlayer, &FieldOfView)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();
    let mut player = <(&Point, &Player)>::query();

    let player_pos = player.iter(ecs).nth(0).unwrap().0;
    let player_idx = map_idx_point(*player_pos);

    let search_targets = vec![player_idx];
    let map_dimensions = map.dimensions();
    let dijkstra_map = DijkstraMap::new(
        map_dimensions.x,
        map_dimensions.y,
        &search_targets,
        map,
        1024.0,
    );

    movers.iter(ecs).for_each(|(entity, pos, _, fov)| {
        if !fov.visible_tiles.contains(&player_pos) {
            return;
        }

        let idx = map_idx_point(*pos);
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);

            let destination = if distance > 1.5 {
                map.index_to_point2d(destination)
            } else {
                *player_pos
            };

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
                        cmd.push((
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
                cmd.push((
                    (),
                    MovementIntention {
                        entity: *entity,
                        destination,
                    },
                ));
            }
        }
    })
}
