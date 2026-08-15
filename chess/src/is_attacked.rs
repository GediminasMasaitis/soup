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

        // Bishops, rooks, and queens - magic::queen_moves is bishop_moves | rook_moves, so
        // computing those two once and reusing them for the queen check avoids redoing both
        // magic lookups a second time.
        let bishop_attacks = magic::bishop_moves(sq, occ);
        let rook_attacks = magic::rook_moves(sq, occ);

        if (bishop_attacks & self.pieces[Piece::Bishop] & self.sides[!self.turn]).is_occupied() {
            return true;
        }

        if (rook_attacks & self.pieces[Piece::Rook] & self.sides[!self.turn]).is_occupied() {
            return true;
        }

        if ((bishop_attacks | rook_attacks) & self.pieces[Piece::Queen] & self.sides[!self.turn])
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
    pub fn is_attacked2(&self, sq: Square, exclude: Bitboard) -> bool {
        let bb = Bitboard::from_square(sq);
        let occ = (self.sides[Side::White] | self.sides[Side::Black]) & !exclude;
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

        // Bishops, rooks, and queens - magic::queen_moves is bishop_moves | rook_moves, so
        // computing those two once and reusing them for the queen check avoids redoing both
        // magic lookups a second time.
        let bishop_attacks = magic::bishop_moves(sq, occ);
        let rook_attacks = magic::rook_moves(sq, occ);

        if (bishop_attacks & self.pieces[Piece::Bishop] & self.sides[!self.turn]).is_occupied() {
            return true;
        }

        if (rook_attacks & self.pieces[Piece::Rook] & self.sides[!self.turn]).is_occupied() {
            return true;
        }

        if ((bishop_attacks | rook_attacks) & self.pieces[Piece::Queen] & self.sides[!self.turn])
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
