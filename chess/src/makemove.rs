use crate::{
    bitboard::Bitboard,
    castling::Castling,
    movetype::Move,
    piece::Piece,
    position::{Irrecoverable, Position},
    side::Side,
    square::Square,
};

impl Position {
    pub fn makemove(&mut self, mv: &Move) {
        debug_assert!(self.is_valid(), "makemove got invalid state");

        // debug_assert_eq!(
        //     self.side_on(mv.from),
        //     Some(self.turn),
        //     "Not own piece on move.from"
        // );

        let piece = self
            .piece_on(mv.from)
            .expect("makemove - Must be a piece on the from square");

        let captured = self.piece_on(mv.to);

        // Store irrecoverable information
        self.history.push(Irrecoverable {
            mv: Some(*mv),
            captured,
            castling: self.castling,
            ep: self.ep,
            halfmoves: self.halfmoves,
        });

        self.ep = None;

        // println!("bbb {:?}", piece);
        // println!("bbb {}", mv.from.rank());

        // Apply castle
        if self.turn == Side::White && Some(mv.to) == self.castling[Castling::Wksc] {
            // Remove the king
            self.pieces[Piece::King] ^= Bitboard::from_square(mv.from);
            self.sides[Side::White] ^= Bitboard::from_square(mv.from);

            // Remove the rook
            self.pieces[Piece::Rook] ^= Bitboard::from_square(mv.to);
            self.sides[Side::White] ^= Bitboard::from_square(mv.to);

            // Add the king
            self.pieces[Piece::King] ^= Bitboard::new(0x40);
            self.sides[Side::White] ^= Bitboard::new(0x40);

            // Add the rook
            self.pieces[Piece::Rook] ^= Bitboard::new(0x20);
            self.sides[Side::White] ^= Bitboard::new(0x20);
        } else if self.turn == Side::White && Some(mv.to) == self.castling[Castling::Wqsc] {
            // Remove the king
            self.pieces[Piece::King] ^= Bitboard::from_square(mv.from);
            self.sides[Side::White] ^= Bitboard::from_square(mv.from);

            // Remove the rook
            self.pieces[Piece::Rook] ^= Bitboard::from_square(mv.to);
            self.sides[Side::White] ^= Bitboard::from_square(mv.to);

            // Add the king
            self.pieces[Piece::King] ^= Bitboard::new(0x4);
            self.sides[Side::White] ^= Bitboard::new(0x4);

            // Add the rook
            self.pieces[Piece::Rook] ^= Bitboard::new(0x8);
            self.sides[Side::White] ^= Bitboard::new(0x8);
        } else if self.turn == Side::Black && Some(mv.to) == self.castling[Castling::Bksc] {
            // Remove the king
            self.pieces[Piece::King] ^= Bitboard::from_square(mv.from);
            self.sides[Side::Black] ^= Bitboard::from_square(mv.from);

            // Remove the rook
            self.pieces[Piece::Rook] ^= Bitboard::from_square(mv.to);
            self.sides[Side::Black] ^= Bitboard::from_square(mv.to);

            // Add the king
            self.pieces[Piece::King] ^= Bitboard::new(0x4000000000000000);
            self.sides[Side::Black] ^= Bitboard::new(0x4000000000000000);

            // Add the rook
            self.pieces[Piece::Rook] ^= Bitboard::new(0x2000000000000000);
            self.sides[Side::Black] ^= Bitboard::new(0x2000000000000000);
        } else if self.turn == Side::Black && Some(mv.to) == self.castling[Castling::Bqsc] {
            // Remove the king
            self.pieces[Piece::King] ^= Bitboard::from_square(mv.from);
            self.sides[Side::Black] ^= Bitboard::from_square(mv.from);

            // Remove the rook
            self.pieces[Piece::Rook] ^= Bitboard::from_square(mv.to);
            self.sides[Side::Black] ^= Bitboard::from_square(mv.to);

            // Add the king
            self.pieces[Piece::King] ^= Bitboard::new(0x400000000000000);
            self.sides[Side::Black] ^= Bitboard::new(0x400000000000000);

            // Add the rook
            self.pieces[Piece::Rook] ^= Bitboard::new(0x800000000000000);
            self.sides[Side::Black] ^= Bitboard::new(0x800000000000000);
        }
        // Double pawn
        else if piece == Piece::Pawn && mv.from.rank() == 1 && mv.to.rank() == 3 {
            self.pieces[piece] ^= Bitboard::from_square(mv.from);
            self.sides[self.turn] ^= Bitboard::from_square(mv.from);

            self.pieces[piece] ^= Bitboard::from_square(mv.to);
            self.sides[self.turn] ^= Bitboard::from_square(mv.to);

            self.ep = Some(mv.to.south());
        } else if piece == Piece::Pawn && mv.from.rank() == 6 && mv.to.rank() == 4 {
            self.pieces[piece] ^= Bitboard::from_square(mv.from);
            self.sides[self.turn] ^= Bitboard::from_square(mv.from);

            self.pieces[piece] ^= Bitboard::from_square(mv.to);
            self.sides[self.turn] ^= Bitboard::from_square(mv.to);

            self.ep = Some(mv.to.north());
        }
        // Capture
        else if let Some(captured) = captured {
            // Remove our piece
            self.pieces[piece] ^= Bitboard::from_square(mv.from);
            self.sides[self.turn] ^= Bitboard::from_square(mv.from);

            // Add our piece
            self.pieces[piece] ^= Bitboard::from_square(mv.to);
            self.sides[self.turn] ^= Bitboard::from_square(mv.to);

            // Remove their piece
            self.pieces[captured] ^= Bitboard::from_square(mv.to);
            self.sides[!self.turn] ^= Bitboard::from_square(mv.to);
        }
        // En passant
        else if piece == Piece::Pawn && mv.from.file() != mv.to.file() && captured.is_none() {
            // Remove our piece
            self.pieces[piece] ^= Bitboard::from_square(mv.from);
            self.sides[self.turn] ^= Bitboard::from_square(mv.from);

            // Add our piece
            self.pieces[piece] ^= Bitboard::from_square(mv.to);
            self.sides[self.turn] ^= Bitboard::from_square(mv.to);

            // Remove their pawn
            if self.turn == Side::White {
                self.pieces[Piece::Pawn] ^= Bitboard::from_square(mv.to.south());
                self.sides[!self.turn] ^= Bitboard::from_square(mv.to.south());
            } else {
                self.pieces[Piece::Pawn] ^= Bitboard::from_square(mv.to.north());
                self.sides[!self.turn] ^= Bitboard::from_square(mv.to.north());
            }
        }
        // Normal move
        else {
            self.pieces[piece] ^= Bitboard::from_square(mv.from);
            self.sides[self.turn] ^= Bitboard::from_square(mv.from);

            self.pieces[piece] ^= Bitboard::from_square(mv.to);
            self.sides[self.turn] ^= Bitboard::from_square(mv.to);
        }

        // Add promotion piece
        if let Some(promo) = mv.promo {
            self.pieces[promo] ^= Bitboard::from_square(mv.to);
            self.pieces[Piece::Pawn] ^= Bitboard::from_square(mv.to);
        }

        // Castling - white
        if piece == Piece::King && self.turn == Side::White {
            self.castling[Castling::Wksc] = None;
            self.castling[Castling::Wqsc] = None;
        }
        if Some(mv.from) == self.castling[Castling::Wksc]
            || Some(mv.to) == self.castling[Castling::Wksc]
        {
            self.castling[Castling::Wksc] = None;
        }
        if Some(mv.from) == self.castling[Castling::Wqsc]
            || Some(mv.to) == self.castling[Castling::Wqsc]
        {
            self.castling[Castling::Wqsc] = None;
        }

        // Castling - black
        if piece == Piece::King && self.turn == Side::Black {
            self.castling[Castling::Bksc] = None;
            self.castling[Castling::Bqsc] = None;
        }
        if mv.from == Square::from_index(63) || mv.to == Square::from_index(63) {
            self.castling[Castling::Bksc] = None;
        }
        if mv.from == Square::from_index(56) || mv.to == Square::from_index(56) {
            self.castling[Castling::Bqsc] = None;
        }

        if self.turn == Side::Black {
            self.fullmoves += 1;
        }

        self.turn = !self.turn;

        let errors = self.errors();
        if !errors.is_empty() {
            println!("{}", self);

            println!("Moves:");
            for irr in &self.history {
                if let Some(mv) = irr.mv {
                    println!("- {}", mv.as_uci());
                } else {
                    println!("- 0000");
                }
            }

            println!("White:\n{}", self.sides[Side::White]);
            println!("Black:\n{}", self.sides[Side::Black]);
            println!("Rook:\n{}", self.pieces[Piece::Rook]);
            println!("King:\n{}", self.pieces[Piece::King]);

            println!("Errors:");
            for e in errors {
                println!("- {:?}", e);
            }
        }

        debug_assert!(self.is_valid(), "makemove left invalid state");
    }

