use std::time::Duration;

use crate::{
    core::timer::CountdownTimer,
    core::Vec2,
    core::{Board, MoveResult, Piece, Player},
};

pub enum State {
    Paused,
    Playing,
    Exit,
}

pub enum Mode {
    Selecting,
    Moving(Vec2),
}

pub struct Chess {
    pub turn: Player,
    pub cursor: Vec2,
    pub timers: [CountdownTimer; 2],
    pub board: Board,
    pub mode: Mode,
    pub state: State,
}

impl Chess {
    pub fn change_player(&mut self) {
        self.stop();
        self.turn = if let Player::White = self.turn {
            Player::Black
        } else {
            Player::White
        };
        self.start();
        self.mode = Mode::Selecting;
    }

    pub fn cursor_piece(&self) -> Option<&Piece> {
        self.board.get(self.cursor)
    }

    fn new(board: Board, duration: Duration) -> Self {
        Chess {
            board,
            turn: Player::White,
            cursor: Vec2::new(0, 0),
            timers: [CountdownTimer::new(duration), CountdownTimer::new(duration)],
            mode: Mode::Selecting,
            state: State::Paused,
        }
    }

    pub fn can_move_cursor_piece(&self) -> bool {
        if let Some(p) = self.cursor_piece() {
            if p.player as usize == self.turn as usize {
                if self.board.get_valid_moves(self.cursor, false).len() > 0 {
                    return true;
                }
            }
        }
        false
    }

    pub fn move_cursor(&mut self, dir: Vec2) {
        let pos = self.cursor + dir;
        self.set_cursor(pos)
    }

    pub fn set_cursor(&mut self, pos: Vec2) {
        if pos.x >= 0 && pos.x < 8 && pos.y >= 0 && pos.y < 8 {
            self.cursor = pos;
        }
    }

    pub fn start(&mut self) {
        self.state = State::Playing;
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
            Mode::Moving(from) => {
                let to = self.cursor;
                match self.get_move_result(from, to) {
                    MoveResult::Cancel => {
                        self.mode = Mode::Selecting;
                        return;
                    }
                    MoveResult::Nothing => self.board.move_piece(from, to),
                    MoveResult::Capture(pos) => {
                        self.board.take_piece(pos);
                        self.board.move_piece(from, to);
                    }
                    MoveResult::Castle => todo!(),
                    MoveResult::Promotion(_) => todo!(),
                    MoveResult::Invalid => return,
                }

                self.change_player();
            }
        }
    }

    pub fn get_move_result(&self, from: Vec2, to: Vec2) -> MoveResult {
        if let Some(_) = self.board.get(from) {
            let valid = self.board.get_valid_moves(from, true);

            if let Some(m) = valid.iter().find(|m| m.pos == to) {
                return m.result;
            }
        }
        MoveResult::Invalid
    }

    pub fn quit(&mut self) {
        self.state = State::Exit;
    }
}

impl Default for Chess {
    fn default() -> Self {
        let data = "RNBQKBNR\
                    PPPPPPPP\
                    ........\
                    ........\
                    ........\
                    ........\
                    pppppppp\
                    rnbqkbnr";

        let board = Board::from_string(data);
        let duration = Duration::from_secs(10 * 60);

        Chess::new(board, duration)
    }
}
