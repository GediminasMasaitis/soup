#[derive(Debug, Clone, Copy)]
pub enum Castling {
    Wksc,
    Wqsc,
    Bksc,
    Bqsc,
}

impl<T> std::ops::Index<Castling> for [T; 4] {
    type Output = T;

    fn index(&self, perm: Castling) -> &T {
        &self[perm as usize]
    }
}

impl<T> std::ops::IndexMut<Castling> for [T; 4] {
    fn index_mut(&mut self, perm: Castling) -> &mut T {
        &mut self[perm as usize]
    }
}
