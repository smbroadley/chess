use std::time::Duration;

use crate::{
    core::{Board, Direction, Piece, Player},
    timer::CountdownTimer,
    vec::Vec2,
};

pub enum Mode {
    Selecting,
    Moving(Vec2),
}

pub struct GameState {
    pub turn: Player,
    pub cursor: Vec2,
    pub timers: [CountdownTimer; 2],
    pub board: Board,
    pub mode: Mode,
}

impl GameState {
    pub fn change_player(&mut self) {
        self.stop();
        self.turn = if let Player::White = self.turn {
            Player::Black
        } else {
            Player::White
        };
        self.start();
    }

    pub fn cursor_piece(&self) -> Option<&Piece> {
        self.board.get(self.cursor)
    }

    fn new(board: Board, duration: Duration) -> Self {
        GameState {
            board,
            turn: Player::White,
            cursor: Vec2::new(0, 0),
            timers: [CountdownTimer::new(duration), CountdownTimer::new(duration)],
            mode: Mode::Selecting,
        }
    }

    pub fn can_move_cursor_piece(&self) -> bool {
        if let Some(p) = self.cursor_piece() {
            if p.player as usize == self.turn as usize {
                if self.board.get_valid_moves(self.cursor).len() > 0 {
                    return true;
                }
            }
        }
        false
    }

    pub fn move_cursor(&mut self, d: Direction) {
        let x = self.cursor.x;
        let y = self.cursor.y;

        self.cursor = match d {
            Direction::Up => (x, 0.max(y - 1)),
            Direction::Down => (x, 7.min(y + 1)),
            Direction::Left => (0.max(x - 1), y),
            Direction::Right => (7.min(x + 1), y),
        }
        .into();
    }

    pub fn start(&mut self) {
        self.timers[self.turn as usize].start();
    }

    pub fn stop(&mut self) {
        self.timers[self.turn as usize].stop();
    }

    pub fn action(&mut self) {
        match self.mode {
            Mode::Selecting => {
                if let Some(p) = self.cursor_piece() {
                    if p.player as u32 == self.turn as u32 {
                        self.mode = Mode::Moving(self.cursor);
                    }
                }
            }
            Mode::Moving(pos) => {
                self.mode = Mode::Selecting;
                self.change_player();

                // test move
                //
                self.board.swap(pos, self.cursor);
                self.board.get_mut(self.cursor).unwrap().move_count += 1;
            }
        }
    }

    pub fn is_valid_move(&self, pos: Vec2) -> bool {
        if let Mode::Moving(held) = self.mode {
            let valid = self.board.get_valid_moves(held);

            return valid.contains(&pos);
        }
        false
    }
}

impl Default for GameState {
    fn default() -> Self {
        let data = "RNBQKBNR\
                    PPPPPPPP\
                    ........\
                    ........\
                    ........\
                    ........\
                    pppppppp\
                    rnbkqbnr";

        let board = Board::from_string(data);
        let duration = Duration::from_secs(10 * 60);

        GameState::new(board, duration)
    }
}
