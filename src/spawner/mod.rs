mod template;

use crate::prelude::*;
use crate::spawner::template::Templates;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player { map_level: 0 },
        pos,
        Name("Eric".to_string()),
        Health { current: 3, max: 5 },
        FieldOfView::new(8),
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Damage(1),
    ));
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        Name("Amulet of Yala".to_string()),
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
    ));
}

pub fn spawn_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point],
) {
    // TODO: optimize this to load only once
    let template = Templates::load();
    template.spawn_entities(ecs, rng, level, spawn_points);
}
