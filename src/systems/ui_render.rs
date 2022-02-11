use crate::prelude::*;

#[system]
pub fn ui_render(#[resource] score_counter: &ScoreCounter) {
    let mut draw_batch = DrawBatch::new();

    // Draw to ui layer
    draw_batch.target(2);

    draw_batch.print(Point::new(1, 1), format!("Score: {}", score_counter.score));

    draw_batch.submit(6000).expect("Batch draw error");
}
