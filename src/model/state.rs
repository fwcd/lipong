use lighthouse_client::protocol::{Delta, Direction, Frame, Pos};
use tracing::info;

use crate::model::PLAYER_COUNT;

use super::{Board, Options, DIGITS, DIGIT_HEIGHT, DIGIT_WIDTH};

#[derive(Debug, PartialEq, Clone)]
pub struct State<const W: usize, const H: usize> {
    board: Board<W, H>,
    scores: [usize; PLAYER_COUNT],
    opts: Options,
}

impl<const W: usize, const H: usize> State<W, H> {
    pub fn new(opts: Options) -> Self {
        Self {
            board: Board::new(opts.ball_speed),
            scores: [0; PLAYER_COUNT],
            opts,
        }
    }

    pub fn move_paddle(&mut self, i: usize, dir: Direction) {
        assert!(i < PLAYER_COUNT);
        let speed = self.opts.paddle_sensitivity;
        match dir {
            Direction::Up => self.board.move_paddle(i, -speed),
            Direction::Down => self.board.move_paddle(i, speed),
            _ => {},
        };
    }

    pub fn teleport_paddle(&mut self, i: usize, y: i32) {
        assert!(i < PLAYER_COUNT);
        self.board.teleport_paddle(i, y);
    }

    pub fn tick(&mut self) {
        if let Some(goal_scorer) = self.board.tick_ball() {
            info!("Goal for player {} (scores: {} : {}), respawning ball...", goal_scorer + 1, self.scores[0], self.scores[1]);
            self.scores[goal_scorer] += 1;
            self.board.respawn_ball();
        }
    }

    pub fn render(&self) -> Frame {
        let mut frame = Frame::empty();
        self.render_scores(&mut frame);
        self.render_net(&mut frame);
        self.render_paddles(&mut frame);
        self.render_ball(&mut frame);
        frame
    }

    fn render_net(&self, frame: &mut Frame) {
        let x = W / 2;
        for y in 0..H {
            frame.set(x, y, self.opts.net_color);
        }
    }

    fn render_paddles(&self, frame: &mut Frame) {
        for paddle in self.board.paddles() {
            let bounds = paddle.bounds();
            for dy in 0..bounds.height() {
                for dx in 0..bounds.width() {
                    let x = (bounds.origin.x + dx) as usize;
                    let y = (bounds.origin.y + dy) as usize;
                    frame.set(x, y, self.opts.paddle_color);
                }
            }
        }
    }

    fn render_ball(&self, frame: &mut Frame) {
        let pos = self.board.ball().grid_pos();
        frame.set(pos.x as usize, pos.y as usize, self.opts.ball_color);
    }

    fn render_scores(&self, frame: &mut Frame) {
        let padding = Delta::new(2, 1);
        for i in 0..PLAYER_COUNT {
            let pos = Pos::new(((i * W) / PLAYER_COUNT) as i32, 0) + padding;
            self.render_int(self.scores[i], pos, frame);
        }
    }

    fn render_int(&self, n: usize, origin: Pos<i32>, frame: &mut Frame) -> usize {
        let base = DIGITS.len();
        let mut digits = 1;

        if n > base {
            digits += self.render_int(n / base, origin, frame);
        }

        let spacing = 1;
        self.render_digit(n % base, origin + Delta::new(((digits - 1) * (spacing + DIGIT_WIDTH)) as i32, 0), frame);

        digits
    }

    fn render_digit(&self, d: usize, origin: Pos<i32>, frame: &mut Frame) {
        let digit = DIGITS[d];
        for dy in 0..DIGIT_HEIGHT {
            for dx in 0..DIGIT_WIDTH {
                let x = origin.x as usize + dx;
                let y = origin.y as usize + dy;
                if digit[dy][dx] > 0 {
                    frame.set(x, y, self.opts.digit_color);
                }
            }
        }
    }
}
