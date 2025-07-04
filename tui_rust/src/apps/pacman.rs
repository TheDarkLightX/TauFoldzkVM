use std::collections::HashSet;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GhostMode {
    Chase,
    Scatter,
    Frightened,
    Eaten,
}

#[derive(Debug, Clone)]
pub struct Ghost {
    pub position: (u8, u8),
    pub home_position: (u8, u8),
    pub direction: Direction,
    pub mode: GhostMode,
    pub color: GhostColor,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GhostColor {
    Red,    // Blinky - chases Pacman directly
    Pink,   // Pinky - tries to get ahead of Pacman
    Blue,   // Inky - uses Blinky's position to trap
    Orange, // Clyde - switches between chase and scatter
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Playing,
    PowerUp(u32), // countdown timer
    GameOver,
    Victory,
    Paused,
}

#[derive(Clone)]
pub struct PacmanGame {
    pub player_pos: (u8, u8),
    pub player_dir: Direction,
    pub ghosts: Vec<Ghost>,
    pub dots: HashSet<(u8, u8)>,
    pub power_pellets: HashSet<(u8, u8)>,
    pub score: u32,
    pub lives: u8,
    pub level: u8,
    pub game_state: GameState,
    pub maze: [[bool; 19]; 21], // true = wall
    pub dots_eaten: u32,
    pub ghosts_eaten_combo: u32,
}

impl PacmanGame {
    pub fn new() -> Self {
        let mut game = Self {
            player_pos: (9, 15),
            player_dir: Direction::Right,
            ghosts: vec![
                Ghost {
                    position: (9, 9),
                    home_position: (9, 9),
                    direction: Direction::Up,
                    mode: GhostMode::Scatter,
                    color: GhostColor::Red,
                },
                Ghost {
                    position: (8, 9),
                    home_position: (8, 9),
                    direction: Direction::Down,
                    mode: GhostMode::Scatter,
                    color: GhostColor::Pink,
                },
                Ghost {
                    position: (10, 9),
                    home_position: (10, 9),
                    direction: Direction::Up,
                    mode: GhostMode::Scatter,
                    color: GhostColor::Blue,
                },
                Ghost {
                    position: (9, 10),
                    home_position: (9, 10),
                    direction: Direction::Left,
                    mode: GhostMode::Scatter,
                    color: GhostColor::Orange,
                },
            ],
            dots: HashSet::new(),
            power_pellets: HashSet::new(),
            score: 0,
            lives: 3,
            level: 1,
            game_state: GameState::Playing,
            maze: [[false; 19]; 21],
            dots_eaten: 0,
            ghosts_eaten_combo: 0,
        };
        
        game.initialize_maze();
        game.place_dots();
        game
    }
    
    fn initialize_maze(&mut self) {
        // Simple maze layout - true = wall
        // Row 0 - top wall
        for x in 0..19 {
            self.maze[0][x] = true;
            self.maze[20][x] = true;
        }
        
        // Side walls
        for y in 0..21 {
            self.maze[y][0] = true;
            self.maze[y][18] = true;
        }
        
        // Internal walls (simplified maze)
        // Horizontal walls
        for x in 2..7 {
            self.maze[2][x] = true;
            self.maze[2][18-x] = true;
        }
        
        for x in 2..17 {
            if x < 7 || x > 11 {
                self.maze[5][x] = true;
            }
        }
        
        // Ghost house
        for x in 7..12 {
            self.maze[8][x] = true;
            self.maze[11][x] = true;
        }
        for y in 8..12 {
            self.maze[y][7] = true;
            self.maze[y][11] = true;
        }
        
        // More walls for interesting gameplay
        for x in 2..5 {
            self.maze[14][x] = true;
            self.maze[14][18-x] = true;
        }
        
        for y in 15..18 {
            self.maze[y][9] = true;
        }
    }
    
    fn place_dots(&mut self) {
        for y in 1..20 {
            for x in 1..18 {
                if !self.maze[y][x] && !self.is_ghost_house(x, y) {
                    self.dots.insert((x as u8, y as u8));
                }
            }
        }
        
        // Place power pellets
        self.power_pellets.insert((1, 1));
        self.power_pellets.insert((17, 1));
        self.power_pellets.insert((1, 19));
        self.power_pellets.insert((17, 19));
        
        // Remove dots where power pellets are
        for pellet in &self.power_pellets {
            self.dots.remove(pellet);
        }
    }
    
