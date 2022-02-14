use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Name("Eric".to_string()),
        Health {
            current: 12,
            max: 20,
        },
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

pub fn spawn_monster(esc: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    esc.push((
        Enemy,
        Name(name),
        Health {
            current: hp,
            max: hp,
        },
        MovingRandomly,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
    ));
}

pub fn spawn_treasure(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    ecs.push((
        Treasure {
            value: rng.range(1, 5),
        },
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: match rng.range(0, 5) {
                0 => to_cp437('|'),
                1 => to_cp437('!'),
                2 => to_cp437('{'),
                3 => to_cp437('s'),
                4 => to_cp437('S'),
                _ => to_cp437('/'),
            },
        },
        Name("Shiny thing".to_string()),
    ));
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}