    pub fn makenull(&mut self) {
        debug_assert!(self.is_valid(), "makenull got invalid state");
        debug_assert!(!self.in_check(), "Can't nullmove while in check");

        // Store irrecoverable information
        self.history.push(Irrecoverable {
            mv: None,
            captured: None,
            castling: self.castling,
            ep: self.ep,
            halfmoves: self.halfmoves,
        });

        self.halfmoves = 0;
        self.ep = None;

        if self.turn == Side::Black {
            self.fullmoves += 1;
        }

        self.turn = !self.turn;

        debug_assert!(self.is_valid(), "makenull left invalid state");
    }

    pub fn undomove(&mut self) {
        debug_assert!(self.is_valid(), "undomove got invalid state");
        let irr = self.history.pop().expect("Must be something in history");

        self.turn = !self.turn;

        if let Some(mv) = irr.mv {
            let piece = self.piece_on(mv.to);

            // Apply castle
            if self.turn == Side::White && Some(mv.to) == irr.castling[Castling::Wksc] {
                // Remove the rook
                self.pieces[Piece::Rook] ^= Bitboard::new(0x20);
                self.sides[Side::White] ^= Bitboard::new(0x20);

                // Remove the king
                self.pieces[Piece::King] ^= Bitboard::new(0x40);
                self.sides[Side::White] ^= Bitboard::new(0x40);

                // Add the king
                self.pieces[Piece::King] ^= Bitboard::from_square(mv.from);
                self.sides[Side::White] ^= Bitboard::from_square(mv.from);

                // Add the rook
                self.pieces[Piece::Rook] ^= Bitboard::from_square(mv.to);
                self.sides[Side::White] ^= Bitboard::from_square(mv.to);
            } else if self.turn == Side::White && Some(mv.to) == irr.castling[Castling::Wqsc] {
                // Remove the rook
                self.pieces[Piece::Rook] ^= Bitboard::new(0x8);
                self.sides[Side::White] ^= Bitboard::new(0x8);

                // Remove the king
                self.pieces[Piece::King] ^= Bitboard::new(0x4);
                self.sides[Side::White] ^= Bitboard::new(0x4);

                // Add the king
                self.pieces[Piece::King] ^= Bitboard::from_square(mv.from);
                self.sides[Side::White] ^= Bitboard::from_square(mv.from);

                // Add the rook
                self.pieces[Piece::Rook] ^= Bitboard::from_square(mv.to);
                self.sides[Side::White] ^= Bitboard::from_square(mv.to);
            } else if self.turn == Side::Black && Some(mv.to) == irr.castling[Castling::Bksc] {
                // Remove the rook
                self.pieces[Piece::Rook] ^= Bitboard::new(0x2000000000000000);
                self.sides[Side::Black] ^= Bitboard::new(0x2000000000000000);

                // Remove the king
                self.pieces[Piece::King] ^= Bitboard::new(0x4000000000000000);
                self.sides[Side::Black] ^= Bitboard::new(0x4000000000000000);

                // Add the king
                self.pieces[Piece::King] ^= Bitboard::from_square(mv.from);
                self.sides[Side::Black] ^= Bitboard::from_square(mv.from);

                // Add the rook
                self.pieces[Piece::Rook] ^= Bitboard::from_square(mv.to);
                self.sides[Side::Black] ^= Bitboard::from_square(mv.to);
            } else if self.turn == Side::Black && Some(mv.to) == irr.castling[Castling::Bqsc] {
                // Remove the rook
                self.pieces[Piece::Rook] ^= Bitboard::new(0x800000000000000);
                self.sides[Side::Black] ^= Bitboard::new(0x800000000000000);

                // Remove the king
                self.pieces[Piece::King] ^= Bitboard::new(0x400000000000000);
                self.sides[Side::Black] ^= Bitboard::new(0x400000000000000);

                // Add the king
                self.pieces[Piece::King] ^= Bitboard::from_square(mv.from);
                self.sides[Side::Black] ^= Bitboard::from_square(mv.from);

                // Add the rook
                self.pieces[Piece::Rook] ^= Bitboard::from_square(mv.to);
                self.sides[Side::Black] ^= Bitboard::from_square(mv.to);
            }
            // Double pawn
            else if let Some(piece) = piece
                && piece == Piece::Pawn
                && mv.from.rank() == 1
                && mv.from.rank() == 3
            {
                self.pieces[piece] ^= Bitboard::from_square(mv.from);
                self.sides[self.turn] ^= Bitboard::from_square(mv.from);

                self.pieces[piece] ^= Bitboard::from_square(mv.to);
                self.sides[self.turn] ^= Bitboard::from_square(mv.to);

                self.ep = Some(mv.to.south());
            } else if let Some(piece) = piece
                && piece == Piece::Pawn
                && mv.from.rank() == 6
                && mv.from.rank() == 4
            {
                self.pieces[piece] ^= Bitboard::from_square(mv.from);
                self.sides[self.turn] ^= Bitboard::from_square(mv.from);

                self.pieces[piece] ^= Bitboard::from_square(mv.to);
                self.sides[self.turn] ^= Bitboard::from_square(mv.to);

                self.ep = Some(mv.to.north());
            }
            // En passant
            else if let Some(piece) = piece
                && piece == Piece::Pawn
                && mv.from.file() != mv.to.file()
                && irr.captured.is_none()
            {
                self.pieces[piece] ^= Bitboard::from_square(mv.from);
                self.sides[self.turn] ^= Bitboard::from_square(mv.from);

                self.pieces[piece] ^= Bitboard::from_square(mv.to);
                self.sides[self.turn] ^= Bitboard::from_square(mv.to);

                if self.turn == Side::White {
                    self.pieces[Piece::Pawn] ^= Bitboard::from_square(mv.to.south());
                    self.sides[Side::Black] ^= Bitboard::from_square(mv.to.south());
                } else {
                    self.pieces[Piece::Pawn] ^= Bitboard::from_square(mv.to.north());
                    self.sides[Side::White] ^= Bitboard::from_square(mv.to.north());
                }
            }
            // Capture
            else if let Some(captured) = irr.captured {
                let piece = self.piece_on(mv.to).expect("piece on to square");

                // Remove piece
                self.pieces[piece] ^= Bitboard::from_square(mv.to);
                self.sides[self.turn] ^= Bitboard::from_square(mv.to);

                // Restore piece
                self.pieces[piece] ^= Bitboard::from_square(mv.from);
                self.sides[self.turn] ^= Bitboard::from_square(mv.from);

                // Restore captured
                self.pieces[captured] ^= Bitboard::from_square(mv.to);
                self.sides[!self.turn] ^= Bitboard::from_square(mv.to);
            }
            // Normal move
            else {
                let piece = self.piece_on(mv.to).unwrap();

                self.pieces[piece] ^= Bitboard::from_square(mv.to);
                self.sides[self.turn] ^= Bitboard::from_square(mv.to);

                self.pieces[piece] ^= Bitboard::from_square(mv.from);
                self.sides[self.turn] ^= Bitboard::from_square(mv.from);
            }

            // Remove promotion piece
            if let Some(promo) = mv.promo {
                self.pieces[promo] ^= Bitboard::from_square(mv.from);
                self.pieces[Piece::Pawn] ^= Bitboard::from_square(mv.from);
            }
        }

        self.castling = irr.castling;
        self.halfmoves = irr.halfmoves;
        self.ep = irr.ep;

        if self.turn == Side::Black {
            self.fullmoves -= 1;
        }

        let errors = self.errors();
        if !errors.is_empty() {
            println!("{}", self);
            println!("White:\n{}", self.sides[Side::White]);
            println!("Black:\n{}", self.sides[Side::Black]);
            println!("Rook:\n{}", self.pieces[Piece::Rook]);
            println!("King:\n{}", self.pieces[Piece::King]);
            println!("Errors:");
            for e in errors {
                println!("- {:?}", e);
            }
        }

        debug_assert!(self.is_valid(), "undomove left invalid state");
    }
}
