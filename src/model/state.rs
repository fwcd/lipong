use lighthouse_client::protocol::{Color, Frame};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct State {
    // FIXME: Add the game/app state here
}

impl State {
    pub fn new() -> Self {
        Self {}
    }

    pub fn tick(&mut self) {
        // FIXME: Update the state here, i.e. implement game logic that needs to run on every tick
    }

    pub fn render(&self) -> Frame {
        // FIXME: Render the state to a frame here
        Frame::fill(Color::BLUE)
    }
}
