use std::f64::consts::TAU;

use lighthouse_client::protocol::{Direction, Rect, Vec2, Zero};

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
        let mut paddle = self.paddles[i];
        paddle.move_by(dy);
        if paddle.in_bounds(Self::bounds()) {
            self.paddles[i] = paddle;
        }
    }

    pub fn respawn_ball(&mut self) {
        self.ball = Self::new_ball();
    }

    pub fn tick_ball(&mut self) -> Option<usize> {
        let mut ball = self.ball;
        ball.apply_velocity();

        if let Some(dir) = Self::colliding_wall_normal(ball) {
            match dir {
                Direction::Left => return Some(0),
                Direction::Right => return Some(1),
                _ => {},
            }

            ball.bounce(dir);
        }

        if let Some(dir) = self.colliding_paddle_normal(ball) {
            ball.bounce(dir); // TODO: Randomize direction
        }

        self.ball = ball;
        None
    }

    fn colliding_wall_normal(ball: Ball) -> Option<Direction> {
        let pos = ball.exact_pos();
        if pos.x < 0.0 {
            Some(Direction::Down)
        } else if pos.x >= W as f64 {
            Some(Direction::Up)
        } else if pos.y < 0.0 {
            Some(Direction::Right)
        } else if pos.y >= H as f64 {
            Some(Direction::Left)
        } else {
            None
        }
    }

    fn colliding_paddle_normal(&self, ball: Ball) -> Option<Direction> {
        if self.paddles[0].collides_with(ball) {
            Some(Direction::Right)
        } else if self.paddles[1].collides_with(ball) {
            Some(Direction::Left)
        } else {
            None
        }
    }

    pub fn paddles(&self) -> &[Paddle; PLAYER_COUNT] {
        &self.paddles
    }
}
