use crate::position::Position;

impl Position {
    #[must_use]
    pub fn after_null<const HASH: bool>(&self) -> Self {
        let mut npos = *self;
        npos.makenull::<HASH>();
        npos
    }
}
