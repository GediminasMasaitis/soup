use crate::{
    bitboard::Bitboard, magic, piece::Piece, position::Position, side::Side, square::Square,
};

impl Position {
    #[must_use]
    pub fn is_attacked(&self, sq: Square) -> bool {
        let bb = Bitboard::from_square(sq);
        let occ = self.sides[Side::White] | self.sides[Side::Black];
        let pawns = self.pieces[Piece::Pawn] & self.sides[!self.turn];

        // Pawn
        if self.turn == Side::White {
            if (pawns.se() | pawns.sw()).is_set(sq) {
                return true;
            }
        } else {
            if (pawns.ne() | pawns.nw()).is_set(sq) {
                return true;
            }
        }

        // Knights
        if (Bitboard::knights(sq) & self.pieces[Piece::Knight] & self.sides[!self.turn])
            .is_occupied()
        {
            return true;
        }

        // Bishops
        if (magic::bishop_moves(sq, occ) & self.pieces[Piece::Bishop] & self.sides[!self.turn])
            .is_occupied()
        {
            return true;
        }

        // Rooks
        if (magic::rook_moves(sq, occ) & self.pieces[Piece::Rook] & self.sides[!self.turn])
            .is_occupied()
        {
            return true;
        }

        // Queens
        if (magic::queen_moves(sq, occ) & self.pieces[Piece::Queen] & self.sides[!self.turn])
            .is_occupied()
        {
            return true;
        }

        // Kings
        if (bb.adjacent() & self.pieces[Piece::King] & self.sides[!self.turn]).is_occupied() {
            return true;
        }

        false
    }

    #[must_use]
    pub fn is_attacked2(&self, sq: Square, exclude: Square) -> bool {
        let bb = Bitboard::from_square(sq);
        let occ =
            (self.sides[Side::White] | self.sides[Side::Black]) & !Bitboard::from_square(exclude);
        let pawns = self.pieces[Piece::Pawn] & self.sides[!self.turn];

        // Pawn
        if self.turn == Side::White {
            if (pawns.se() | pawns.sw()).is_set(sq) {
                return true;
            }
        } else {
            if (pawns.ne() | pawns.nw()).is_set(sq) {
                return true;
            }
        }

        // Knights
        if (Bitboard::knights(sq) & self.pieces[Piece::Knight] & self.sides[!self.turn])
            .is_occupied()
        {
            return true;
        }

        // Bishops
        if (magic::bishop_moves(sq, occ) & self.pieces[Piece::Bishop] & self.sides[!self.turn])
            .is_occupied()
        {
            return true;
        }

        // Rooks
        if (magic::rook_moves(sq, occ) & self.pieces[Piece::Rook] & self.sides[!self.turn])
            .is_occupied()
        {
            return true;
        }

        // Queens
        if (magic::queen_moves(sq, occ) & self.pieces[Piece::Queen] & self.sides[!self.turn])
            .is_occupied()
        {
            return true;
        }

        // Kings
        if (bb.adjacent() & self.pieces[Piece::King] & self.sides[!self.turn]).is_occupied() {
            return true;
        }

        false
    }
}
