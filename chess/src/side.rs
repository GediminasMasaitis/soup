use std::ops::Not;

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

impl<T> std::ops::Index<Side> for [T; 2] {
    type Output = T;

    fn index(&self, side: Side) -> &T {
        &self[side as usize]
    }
}

impl<T> std::ops::IndexMut<Side> for [T; 2] {
    fn index_mut(&mut self, side: Side) -> &mut T {
        &mut self[side as usize]
    }
}
