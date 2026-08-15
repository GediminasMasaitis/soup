use crate::{
    bitboard::Bitboard, castling::Castling, piece::Piece, position::Position, side::Side,
    square::Square,
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum PositionalError {
    BackrankPawns(Side),
    OverlappingSides,
    OverlappingPiece(Piece, Piece),
    TouchingKings,
    IllegalEP(Square),
    EPIllegalSq(Square),
    CastleNoRook(Castling, Square),
    CastlingKingRank(Square),
    CastlingIllegalKingSquare(Square),
    AdjacentKings,
    TooManyPieces(Side),
    TooFewPiecesKind(Side, Piece, u32),
    TooManyPiecesKind(Side, Piece, u32),
}

impl fmt::Display for PositionalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BackrankPawns(side) => write!(f, "{side:?} has pawns on the back rank"),
            Self::OverlappingSides => write!(f, "white and black pieces overlap"),
            Self::OverlappingPiece(p1, p2) => write!(f, "{p1:?} and {p2:?} overlap"),
            Self::TouchingKings => write!(f, "kings are touching"),
            Self::IllegalEP(sq) => write!(f, "no pawn to capture en passant on {sq}"),
            Self::EPIllegalSq(sq) => write!(f, "en passant square {sq} is invalid"),
            Self::CastleNoRook(perm, sq) => {
                write!(f, "no rook on {sq} for castling right {perm:?}")
            }
            Self::CastlingKingRank(sq) => {
                write!(f, "castling square {sq} is not on the back rank")
            }
            Self::CastlingIllegalKingSquare(sq) => {
                write!(f, "king on {sq} can't castle from that square")
            }
            Self::AdjacentKings => write!(f, "kings are adjacent"),
            Self::TooManyPieces(side) => write!(f, "{side:?} has too many pieces"),
            Self::TooFewPiecesKind(side, piece, count) => {
                write!(f, "{side:?} has too few {piece:?}s ({count})")
            }
            Self::TooManyPiecesKind(side, piece, count) => {
                write!(f, "{side:?} has too many {piece:?}s ({count})")
            }
        }
    }
}

impl std::error::Error for PositionalError {}

#[must_use]
fn pawn_single(side: Side, mask: Bitboard) -> Bitboard {
    match side {
        Side::White => mask.north(),
        Side::Black => mask.south(),
    }
}

#[must_use]
fn get_castling_perms(side: Side) -> [Castling; 2] {
    match side {
        Side::White => [Castling::Wksc, Castling::Wqsc],
        Side::Black => [Castling::Bksc, Castling::Bqsc],
    }
}

#[must_use]
fn backrank_for(side: Side) -> Bitboard {
    match side {
        Side::White => Bitboard::new(0xFF),
        Side::Black => Bitboard::new(0xFF00000000000000),
    }
}

impl Position {
    #[must_use]
    pub fn errors(&self) -> Vec<PositionalError> {
        let mut errors = vec![];

        let pieces = [
            Piece::Pawn,
            Piece::Knight,
            Piece::Bishop,
            Piece::Rook,
            Piece::Queen,
            Piece::King,
        ];

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

        for side in [Side::White, Side::Black] {
            let backrank = backrank_for(side);

            // Too many pieces
            if self.sides[side].count() > 16 {
                errors.push(PositionalError::TooManyPieces(side));
            }

            // Back rank pawns
            if (self.pieces[Piece::Pawn] & backrank).is_occupied() {
                errors.push(PositionalError::BackrankPawns(side));
            }

            // Pieces
            for piece in pieces {
                let bb = self.sides[side] & self.pieces[piece];

                let min = match piece {
                    Piece::King => 1,
                    _ => 0,
                };

                let max = match piece {
                    Piece::Pawn => 8,
                    Piece::Knight => 2 + 8,
                    Piece::Bishop => 2 + 8,
                    Piece::Rook => 2 + 8,
                    Piece::Queen => 1 + 8,
                    Piece::King => 1,
                };

                // Not enough pieces of a given type
                if bb.count() < min {
                    errors.push(PositionalError::TooFewPiecesKind(side, piece, bb.count()));
                }

                // Too many pieces of a given type
                if bb.count() > max {
                    errors.push(PositionalError::TooManyPiecesKind(side, piece, bb.count()));
                }
            }

            // Castling
            for perm in get_castling_perms(side) {
                let kbb = self.sides[side] & self.pieces[Piece::King];
                let Some(rsq) = self.castling[perm] else {
                    continue;
                };
                let Some(ksq) = kbb.lsb() else {
                    continue;
                };

                let rank = match side {
                    Side::White => 0,
                    Side::Black => 7,
                };

                // King has to be on the back rank
                if ksq.rank() != rank {
                    errors.push(PositionalError::CastlingKingRank(rsq));
                }

                // Castling square has to be on the back rank
                if rsq.rank() != rank {
                    errors.push(PositionalError::CastlingKingRank(rsq));
                }

                // The rook has to exist
                if self.piece_on(rsq) != Some(Piece::Rook) || self.side_on(rsq) != Some(side) {
                    errors.push(PositionalError::CastleNoRook(perm, rsq));
                }

                // Illegal setup from the rules
                if ksq.file() == 0 || ksq.file() == 7 {
                    errors.push(PositionalError::CastlingIllegalKingSquare(ksq));
                }
            }
        }

        // En passant
        if let Some(sq) = self.ep {
            let rank = match self.turn {
                Side::White => 5,
                Side::Black => 2,
            };

            let bb = Bitboard::from_square(sq);
            let bb_captured = pawn_single(!self.turn, bb);

            // Missing pawn
            if (bb_captured & self.sides[!self.turn] & self.pieces[Piece::Pawn]).is_empty() {
                errors.push(PositionalError::IllegalEP(sq));
            }

            // Wrong rank
            if sq.rank() != rank {
                errors.push(PositionalError::EPIllegalSq(sq));
            }

            // Blocked EP square
            if self.piece_on(sq).is_some() {
                errors.push(PositionalError::EPIllegalSq(sq));
            }
        }

        // Touching kings
        if (self.pieces[Piece::King].adjacent() & self.pieces[Piece::King]).is_occupied() {
            errors.push(PositionalError::AdjacentKings);
        }

        errors
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.errors().is_empty()
    }
}
