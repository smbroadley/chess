use std::fmt::Display;

#[repr(usize)]
#[derive(Copy, Clone, Debug)]
pub enum PieceType {
    King = 0,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Player {
    White,
    Black,
}

pub struct Piece {
    pub ty: PieceType,
    pub player: Player,
    pub move_count: usize,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.ty)
    }
}
