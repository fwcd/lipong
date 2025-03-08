use lighthouse_client::protocol::{Color, Direction, Frame};

use crate::model::PLAYER_COUNT;

use super::Board;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct State<const W: usize, const H: usize> {
    board: Board<W, H>,
    paddle_color: Color,
    net_color: Color,
}

impl<const W: usize, const H: usize> State<W, H> {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            paddle_color: Color::WHITE,
            net_color: Color::GRAY,
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
        self.render_net(&mut frame);
        self.render_paddles(&mut frame);
        frame
    }

    fn render_net(&self, frame: &mut Frame) {
        let x = W / 2;
        for y in 0..H {
            frame.set(x, y, self.net_color);
        }
    }

    fn render_paddles(&self, frame: &mut Frame) {
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
    }
}
