use lighthouse_client::protocol::{Rect, Vec2, LIGHTHOUSE_RECT};

use super::{Paddle, PLAYER_COUNT};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board<const W: usize, const H: usize> {
    paddles: [Paddle<H>; PLAYER_COUNT]
}

impl<const W: usize, const H: usize> Board<W, H> {
    pub fn new() -> Self {
        let paddle_size = Vec2::new(1, H as i32 / 4);
        let padding = 1;

        Self {
            paddles: [
                Paddle::new(Rect::new(LIGHTHOUSE_RECT.center_left() + Vec2::new(padding, 0), paddle_size)),
                Paddle::new(Rect::new(LIGHTHOUSE_RECT.center_right() - Vec2::new(padding + 1, 0), paddle_size)),
            ],
        }
    }

    pub fn paddles(&self) -> &[Paddle<H>; PLAYER_COUNT] {
        &self.paddles
    }

    #[allow(unused)]
    pub fn paddle(&self, i: usize) -> &Paddle<H> {
        &self.paddles[i]
    }

    pub fn paddle_mut(&mut self, i: usize) -> &mut Paddle<H> {
        &mut self.paddles[i]
    }
}
