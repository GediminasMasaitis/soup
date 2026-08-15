use crate::bitboard::Bitboard;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, PartialEq)]
pub enum PieceSide {
    WP,
    WN,
    WB,
    WR,
    WQ,
    WK,
    BP,
    BN,
    BB,
    BR,
    BQ,
    BK,
}

impl std::ops::Index<Piece> for [Bitboard; 6] {
    type Output = Bitboard;

    fn index(&self, piece: Piece) -> &Self::Output {
        &self[piece as usize]
    }
}

impl std::ops::IndexMut<Piece> for [Bitboard; 6] {
    fn index_mut(&mut self, piece: Piece) -> &mut Self::Output {
        &mut self[piece as usize]
    }
}
