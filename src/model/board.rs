use std::f64::consts::TAU;

use lighthouse_client::protocol::{Rect, Vec2, Zero};

use super::{Ball, Paddle, PLAYER_COUNT};

#[derive(Debug, Clone, PartialEq)]
pub struct Board<const W: usize, const H: usize> {
    paddles: [Paddle; PLAYER_COUNT],
    ball: Ball,
}

impl<const W: usize, const H: usize> Board<W, H> {
    pub fn new() -> Self {
        let paddle_size = Vec2::new(1, H as i32 / 4);
        let padding = 1;

        Self {
            paddles: [
                Paddle::new(Rect::new(Self::bounds().center_left() + Vec2::new(padding, 0), paddle_size)),
                Paddle::new(Rect::new(Self::bounds().center_right() - Vec2::new(padding + 1, 0), paddle_size)),
            ],
            ball: Self::new_ball(),
        }
    }

    pub fn bounds() -> Rect<i32> {
        Rect::new(Vec2::ZERO, Vec2::new(W as i32, H as i32))
    }

    fn new_ball() -> Ball {
        let angle = rand::random_range(0.0..=TAU);
        let pos = Vec2::new((W / 2) as f64, (H / 2) as f64);
        let delta = Vec2::new(angle.cos(), angle.sin());
        Ball::new(pos, delta)
    }

    pub fn move_paddle(&mut self, i: usize, dy: i32) {
        let mut paddle: Paddle = self.paddles[i];
        paddle.move_by(dy);
        if paddle.in_bounds(Self::bounds()) {
            self.paddles[i] = paddle;
        }
    }

    pub fn paddles(&self) -> &[Paddle; PLAYER_COUNT] {
        &self.paddles
    }
}
