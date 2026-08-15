use crate::{bitboard::Bitboard, magic, piece::Piece, position::Position, side::Side};

impl Position {
    #[must_use]
    pub fn in_check(&self) -> bool {
        debug_assert!(self.is_valid());

        let ksq = self.ksq(self.turn);
        // let kbb = Bitboard::from_square(ksq);
        let blockers = self.sides[Side::White] | self.sides[Side::Black];
        let pawns = self.pieces[Piece::Pawn] & self.sides[!self.turn];

        // Pawns
        if self.turn == Side::White {
            if (pawns.se() | pawns.sw()).is_set(ksq) {
                return true;
            }
        } else {
            if (pawns.ne() | pawns.nw()).is_set(ksq) {
                return true;
            }
        }

        // Knights
        if (Bitboard::knights(ksq) & self.pieces[Piece::Knight] & self.sides[!self.turn])
            .is_occupied()
        {
            return true;
        }

        // Bishops
        if (magic::bishop_moves(ksq, blockers)
            & self.pieces[Piece::Bishop]
            & self.sides[!self.turn])
            .is_occupied()
        {
            return true;
        }

        // Rooks
        if (magic::rook_moves(ksq, blockers) & self.pieces[Piece::Rook] & self.sides[!self.turn])
            .is_occupied()
        {
            return true;
        }

        // Queens
        if (magic::queen_moves(ksq, blockers) & self.pieces[Piece::Queen] & self.sides[!self.turn])
            .is_occupied()
        {
            return true;
        }

        false
    }
}
