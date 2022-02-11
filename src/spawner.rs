use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

pub fn spawn_monster(esc: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    esc.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: match rng.range(0, 4) {
                0 => to_cp437('E'), // Ettin
                1 => to_cp437('O'), // Ogre
                2 => to_cp437('o'), // Ork
                _ => to_cp437('g'), // Goblin
            },
        },
    ));
}
