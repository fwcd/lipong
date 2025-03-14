use lighthouse_client::protocol::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Options {
    pub ball_speed: f64,
    pub paddle_sensitivity: i32,

    pub paddle_color: Color,
    pub net_color: Color,
    pub digit_color: Color,
    pub ball_color: Color,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            ball_speed: 0.75,
            paddle_sensitivity: 1,
            paddle_color: Color::WHITE,
            net_color: Color::GRAY,
            digit_color: Color::GREEN,
            ball_color: Color::WHITE,
        }
    }
}
