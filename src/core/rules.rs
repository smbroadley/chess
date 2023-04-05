use super::{PieceType, Vec2};

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum MoveResult {
    Cancel,
    Nothing,
    Capture(Vec2),
    Castle,
    Promotion(PieceType),
    Invalid,
}

pub struct Move {
    pub pos: Vec2,
    pub result: MoveResult,
}

impl Move {
    pub fn new(pos: Vec2, result: MoveResult) -> Self {
        Self { pos, result }
    }

    pub fn capture(pos: Vec2) -> Self {
        Self {
            pos,
            result: MoveResult::Capture(pos),
        }
    }

    pub fn to(pos: Vec2) -> Self {
        Self {
            pos,
            result: MoveResult::Nothing,
        }
    }

    pub fn null(pos: Vec2) -> Self {
        Self {
            pos,
            result: MoveResult::Cancel,
        }
    }
}
