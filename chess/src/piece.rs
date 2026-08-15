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

impl<T> std::ops::Index<Piece> for [T; 6] {
    type Output = T;

    fn index(&self, piece: Piece) -> &T {
        &self[piece as usize]
    }
}

impl<T> std::ops::IndexMut<Piece> for [T; 6] {
    fn index_mut(&mut self, piece: Piece) -> &mut T {
        &mut self[piece as usize]
    }
}
