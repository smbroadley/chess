use std::fmt::Display;

use crate::vec::Vec2;

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
}

pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn from_string(data: &str) -> Self {
        let mut iter = data.chars();

        let mut squares: [[Option<Piece>; 8]; 8] = Default::default();

        for y in 0..8 {
            for x in 0..8 {
                let c = iter.next().expect("insufficient data to setup board");
                if c == '.' {
                    continue;
                }

                squares[y][x] = Some(Piece {
                    ty: get_char_type(c),
                    player: get_char_player(c),
                    move_count: 0,
                });
            }
        }

        Self { squares }
    }

    pub fn get(&self, pos: Vec2) -> Option<&Piece> {
        if pos.x < 0 || pos.x > 7 || pos.y < 0 || pos.y > 7 {
            return None;
        }
        self.squares[pos.y as usize][pos.x as usize].as_ref()
    }

    pub fn get_square(&mut self, pos: Vec2) -> Result<&mut Option<Piece>, ()> {
        if pos.x < 0 || pos.x > 7 || pos.y < 0 || pos.y > 7 {
            return Err(());
        }
        Ok(&mut self.squares[pos.y as usize][pos.x as usize])
    }

    pub fn get_mut(&mut self, pos: Vec2) -> Option<&mut Piece> {
        self.squares[pos.y as usize][pos.x as usize].as_mut()
    }

    pub fn swap(&mut self, a: Vec2, b: Vec2) {
        if a == b {
            return;
        }

        let p1 = self.squares[a.y as usize][a.x as usize].take();
        let p2 = self.squares[b.y as usize][b.x as usize].take();

        self.squares[a.y as usize][a.x as usize] = p2;
        self.squares[b.y as usize][b.x as usize] = p1;
    }

    pub fn get_valid_moves(&self, pos: Vec2, inc_cancel: bool) -> Vec<Move> {
        let piece = self.get(pos).expect("Peice expected at position");

        let mut valid: Vec<Move> = Default::default();

        if inc_cancel {
            valid.push(Move::new(pos, MoveResult::Cancel));
        }

        match piece.ty {
            PieceType::King => valid_king_moves(self, pos, piece, &mut valid),
            PieceType::Queen => valid_queen_moves(self, pos, piece, &mut valid),
            PieceType::Bishop => valid_bishop_moves(self, pos, piece, &mut valid),
            PieceType::Knight => valid_knight_moves(self, pos, piece, &mut valid),
            PieceType::Rook => valid_rook_moves(self, pos, piece, &mut valid),
            PieceType::Pawn => valid_pawn_moves(self, pos, piece, &mut valid),
        }

        valid
    }

    pub fn move_piece(&mut self, from: Vec2, to: Vec2) {
        self.get_mut(from).expect("Moves must be valid").move_count += 1;
        self.swap(from, to);
    }

    pub fn take_piece(&mut self, pos: Vec2) {
        self.get_square(pos).unwrap().take();
    }

    pub fn is_vacant(&self, pos: Vec2) -> Result<bool, ()> {
        if pos.x < 0 || pos.x > 7 || pos.y < 0 || pos.y > 7 {
            return Err(());
        }
        Ok(self.get(pos).is_none())
    }

    pub fn is_opponent(&self, pos: Vec2, player: Player) -> Result<bool, ()> {
        if pos.x < 0 || pos.x > 7 || pos.y < 0 || pos.y > 7 {
            return Err(());
        }
        if let Some(p) = self.get(pos) {
            return Ok(p.player != player);
        }
        Ok(false)
    }
}

fn valid_king_moves(board: &Board, pos: Vec2, piece: &Piece, results: &mut Vec<Move>) {
    valid_linear_moves(board, piece.player, pos, Vec2::UP, 1, results);
    valid_linear_moves(board, piece.player, pos, Vec2::DOWN, 1, results);
    valid_linear_moves(board, piece.player, pos, Vec2::LEFT, 1, results);
    valid_linear_moves(board, piece.player, pos, Vec2::RIGHT, 1, results);
}

fn valid_queen_moves(board: &Board, pos: Vec2, piece: &Piece, results: &mut Vec<Move>) {
    valid_bishop_moves(board, pos, piece, results);
    valid_rook_moves(board, pos, piece, results);
}

