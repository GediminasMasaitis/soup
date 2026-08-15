use crate::{
    bitboard::Bitboard,
    castling::Castling,
    movetype::Move,
    piece::Piece,
    position::Position,
    side::Side,
    square::Square,
    zobrist::{castling_zobrist, ep_zobrist, piece_zobrist, side_zobrist},
};

impl Position {
    pub fn makemove<const HASH: bool>(&mut self, mv: &Move) {
        debug_assert!(self.is_valid(), "makemove got invalid state");

        let piece = self
            .piece_on(mv.from)
            .expect("makemove - Must be a piece on the from square");

        let captured = self.piece_on(mv.to);

        #[cfg(debug_assertions)]
        let predicted = if HASH { Some(self.predict_hash(mv)) } else { None };

        if HASH && let Some(sq) = self.ep {
            self.zobrist ^= ep_zobrist(sq);
        }
        self.ep = None;

        // Apply castle
        if self.turn == Side::White && Some(mv.to) == self.castling[Castling::Wksc] {
            // Remove the king
            self.pieces[Piece::King] ^= Bitboard::from_square(mv.from);
            self.sides[Side::White] ^= Bitboard::from_square(mv.from);
            if HASH {
                self.zobrist ^= piece_zobrist(Piece::King, Side::White, mv.from);
            }

            // Remove the rook
            self.pieces[Piece::Rook] ^= Bitboard::from_square(mv.to);
            self.sides[Side::White] ^= Bitboard::from_square(mv.to);
            if HASH {
                self.zobrist ^= piece_zobrist(Piece::Rook, Side::White, mv.to);
            }

            // Add the king
            self.pieces[Piece::King] ^= Bitboard::new(0x40);
            self.sides[Side::White] ^= Bitboard::new(0x40);
            if HASH {
                self.zobrist ^= piece_zobrist(Piece::King, Side::White, Square::from_index(6));
            }

            // Add the rook
            self.pieces[Piece::Rook] ^= Bitboard::new(0x20);
            self.sides[Side::White] ^= Bitboard::new(0x20);
            if HASH {
                self.zobrist ^= piece_zobrist(Piece::Rook, Side::White, Square::from_index(5));
            }
        } else if self.turn == Side::White && Some(mv.to) == self.castling[Castling::Wqsc] {
            // Remove the king
            self.pieces[Piece::King] ^= Bitboard::from_square(mv.from);
            self.sides[Side::White] ^= Bitboard::from_square(mv.from);
            if HASH {
                self.zobrist ^= piece_zobrist(Piece::King, Side::White, mv.from);
            }

            // Remove the rook
            self.pieces[Piece::Rook] ^= Bitboard::from_square(mv.to);
            self.sides[Side::White] ^= Bitboard::from_square(mv.to);
            if HASH {
                self.zobrist ^= piece_zobrist(Piece::Rook, Side::White, mv.to);
            }

            // Add the king
            self.pieces[Piece::King] ^= Bitboard::new(0x4);
            self.sides[Side::White] ^= Bitboard::new(0x4);
            if HASH {
                self.zobrist ^= piece_zobrist(Piece::King, Side::White, Square::from_index(2));
            }

            // Add the rook
            self.pieces[Piece::Rook] ^= Bitboard::new(0x8);
            self.sides[Side::White] ^= Bitboard::new(0x8);
            if HASH {
                self.zobrist ^= piece_zobrist(Piece::Rook, Side::White, Square::from_index(3));
            }
        } else if self.turn == Side::Black && Some(mv.to) == self.castling[Castling::Bksc] {
            // Remove the king
            self.pieces[Piece::King] ^= Bitboard::from_square(mv.from);
            self.sides[Side::Black] ^= Bitboard::from_square(mv.from);
            if HASH {
                self.zobrist ^= piece_zobrist(Piece::King, Side::Black, mv.from);
            }

            // Remove the rook
            self.pieces[Piece::Rook] ^= Bitboard::from_square(mv.to);
            self.sides[Side::Black] ^= Bitboard::from_square(mv.to);
            if HASH {
                self.zobrist ^= piece_zobrist(Piece::Rook, Side::Black, mv.to);
            }

            // Add the king
            self.pieces[Piece::King] ^= Bitboard::new(0x4000000000000000);
            self.sides[Side::Black] ^= Bitboard::new(0x4000000000000000);
            if HASH {
                self.zobrist ^= piece_zobrist(Piece::King, Side::Black, Square::from_index(62));
            }

            // Add the rook
            self.pieces[Piece::Rook] ^= Bitboard::new(0x2000000000000000);
            self.sides[Side::Black] ^= Bitboard::new(0x2000000000000000);
            if HASH {
                self.zobrist ^= piece_zobrist(Piece::Rook, Side::Black, Square::from_index(61));
            }
        } else if self.turn == Side::Black && Some(mv.to) == self.castling[Castling::Bqsc] {
            // Remove the king
            self.pieces[Piece::King] ^= Bitboard::from_square(mv.from);
            self.sides[Side::Black] ^= Bitboard::from_square(mv.from);
            if HASH {
                self.zobrist ^= piece_zobrist(Piece::King, Side::Black, mv.from);
            }

            // Remove the rook
            self.pieces[Piece::Rook] ^= Bitboard::from_square(mv.to);
            self.sides[Side::Black] ^= Bitboard::from_square(mv.to);
            if HASH {
                self.zobrist ^= piece_zobrist(Piece::Rook, Side::Black, mv.to);
            }

            // Add the king
            self.pieces[Piece::King] ^= Bitboard::new(0x400000000000000);
            self.sides[Side::Black] ^= Bitboard::new(0x400000000000000);
            if HASH {
                self.zobrist ^= piece_zobrist(Piece::King, Side::Black, Square::from_index(58));
            }

            // Add the rook
            self.pieces[Piece::Rook] ^= Bitboard::new(0x800000000000000);
            self.sides[Side::Black] ^= Bitboard::new(0x800000000000000);
            if HASH {
                self.zobrist ^= piece_zobrist(Piece::Rook, Side::Black, Square::from_index(59));
            }
        }
        // Double pawn
        else if piece == Piece::Pawn && mv.from.rank() == 1 && mv.to.rank() == 3 {
            self.pieces[piece] ^= Bitboard::from_square(mv.from);
            self.sides[self.turn] ^= Bitboard::from_square(mv.from);
            if HASH {
                self.zobrist ^= piece_zobrist(piece, self.turn, mv.from);
            }

            self.pieces[piece] ^= Bitboard::from_square(mv.to);
            self.sides[self.turn] ^= Bitboard::from_square(mv.to);
            if HASH {
                self.zobrist ^= piece_zobrist(piece, self.turn, mv.to);
            }

            self.ep = Some(mv.to.south());
            if HASH {
                self.zobrist ^= ep_zobrist(mv.to.south());
            }
        } else if piece == Piece::Pawn && mv.from.rank() == 6 && mv.to.rank() == 4 {
            self.pieces[piece] ^= Bitboard::from_square(mv.from);
            self.sides[self.turn] ^= Bitboard::from_square(mv.from);
            if HASH {
                self.zobrist ^= piece_zobrist(piece, self.turn, mv.from);
            }

            self.pieces[piece] ^= Bitboard::from_square(mv.to);
            self.sides[self.turn] ^= Bitboard::from_square(mv.to);
            if HASH {
                self.zobrist ^= piece_zobrist(piece, self.turn, mv.to);
            }

            self.ep = Some(mv.to.north());
            if HASH {
                self.zobrist ^= ep_zobrist(mv.to.north());
            }
        }
        // Capture
        else if let Some(captured) = captured {
            // Remove our piece
            self.pieces[piece] ^= Bitboard::from_square(mv.from);
            self.sides[self.turn] ^= Bitboard::from_square(mv.from);
            if HASH {
                self.zobrist ^= piece_zobrist(piece, self.turn, mv.from);
            }

            // Add our piece
            self.pieces[piece] ^= Bitboard::from_square(mv.to);
            self.sides[self.turn] ^= Bitboard::from_square(mv.to);
            if HASH {
                self.zobrist ^= piece_zobrist(piece, self.turn, mv.to);
            }

            // Remove their piece
            self.pieces[captured] ^= Bitboard::from_square(mv.to);
            self.sides[!self.turn] ^= Bitboard::from_square(mv.to);
            if HASH {
                self.zobrist ^= piece_zobrist(captured, !self.turn, mv.to);
            }
        }
        // En passant
        else if piece == Piece::Pawn && mv.from.file() != mv.to.file() && captured.is_none() {
            // Remove our piece
            self.pieces[piece] ^= Bitboard::from_square(mv.from);
            self.sides[self.turn] ^= Bitboard::from_square(mv.from);
            if HASH {
                self.zobrist ^= piece_zobrist(piece, self.turn, mv.from);
            }

            // Add our piece
            self.pieces[piece] ^= Bitboard::from_square(mv.to);
            self.sides[self.turn] ^= Bitboard::from_square(mv.to);
            if HASH {
                self.zobrist ^= piece_zobrist(piece, self.turn, mv.to);
            }

            // Remove their pawn
            if self.turn == Side::White {
                self.pieces[Piece::Pawn] ^= Bitboard::from_square(mv.to.south());
                self.sides[!self.turn] ^= Bitboard::from_square(mv.to.south());
                if HASH {
                    self.zobrist ^= piece_zobrist(Piece::Pawn, !self.turn, mv.to.south());
                }
            } else {
                self.pieces[Piece::Pawn] ^= Bitboard::from_square(mv.to.north());
                self.sides[!self.turn] ^= Bitboard::from_square(mv.to.north());
                if HASH {
                    self.zobrist ^= piece_zobrist(Piece::Pawn, !self.turn, mv.to.north());
                }
            }
        }
        // Normal move
        else {
            self.pieces[piece] ^= Bitboard::from_square(mv.from);
            self.sides[self.turn] ^= Bitboard::from_square(mv.from);
            if HASH {
                self.zobrist ^= piece_zobrist(piece, self.turn, mv.from);
            }

            self.pieces[piece] ^= Bitboard::from_square(mv.to);
            self.sides[self.turn] ^= Bitboard::from_square(mv.to);
            if HASH {
                self.zobrist ^= piece_zobrist(piece, self.turn, mv.to);
            }
        }

        // Add promotion piece
        if let Some(promo) = mv.promo {
            self.pieces[promo] ^= Bitboard::from_square(mv.to);
            self.pieces[Piece::Pawn] ^= Bitboard::from_square(mv.to);
            if HASH {
                self.zobrist ^= piece_zobrist(Piece::Pawn, self.turn, mv.to);
                self.zobrist ^= piece_zobrist(promo, self.turn, mv.to);
            }
        }

        // Castling - white
        if piece == Piece::King && self.turn == Side::White {
            if self.castling[Castling::Wksc].take().is_some() && HASH {
                self.zobrist ^= castling_zobrist(Castling::Wksc);
            }
            if self.castling[Castling::Wqsc].take().is_some() && HASH {
                self.zobrist ^= castling_zobrist(Castling::Wqsc);
            }
        }
        if (Some(mv.from) == self.castling[Castling::Wksc]
            || Some(mv.to) == self.castling[Castling::Wksc])
            && self.castling[Castling::Wksc].take().is_some()
            && HASH
        {
            self.zobrist ^= castling_zobrist(Castling::Wksc);
        }
        if (Some(mv.from) == self.castling[Castling::Wqsc]
            || Some(mv.to) == self.castling[Castling::Wqsc])
            && self.castling[Castling::Wqsc].take().is_some()
            && HASH
        {
            self.zobrist ^= castling_zobrist(Castling::Wqsc);
        }

        // Castling - black
        if piece == Piece::King && self.turn == Side::Black {
            if self.castling[Castling::Bksc].take().is_some() && HASH {
                self.zobrist ^= castling_zobrist(Castling::Bksc);
            }
            if self.castling[Castling::Bqsc].take().is_some() && HASH {
                self.zobrist ^= castling_zobrist(Castling::Bqsc);
            }
        }
        if (Some(mv.from) == self.castling[Castling::Bksc]
            || Some(mv.to) == self.castling[Castling::Bksc])
            && self.castling[Castling::Bksc].take().is_some()
            && HASH
        {
            self.zobrist ^= castling_zobrist(Castling::Bksc);
        }
        if (Some(mv.from) == self.castling[Castling::Bqsc]
            || Some(mv.to) == self.castling[Castling::Bqsc])
            && self.castling[Castling::Bqsc].take().is_some()
            && HASH
        {
            self.zobrist ^= castling_zobrist(Castling::Bqsc);
        }

        if self.turn == Side::Black {
            self.fullmoves += 1;
        }

        self.turn = !self.turn;
        if HASH {
            self.zobrist ^= side_zobrist();
        }

        debug_assert!(self.is_valid(), "makemove left invalid state");
        #[cfg(debug_assertions)]
        if HASH {
            debug_assert_eq!(
                self.zobrist,
                self.calculate_zobrist(),
                "incremental zobrist mismatch after makemove"
            );
            debug_assert_eq!(
                Some(self.zobrist),
                predicted,
                "predict_hash mismatch after makemove"
            );
        }
    }

    pub fn makenull<const HASH: bool>(&mut self) {
        debug_assert!(self.is_valid(), "makenull got invalid state");
        debug_assert!(!self.in_check(), "Can't nullmove while in check");

        if HASH && let Some(sq) = self.ep {
            self.zobrist ^= ep_zobrist(sq);
        }

        self.halfmoves = 0;
        self.ep = None;

        if self.turn == Side::Black {
            self.fullmoves += 1;
        }

        self.turn = !self.turn;
        if HASH {
            self.zobrist ^= side_zobrist();
        }

        debug_assert!(self.is_valid(), "makenull left invalid state");
        #[cfg(debug_assertions)]
        if HASH {
            debug_assert_eq!(
                self.zobrist,
                self.calculate_zobrist(),
                "incremental zobrist mismatch after makenull"
            );
        }
    }

}
