use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud_render(ecs: &SubWorld, #[resource] score_counter: &ScoreCounter) {
    let mut draw_batch = DrawBatch::new();

    // Draw to ui layer
    draw_batch.target(2);

    // Display player health
    let player_health = <&Health>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .unwrap();

    draw_batch.bar_horizontal(
        Point::zero(),
        DISPLAY_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!(
            " Health: {} / {} ",
            player_health.current, player_health.max
        ),
        ColorPair::new(WHITE, RED),
    );

    // Display score
    draw_batch.print(Point::new(1, 2), format!("Score: {}", score_counter.score));

    draw_batch.submit(10000).expect("Batch draw error");
}
