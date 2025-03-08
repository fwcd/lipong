use lighthouse_client::protocol::Rect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Paddle<const H: usize> {
    bounds: Rect<i32>,
}

impl<const H: usize> Paddle<H> {
    pub fn new(bounds: Rect<i32>) -> Self {
        Self {
            bounds,
        }
    }

    pub fn bounds(&self) -> Rect<i32> {
        self.bounds
    }

    pub fn move_by(&mut self, dy: i32) {
        let mut new_bounds = self.bounds;
        new_bounds.origin.y += dy;
        if new_bounds.top_left().y >= 0 && new_bounds.bottom_left().y < H as i32 {
            self.bounds = new_bounds;
        }
    }
}
