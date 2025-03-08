use lighthouse_client::protocol::{Rect, Vec2};

use super::Ball;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Paddle {
    bounds: Rect<i32>,
}

impl Paddle {
    pub fn new(bounds: Rect<i32>) -> Self {
        Self {
            bounds,
        }
    }

    pub fn bounds(&self) -> Rect<i32> {
        self.bounds
    }

    pub fn collides_with(&self, ball: Ball) -> bool {
        self.bounds.contains(ball.grid_pos())
    }

    pub fn move_by(&mut self, dy: i32) {
        self.bounds.origin.y += dy;
    }

    pub fn in_bounds(&self, rect: Rect<i32>) -> bool {
        let bounds = self.bounds;
        rect.contains(bounds.top_left()) && rect.contains(bounds.bottom_right() - Vec2::new(1, 1))
    }
}
