use crate::{
    bitboard::Bitboard,
    movetype::Move,
    piece::{Piece, PieceSide},
    side::Side,
    square::Square,
};
use std::fmt;

pub struct Irrecoverable {
    pub mv: Option<Move>,
    pub captured: Option<Piece>,
    pub castling: [Option<Square>; 4],
    pub ep: Option<Square>,
    pub halfmoves: usize,
}

#[must_use]
pub struct Position {
    pub(crate) pieces: [Bitboard; 6],
    pub(crate) sides: [Bitboard; 2],
    pub(crate) turn: Side,
    pub castling: [Option<Square>; 4],
    pub(crate) ep: Option<Square>,
    pub(crate) halfmoves: usize,
    pub(crate) fullmoves: usize,
    pub(crate) is_960: bool,
    pub history: Vec<Irrecoverable>,
}

impl Position {
    pub fn new() -> Self {
        Self {
            pieces: [Bitboard::empty(); 6],
            sides: [Bitboard::empty(); 2],
            turn: Side::White,
            castling: [None; 4],
            ep: None,
            halfmoves: 0,
            fullmoves: 0,
            is_960: false,
            history: vec![],
        }
    }

    pub fn startpos() -> Self {
        Self {
            pieces: [
                Bitboard::new(0xff00000000ff00),
                Bitboard::new(0x4200000000000042),
                Bitboard::new(0x2400000000000024),
                Bitboard::new(0x8100000000000081),
                Bitboard::new(0x800000000000008),
                Bitboard::new(0x1000000000000010),
            ],
            sides: [Bitboard::new(0xffff), Bitboard::new(0xffff000000000000)],
            turn: Side::White,
            castling: [
                Some(Square::from_index(7)),
                Some(Square::from_index(0)),
                Some(Square::from_index(63)),
                Some(Square::from_index(56)),
            ],
            ep: None,
            halfmoves: 0,
            fullmoves: 1,
            is_960: false,
            history: vec![],
        }
    }

    #[must_use]
    pub fn turn(&self) -> Side {
        self.turn
    }

    #[must_use]
    pub fn get_piece_side(&self, piece: Piece, side: Side) -> Bitboard {
        self.pieces[piece] & self.sides[side]
    }

    #[must_use]
    pub fn ksq(&self, side: Side) -> Square {
        debug_assert!(self.is_valid());

        let kbb = self.pieces[Piece::King] & self.sides[side];
        kbb.lsb().expect("No king")
    }

    #[must_use]
    pub fn side_on(&self, sq: Square) -> Option<Side> {
        if self.sides[Side::White].is_set(sq) {
            Some(Side::White)
        } else if self.sides[Side::Black].is_set(sq) {
            Some(Side::Black)
        } else {
            None
        }
    }

    #[must_use]
    pub fn piece_on(&self, sq: Square) -> Option<Piece> {
        for piece in [
            Piece::Pawn,
            Piece::Knight,
            Piece::Bishop,
            Piece::Rook,
            Piece::Queen,
            Piece::King,
        ] {
            if self.pieces[piece].is_set(sq) {
                return Some(piece);
            }
        }
        None
    }

    #[must_use]
    pub fn piece_side_on(&self, sq: Square) -> Option<PieceSide> {
        // White
        if (self.sides[Side::White] & self.pieces[Piece::Pawn]).is_set(sq) {
            return Some(PieceSide::WP);
        } else if (self.sides[Side::White] & self.pieces[Piece::Knight]).is_set(sq) {
            return Some(PieceSide::WN);
        } else if (self.sides[Side::White] & self.pieces[Piece::Bishop]).is_set(sq) {
            return Some(PieceSide::WB);
        } else if (self.sides[Side::White] & self.pieces[Piece::Rook]).is_set(sq) {
            return Some(PieceSide::WR);
        } else if (self.sides[Side::White] & self.pieces[Piece::Queen]).is_set(sq) {
            return Some(PieceSide::WQ);
        } else if (self.sides[Side::White] & self.pieces[Piece::King]).is_set(sq) {
            return Some(PieceSide::WK);
        }
        // Black
        else if (self.sides[Side::Black] & self.pieces[Piece::Pawn]).is_set(sq) {
            return Some(PieceSide::BP);
        } else if (self.sides[Side::Black] & self.pieces[Piece::Knight]).is_set(sq) {
            return Some(PieceSide::BN);
        } else if (self.sides[Side::Black] & self.pieces[Piece::Bishop]).is_set(sq) {
            return Some(PieceSide::BB);
        } else if (self.sides[Side::Black] & self.pieces[Piece::Rook]).is_set(sq) {
            return Some(PieceSide::BR);
        } else if (self.sides[Side::Black] & self.pieces[Piece::Queen]).is_set(sq) {
            return Some(PieceSide::BQ);
        } else if (self.sides[Side::Black] & self.pieces[Piece::King]).is_set(sq) {
            return Some(PieceSide::BK);
        }
        // None
        else {
            None
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in (0..8).rev() {
            for x in 0..8 {
                let sq = Square::from_indices(x, y);
                match self.piece_side_on(sq) {
                    Some(PieceSide::WP) => write!(f, "P")?,
                    Some(PieceSide::WN) => write!(f, "N")?,
                    Some(PieceSide::WB) => write!(f, "B")?,
                    Some(PieceSide::WR) => write!(f, "R")?,
                    Some(PieceSide::WQ) => write!(f, "Q")?,
                    Some(PieceSide::WK) => write!(f, "K")?,
                    Some(PieceSide::BP) => write!(f, "p")?,
                    Some(PieceSide::BN) => write!(f, "n")?,
                    Some(PieceSide::BB) => write!(f, "b")?,
                    Some(PieceSide::BR) => write!(f, "r")?,
                    Some(PieceSide::BQ) => write!(f, "q")?,
                    Some(PieceSide::BK) => write!(f, "k")?,
                    None => write!(f, "-")?,
                }
            }

            writeln!(f)?;
        }
        Ok(())
    }
}
