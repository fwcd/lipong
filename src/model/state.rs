use lighthouse_client::protocol::{Color, Direction, Frame};

use crate::model::PLAYER_COUNT;

use super::Board;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct State<const W: usize, const H: usize> {
    board: Board<W, H>,
    paddle_color: Color,
}

impl<const W: usize, const H: usize> State<W, H> {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            paddle_color: Color::WHITE,
        }
    }

    pub fn move_paddle(&mut self, i: usize, dir: Direction) {
        assert!(i < PLAYER_COUNT);
        let speed = 1;
        match dir {
            Direction::Up => self.board.paddle_mut(i).move_by(-speed),
            Direction::Down => self.board.paddle_mut(i).move_by(speed),
            _ => {},
        };
    }

    pub fn tick(&mut self) {
        // FIXME: Update the state here, i.e. implement game logic that needs to run on every tick
    }

    pub fn render(&self) -> Frame {
        let mut frame = Frame::empty();

        for paddle in self.board.paddles() {
            let bounds = paddle.bounds();
            for dy in 0..bounds.height() {
                for dx in 0..bounds.width() {
                    let x = (bounds.origin.x + dx) as usize;
                    let y = (bounds.origin.y + dy) as usize;
                    frame.set(x, y, self.paddle_color);
                }
            }
        }

        frame
    }
}
