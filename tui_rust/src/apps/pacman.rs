pub struct PacmanGame {
    pub player_pos: (usize, usize),
    pub score: u32,
    pub lives: u8,
}

impl PacmanGame {
    pub fn new() -> Self {
        Self {
            player_pos: (5, 5),
            score: 0,
            lives: 3,
        }
    }
}