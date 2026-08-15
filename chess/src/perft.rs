use crate::position::Position;

impl Position {
    #[must_use]
    pub fn perft(&mut self, depth: u8) -> u64 {
        if depth == 0 {
            return 1;
        } else if depth == 1 {
            return self.legal_moves().len() as u64;
        }

        let mut nodes = 0u64;

        for mv in self.legal_moves() {
            self.makemove(&mv);
            nodes += self.perft(depth - 1);
            self.undomove();
        }

        nodes
    }
}