    fn is_ghost_house(&self, x: usize, y: usize) -> bool {
        x >= 8 && x <= 10 && y >= 9 && y <= 10
    }
    
    pub fn move_player(&mut self, direction: Direction) -> bool {
        if self.game_state == GameState::GameOver || self.game_state == GameState::Victory {
            return false;
        }
        
        let (x, y) = self.player_pos;
        let (new_x, new_y) = match direction {
            Direction::Up => (x, y.saturating_sub(1)),
            Direction::Down => (x, (y + 1).min(20)),
            Direction::Left => (x.saturating_sub(1), y),
            Direction::Right => ((x + 1).min(18), y),
        };
        
        if !self.maze[new_y as usize][new_x as usize] {
            self.player_pos = (new_x, new_y);
            self.player_dir = direction;
            
            // Check dot collection
            if self.dots.remove(&(new_x, new_y)) {
                self.score += 10;
                self.dots_eaten += 1;
                
                if self.dots.is_empty() && self.power_pellets.is_empty() {
                    self.game_state = GameState::Victory;
                    self.score += 1000 * self.level as u32;
                }
            }
            
            // Check power pellet
            if self.power_pellets.remove(&(new_x, new_y)) {
                self.score += 50;
                self.game_state = GameState::PowerUp(200); // 200 ticks
                self.ghosts_eaten_combo = 0;
                
                // Set all ghosts to frightened
                for ghost in &mut self.ghosts {
                    if ghost.mode != GhostMode::Eaten {
                        ghost.mode = GhostMode::Frightened;
                        ghost.reverse_direction();
                    }
                }
            }
            
            // Check ghost collision
            self.check_collisions();
            
            true
        } else {
            false
        }
    }
    
    pub fn update(&mut self) {
        match &mut self.game_state {
            GameState::PowerUp(timer) => {
                *timer = timer.saturating_sub(1);
                if *timer == 0 {
                    self.game_state = GameState::Playing;
                    for ghost in &mut self.ghosts {
                        if ghost.mode == GhostMode::Frightened {
                            ghost.mode = GhostMode::Chase;
                        }
                    }
                }
            }
            GameState::Playing => {
                // Update ghost modes based on level timer
                // Simplified: alternate between scatter and chase
            }
            _ => {}
        }
        
        // Move ghosts
        self.move_ghosts();
        
        // Check collisions after ghost movement
        self.check_collisions();
    }
    
    fn move_ghosts(&mut self) {
        let mut rng = rand::thread_rng();
        
        for i in 0..self.ghosts.len() {
            let (ghost_mode, ghost_color, ghost_position, ghost_direction, ghost_home_position) = {
                let ghost = &self.ghosts[i];
                (ghost.mode, ghost.color, ghost.position, ghost.direction, ghost.home_position)
            };
            
            let target = match ghost_mode {
                GhostMode::Chase => self.get_chase_target(ghost_color),
                GhostMode::Scatter => ghost_home_position,
                GhostMode::Frightened => {
                    // Random movement when frightened
                    (rng.gen_range(1..18), rng.gen_range(1..20))
                }
                GhostMode::Eaten => (9, 9), // Return to ghost house
            };
            
            let new_dir = self.get_best_direction(ghost_position, target, ghost_direction);
            let (x, y) = ghost_position;
            let (new_x, new_y) = match new_dir {
                Direction::Up => (x, y.saturating_sub(1)),
                Direction::Down => (x, (y + 1).min(20)),
                Direction::Left => (x.saturating_sub(1), y),
                Direction::Right => ((x + 1).min(18), y),
            };
            
            if !self.maze[new_y as usize][new_x as usize] {
                self.ghosts[i].position = (new_x, new_y);
                self.ghosts[i].direction = new_dir;
            }
            
            // If eaten ghost reaches home, revive it
            if ghost_mode == GhostMode::Eaten && self.ghosts[i].position == (9, 9) {
                self.ghosts[i].mode = GhostMode::Chase;
            }
        }
    }
    
