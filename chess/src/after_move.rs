use crate::{movetype::Move, position::Position};

impl Position {
    #[must_use]
    pub fn after_move<const HASH: bool>(&self, mv: &Move) -> Self {
        let mut npos = *self;
        npos.makemove::<HASH>(mv);
        npos
    }
}
