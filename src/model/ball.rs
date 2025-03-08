use lighthouse_client::protocol::{Delta, Direction, Pos};

#[derive(Debug, Clone, PartialEq)]
pub struct Ball {
    pos: Pos<f64>,
    velocity: Delta<f64>,
}

impl Ball {
    pub fn new(pos: Pos<f64>, velocity: Delta<f64>) -> Self {
        Self { pos, velocity }
    }

    pub fn apply_velocity(&mut self) {
        self.pos += self.velocity;
    }

    pub fn bounce(&mut self, normal_dir: Direction) {
        match normal_dir {
            Direction::Left | Direction::Right => self.velocity.x *= -1.0,
            Direction::Up | Direction::Down => self.velocity.y *= -1.0,
        }
    }

    pub fn grid_pos(&self) -> Pos<i32> {
        self.pos.map(|c| c.round() as i32)
    }
}
