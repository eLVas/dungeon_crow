#[derive(Debug)]
pub struct ScoreCounter {
    pub score: i32,
}

impl ScoreCounter {
    pub fn new() -> Self {
        Self { score: 0 }
    }

    pub fn add(&mut self, points: i32) {
        self.score += points;
    }
}
