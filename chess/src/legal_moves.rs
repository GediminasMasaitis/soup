use crate::{movetype::Move, position::Position};

impl Position {
    #[must_use]
    pub fn legal_moves(&self) -> Vec<Move> {
        let mut moves = vec![];

        self.move_generator(|mv| {
            moves.push(mv);
        });

        moves
    }
}
