use crate::position::Position;

impl Position {
    #[must_use]
    pub fn legal_ep(&self) -> bool {
        if let Some(sq) = self.ep { true } else { true }
    }
}
