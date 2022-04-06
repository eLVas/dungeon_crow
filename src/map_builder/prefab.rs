use crate::prelude::*;

pub const FORTRESS: (&str, i32, i32) = (
    "
------------
---######---
---#----#---
---#-M--#---
-###----###-
--M------M--
-###----###-
---#----#---
---#----#---
---######---
------------
",
    12,
    11,
);

pub fn apply_structure(
    mb: &mut MapBuilder,
    rng: &mut RandomNumberGenerator,
    prefab: (&str, i32, i32),
) {
    let mut placement = None;

    let dijkstra_map = DijkstraMap::new(
        WORLD_WIDTH,
        WORLD_HEIGHT,
        &vec![mb.map.point2d_to_index(mb.player_start)],
        &mb.map,
        1024.0,
    );

    let mut attempts = 0;
    while placement.is_none() && attempts < 10 {
        let dimentions = Rect::with_size(
            rng.range(0, WORLD_WIDTH - prefab.1),
            rng.range(0, WORLD_HEIGHT - prefab.2),
            prefab.1,
            prefab.2,
        );

        let mut can_place = false;
        dimentions.for_each(|pt| {
            let idx = mb.map.point2d_to_index(pt);
            let distance = dijkstra_map.map[idx];
            if distance < 2000.0 && distance > 20.0 && mb.amulet_start != pt {
                can_place = true;
            }
        });

        if can_place {
            placement = Some(Point::new(dimentions.x1, dimentions.y1));
            let points = dimentions.point_set();
            mb.monster_spawns.retain(|(pt, _)| !points.contains(pt));
        }

        attempts += 1;
    }

    if let Some(placement) = placement {
        let string_vec: Vec<char> = prefab
            .0
            .chars()
            .filter(|a| *a != '\r' && *a != '\n')
            .collect();

        let mut i = 0;

        for ty in placement.y..placement.y + prefab.2 {
            for tx in placement.x..placement.x + prefab.1 {
                let idx = map_idx(tx, ty);
                let c = string_vec[i];

                match c {
                    'M' => {
                        mb.map.tiles[idx] = TileType::Floor;
                        mb.monster_spawns.push((Point::new(tx, ty), false));
                    }
                    '-' => mb.map.tiles[idx] = TileType::Floor,
                    '#' => mb.map.tiles[idx] = TileType::Wall,
                    _ => println!("Prefab error. Prefab contains unrecognized symbol [{}]", c),
                }
                i += 1;
            }
        }
    }
}
