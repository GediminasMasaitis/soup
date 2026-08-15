use crate::{
    bitboard::Bitboard,
    castling::Castling,
    piece::{Piece, PieceSide},
    position::Position,
    side::Side,
    square::Square,
};

#[derive(Debug)]
pub enum PositionalError {
    KingMiscount(Side, u32),
    BackrankPawns,
    OverlappingSides,
    OverlappingPiece(Piece, Piece),
    TouchingKings,
    IllegalEP(Square),
    EPIllegalSq(Square),
    CastleNoRook(Castling, Square),
    CastlingKingRank(Square),
}

impl Position {
    #[must_use]
    pub fn errors(&self) -> Vec<PositionalError> {
        let mut errors = vec![];

        let wkbb = self.pieces[Piece::King] & self.sides[Side::White];
        let bkbb = self.pieces[Piece::King] & self.sides[Side::Black];
        let pieces = [
            Piece::Pawn,
            Piece::Knight,
            Piece::Bishop,
            Piece::Rook,
            Piece::Queen,
            Piece::King,
        ];

        // Kings
        if wkbb.count() != 1 {
            errors.push(PositionalError::KingMiscount(Side::White, wkbb.count()));
        }
        if bkbb.count() != 1 {
            errors.push(PositionalError::KingMiscount(Side::Black, bkbb.count()));
        }

        // Side overlaps
        if (self.sides[Side::White] & self.sides[Side::Black]).is_occupied() {
            errors.push(PositionalError::OverlappingSides);
        }

        // Piece overlaps
        for p1 in 0..6 {
            for p2 in (p1 + 1)..6 {
                if (self.pieces[p1] & self.pieces[p2]).is_occupied() {
                    errors.push(PositionalError::OverlappingPiece(pieces[p1], pieces[p2]));
                }
            }
        }

        // Back rank pawns
        if (self.pieces[Piece::Pawn] & Bitboard(0xFF000000000000FF)).is_occupied() {
            errors.push(PositionalError::BackrankPawns);
        }

        // En passant
        if let Some(sq) = self.ep {
            if self.turn == Side::White {
                // Wrong rank
                if sq.rank() != 5 {
                    errors.push(PositionalError::EPIllegalSq(sq));
                }

                // Missing pawn
                if self.piece_side_on(sq.south()) != Some(PieceSide::BP) {
                    errors.push(PositionalError::IllegalEP(sq));
                }
            } else {
                // Wrong rank
                if sq.rank() != 2 {
                    errors.push(PositionalError::EPIllegalSq(sq));
                }

                // Missing pawn
                if self.piece_side_on(sq.north()) != Some(PieceSide::WP) {
                    errors.push(PositionalError::IllegalEP(sq));
                }
            }
        }

        // Castling
        if let Some(sq) = self.castling[Castling::Wksc] {
            if let Some(ksq) = wkbb.lsb() {
                if ksq.rank() != 0 {
                    errors.push(PositionalError::CastlingKingRank(sq));
                }
            } else {
                errors.push(PositionalError::CastlingKingRank(sq));
            }

            if self.piece_side_on(sq) != Some(PieceSide::WR) {
                errors.push(PositionalError::CastleNoRook(Castling::Wksc, sq));
            }
        }
        if let Some(sq) = self.castling[Castling::Wqsc] {
            if let Some(ksq) = wkbb.lsb() {
                if ksq.rank() != 0 {
                    errors.push(PositionalError::CastlingKingRank(sq));
                }
            } else {
                errors.push(PositionalError::CastlingKingRank(sq));
            }

            if self.piece_side_on(sq) != Some(PieceSide::WR) {
                errors.push(PositionalError::CastleNoRook(Castling::Wqsc, sq));
            }
        }
        if let Some(sq) = self.castling[Castling::Bksc] {
            if let Some(ksq) = bkbb.lsb() {
                if ksq.rank() != 7 {
                    errors.push(PositionalError::CastlingKingRank(sq));
                }
            } else {
                errors.push(PositionalError::CastlingKingRank(sq));
            }

            if self.piece_side_on(sq) != Some(PieceSide::BR) {
                errors.push(PositionalError::CastleNoRook(Castling::Bksc, sq));
            }
        }
        if let Some(sq) = self.castling[Castling::Bqsc] {
            if let Some(ksq) = bkbb.lsb() {
                if ksq.rank() != 7 {
                    errors.push(PositionalError::CastlingKingRank(sq));
                }
            } else {
                errors.push(PositionalError::CastlingKingRank(sq));
            }

            if self.piece_side_on(sq) != Some(PieceSide::BR) {
                errors.push(PositionalError::CastleNoRook(Castling::Bqsc, sq));
            }
        }

        errors
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.errors().is_empty()
    }
}