    fn get_chase_target(&self, color: GhostColor) -> (u8, u8) {
        match color {
            GhostColor::Red => self.player_pos, // Direct chase
            GhostColor::Pink => {
                // Try to get 4 tiles ahead of Pacman
                let (x, y) = self.player_pos;
                match self.player_dir {
                    Direction::Up => (x, y.saturating_sub(4)),
                    Direction::Down => (x, (y + 4).min(20)),
                    Direction::Left => (x.saturating_sub(4), y),
                    Direction::Right => ((x + 4).min(18), y),
                }
            }
            GhostColor::Blue => {
                // Complex targeting using Red ghost position
                self.player_pos // Simplified
            }
            GhostColor::Orange => {
                // Chase if far, scatter if close
                let orange_ghost = &self.ghosts[3];
                let dist = self.manhattan_distance(orange_ghost.position, self.player_pos);
                if dist > 8 {
                    self.player_pos
                } else {
                    orange_ghost.home_position
                }
            }
        }
    }
    
    fn get_best_direction(&self, from: (u8, u8), to: (u8, u8), current_dir: Direction) -> Direction {
        let mut best_dir = current_dir;
        let mut best_dist = u32::MAX;
        
        for dir in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            // Ghosts can't reverse direction (except when mode changes)
            if dir == self.opposite_direction(current_dir) {
                continue;
            }
            
            let (x, y) = from;
            let (new_x, new_y) = match dir {
                Direction::Up => (x, y.saturating_sub(1)),
                Direction::Down => (x, (y + 1).min(20)),
                Direction::Left => (x.saturating_sub(1), y),
                Direction::Right => ((x + 1).min(18), y),
            };
            
            if !self.maze[new_y as usize][new_x as usize] {
                let dist = self.manhattan_distance((new_x, new_y), to);
                if dist < best_dist {
                    best_dist = dist;
                    best_dir = dir;
                }
            }
        }
        
        best_dir
    }
    
    fn manhattan_distance(&self, a: (u8, u8), b: (u8, u8)) -> u32 {
        ((a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()) as u32
    }
    
    fn opposite_direction(&self, dir: Direction) -> Direction {
        match dir {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
    
    fn check_collisions(&mut self) {
        let mut should_reset = false;
        let mut ghost_eaten_indices = Vec::new();
        
        for (i, ghost) in self.ghosts.iter().enumerate() {
            if ghost.position == self.player_pos {
                match ghost.mode {
                    GhostMode::Frightened => {
                        // Eat ghost
                        ghost_eaten_indices.push(i);
                        self.ghosts_eaten_combo += 1;
                        let points = 200 * (1 << self.ghosts_eaten_combo);
                        self.score += points;
                    }
                    GhostMode::Chase | GhostMode::Scatter => {
                        // Lose life
                        self.lives = self.lives.saturating_sub(1);
                        if self.lives == 0 {
                            self.game_state = GameState::GameOver;
                        } else {
                            should_reset = true;
                        }
                    }
                    GhostMode::Eaten => {} // No collision with eaten ghosts
                }
            }
        }
        
        // Apply ghost eaten changes
        for i in ghost_eaten_indices {
            self.ghosts[i].mode = GhostMode::Eaten;
        }
        
        // Reset positions if needed
        if should_reset {
            self.reset_positions();
        }
    }
    
    fn reset_positions(&mut self) {
        self.player_pos = (9, 15);
        self.player_dir = Direction::Right;
        
        for (i, ghost) in self.ghosts.iter_mut().enumerate() {
            ghost.position = ghost.home_position;
            ghost.mode = GhostMode::Scatter;
            ghost.direction = match i {
                0 => Direction::Up,
                1 => Direction::Down,
                2 => Direction::Up,
                3 => Direction::Left,
                _ => Direction::Up,
            };
        }
    }
    
    pub fn prepare_zkvm_input(&self) -> Vec<u32> {
        vec![
            self.player_pos.0 as u32,
            self.player_pos.1 as u32,
            self.score,
            self.lives as u32,
            self.dots_eaten,
            match &self.game_state {
                GameState::Playing => 0,
                GameState::PowerUp(_) => 1,
                GameState::GameOver => 2,
                GameState::Victory => 3,
                GameState::Paused => 4,
            },
        ]
    }
    
    pub fn process_zkvm_result(&mut self, result: &[u32]) {
        if !result.is_empty() && result[0] == 1 {
            // Game state verified by zkVM
        }
    }
}

impl Ghost {
    fn reverse_direction(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };
    }
}