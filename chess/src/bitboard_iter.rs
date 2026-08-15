use crate::{bitboard::Bitboard, square::Square};

pub struct BitboardIter(u64);

impl IntoIterator for Bitboard {
    type Item = Square;
    type IntoIter = BitboardIter;

    fn into_iter(self) -> Self::IntoIter {
        BitboardIter(self.0)
    }
}

impl Iterator for BitboardIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let idx = self.0.trailing_zeros() as i32;
            let sq = Square::from_index(idx as u8);
            self.0 &= self.0 - 1;
            Some(sq)
        }
    }
}
