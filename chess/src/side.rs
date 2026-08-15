use std::ops::Not;

use crate::bitboard::Bitboard;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Side {
    White,
    Black,
}

impl Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }
}

impl std::ops::Index<Side> for [Bitboard; 2] {
    type Output = Bitboard;

    fn index(&self, side: Side) -> &Self::Output {
        match side {
            Side::White => &self[0],
            Side::Black => &self[1],
        }
    }
}

impl std::ops::IndexMut<Side> for [Bitboard; 2] {
    fn index_mut(&mut self, side: Side) -> &mut Self::Output {
        match side {
            Side::White => &mut self[0],
            Side::Black => &mut self[1],
        }
    }
}
