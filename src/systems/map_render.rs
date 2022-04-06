use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &Camera,
    #[resource] theme: &Box<dyn MapTheme>,
) {
    let mut draw_batch = DrawBatch::new();

    // Draw to map layer
    draw_batch.target(0);

    let player_fov = <&FieldOfView>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .unwrap();

    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            let pt = Point::new(x, y);
            let offset = camera.get_offset();

            if map.in_bounds(pt)
                && (player_fov.visible_tiles.contains(&pt) | map.revealed_tiles.contains(&pt))
            {
                let idx = map_idx(x, y);
                let glyph = theme.tile_to_render(map.tiles[idx]);
                let tint = if player_fov.visible_tiles.contains(&pt) {
                    WHITE
                } else {
                    DARK_GREY
                };

                draw_batch.set(pt - offset, ColorPair::new(tint, BLACK), glyph);
            }
        }
    }

    draw_batch.submit(0).expect("Batch draw error");
}
