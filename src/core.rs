use crate::vec::Vec2;

#[repr(usize)]
#[derive(Copy, Clone)]
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

    pub fn get_valid_moves(&self, pos: Vec2) -> Vec<Vec2> {
        let piece = self.get(pos).expect("Peice expected at position");

        let mut valid: Vec<Vec2> = Default::default();

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
}

// type ValidMoves = fn(pos: Vec2, player: Player, board: &Board, results: &mut Vec<Vec2>);

fn valid_king_moves(board: &Board, pos: Vec2, piece: &Piece, results: &mut Vec<Vec2>) {
    valid_linear_moves(board, piece.player, pos, Vec2::new(-1, 0), 1, results);
    valid_linear_moves(board, piece.player, pos, Vec2::new(0, -1), 1, results);
    valid_linear_moves(board, piece.player, pos, Vec2::new(1, 0), 1, results);
    valid_linear_moves(board, piece.player, pos, Vec2::new(0, 1), 1, results);
}

fn valid_queen_moves(board: &Board, pos: Vec2, piece: &Piece, results: &mut Vec<Vec2>) {
    valid_bishop_moves(board, pos, piece, results);
    valid_rook_moves(board, pos, piece, results);
}

fn valid_bishop_moves(board: &Board, pos: Vec2, piece: &Piece, results: &mut Vec<Vec2>) {
    valid_linear_moves(board, piece.player, pos, Vec2::new(-1, -1), 8, results);
    valid_linear_moves(board, piece.player, pos, Vec2::new(1, -1), 8, results);
    valid_linear_moves(board, piece.player, pos, Vec2::new(1, 1), 8, results);
    valid_linear_moves(board, piece.player, pos, Vec2::new(-1, 1), 8, results);
}

fn valid_knight_moves(_board: &Board, _pos: Vec2, _piece: &Piece, _results: &mut Vec<Vec2>) {}

fn valid_rook_moves(board: &Board, pos: Vec2, piece: &Piece, results: &mut Vec<Vec2>) {
    valid_linear_moves(board, piece.player, pos, Vec2::new(-1, 0), 8, results);
    valid_linear_moves(board, piece.player, pos, Vec2::new(0, -1), 8, results);
    valid_linear_moves(board, piece.player, pos, Vec2::new(1, 0), 8, results);
    valid_linear_moves(board, piece.player, pos, Vec2::new(0, 1), 8, results);
}

fn valid_pawn_moves(board: &Board, pos: Vec2, piece: &Piece, results: &mut Vec<Vec2>) {
    let dir = match piece.player {
        Player::White => Vec2::UP,
        Player::Black => Vec2::DOWN,
    };

    if piece.move_count == 0 {
        valid_linear_moves(board, piece.player, pos, dir, 2, results);
    } else {
        valid_linear_moves(board, piece.player, pos, dir, 1, results);
    }
}

fn valid_linear_moves(
    board: &Board,
    player: Player,
    pos: Vec2,
    dir: Vec2,
    limit: usize,
    results: &mut Vec<Vec2>,
) {
    let mut test_pos = pos;
    for _ in 0..limit {
        test_pos = test_pos + dir;

        if test_pos.x < 0 || test_pos.x >= 8 || test_pos.y < 0 || test_pos.y >= 8 {
            return;
        }

        if let Some(p) = board.get(test_pos) {
            if player as u32 != p.player as u32 {
                results.push(test_pos);
            }
            break;
        }

        results.push(test_pos);
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

impl Piece {}