fn valid_bishop_moves(board: &Board, pos: Vec2, piece: &Piece, results: &mut Vec<Move>) {
    valid_linear_moves(board, piece.player, pos, Vec2::UP_LEFT, 8, results);
    valid_linear_moves(board, piece.player, pos, Vec2::UP_RIGHT, 8, results);
    valid_linear_moves(board, piece.player, pos, Vec2::DOWN_LEFT, 8, results);
    valid_linear_moves(board, piece.player, pos, Vec2::DOWN_RIGHT, 8, results);
}

fn valid_knight_moves(board: &Board, pos: Vec2, piece: &Piece, results: &mut Vec<Move>) {
    for x in [-1i16, -2, 1, 2] {
        for y in [-1i16, -2, 1, 2] {
            if x.unsigned_abs() != y.unsigned_abs() {
                valid_linear_moves(board, piece.player, pos, Vec2::new(x, y), 1, results);
            }
        }
    }
}

fn valid_rook_moves(board: &Board, pos: Vec2, piece: &Piece, results: &mut Vec<Move>) {
    valid_linear_moves(board, piece.player, pos, Vec2::UP, 8, results);
    valid_linear_moves(board, piece.player, pos, Vec2::DOWN, 8, results);
    valid_linear_moves(board, piece.player, pos, Vec2::LEFT, 8, results);
    valid_linear_moves(board, piece.player, pos, Vec2::RIGHT, 8, results);
}

fn valid_pawn_moves(board: &Board, mut pos: Vec2, piece: &Piece, results: &mut Vec<Move>) {
    let dir = match piece.player {
        Player::White => Vec2::UP,
        Player::Black => Vec2::DOWN,
    };

    // capture...
    //
    let cap = pos + dir + Vec2::RIGHT;
    if let Ok(true) = board.is_opponent(cap, piece.player) {
        results.push(Move::new(cap, MoveResult::Capture(cap)))
    }

    let cap = pos + dir + Vec2::LEFT;
    if let Ok(true) = board.is_opponent(cap, piece.player) {
        results.push(Move::new(cap, MoveResult::Capture(cap)))
    }

    // en pass...
    //
    let cap = pos + Vec2::LEFT;
    if let Ok(true) = board.is_vacant(cap + dir) {
        if let Some(Piece {
            ty: PieceType::Pawn,
            player,
            move_count: _,
        }) = board.get(cap)
        {
            if *player != piece.player {
                results.push(Move::new(cap + dir, MoveResult::Capture(cap)));
            }
        }
    }

    let cap = pos + Vec2::RIGHT;
    if let Ok(true) = board.is_vacant(cap + dir) {
        if let Some(Piece {
            ty: PieceType::Pawn,
            player,
            move_count: _,
        }) = board.get(cap)
        {
            if *player != piece.player {
                results.push(Move::new(cap + dir, MoveResult::Capture(cap)));
            }
        }
    }

    // move...
    //
    pos = pos + dir;
    if let Ok(true) = board.is_vacant(pos) {
        results.push(Move::new(pos, MoveResult::Nothing));
    }

    if piece.move_count == 0 {
        pos = pos + dir;
        if let Ok(true) = board.is_vacant(pos) {
            results.push(Move::new(pos, MoveResult::Nothing));
        }
    }
}

fn valid_linear_moves(
    board: &Board,
    player: Player,
    mut pos: Vec2,
    dir: Vec2,
    limit: usize,
    results: &mut Vec<Move>,
) {
    for _ in 0..limit {
        pos = pos + dir;

        if pos.x < 0 || pos.x >= 8 || pos.y < 0 || pos.y >= 8 {
            return;
        }

        if let Some(p) = board.get(pos) {
            if player as u32 != p.player as u32 {
                results.push(Move::new(pos, MoveResult::Capture(pos)));
            }
            break;
        }

        results.push(Move::new(pos, MoveResult::Nothing));
    }
}

fn get_char_player(c: char) -> Player {
    if c.is_lowercase() {
        Player::White
    } else {
        Player::Black
    }
}

fn get_char_type(c: char) -> PieceType {
    let lc = c.to_lowercase().next().unwrap();
    match lc {
        'k' => PieceType::King,
        'q' => PieceType::Queen,
        'b' => PieceType::Bishop,
        'n' => PieceType::Knight,
        'r' => PieceType::Rook,
        'p' => PieceType::Pawn,
        _ => panic!("Chess piece char is not known"),
    }
}
