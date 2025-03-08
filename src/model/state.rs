use lighthouse_client::protocol::{Color, Frame};

use super::Board;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct State<const W: usize, const H: usize> {
    board: Board<W, H>,
}

impl<const W: usize, const H: usize> State<W, H> {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
        }
    }

    pub fn tick(&mut self) {
        // FIXME: Update the state here, i.e. implement game logic that needs to run on every tick
    }

    pub fn render(&self) -> Frame {
        // FIXME: Render the state to a frame here
        Frame::fill(Color::BLUE)
    }
}
