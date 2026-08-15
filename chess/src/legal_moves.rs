use crate::{
    bitboard::Bitboard, castling::Castling, magic, movetype::Move, piece::Piece,
    position::Position, side::Side, square::Square,
};

#[must_use]
fn checkers(
    ksq: Square,
    side: Side,
    p: Bitboard,
    n: Bitboard,
    bq: Bitboard,
    rq: Bitboard,
    occupied: Bitboard,
) -> Bitboard {
    let mut bb = Bitboard::empty();
    let kbb = Bitboard::from_square(ksq);

    bb |= if side == Side::White {
        (kbb.ne() | kbb.nw()) & p
    } else {
        (kbb.se() | kbb.sw()) & p
    };
    bb |= n & Bitboard::knights(ksq);
    bb |= bq & magic::bishop_moves(ksq, occupied);
    bb |= rq & magic::rook_moves(ksq, occupied);

    bb
}

#[must_use]
fn bishop_pins(ksq: Square, bq: Bitboard, occupied: Bitboard) -> Bitboard {
    let mut lines = Bitboard::empty();
    for func in [
        Bitboard::ray_ne,
        Bitboard::ray_nw,
        Bitboard::ray_se,
        Bitboard::ray_sw,
    ] {
        let ray = func(ksq, occupied);
        let xray = func(ksq, occupied & !ray);

        if (bq & ray).is_occupied() {
            lines |= ray;
        } else if (bq & xray).is_occupied() {
            lines |= xray;
        }
    }
    lines
}

#[must_use]
fn rook_pins(ksq: Square, rq: Bitboard, occupied: Bitboard) -> Bitboard {
    let mut lines = Bitboard::empty();
    for func in [
        Bitboard::ray_n,
        Bitboard::ray_s,
        Bitboard::ray_e,
        Bitboard::ray_w,
    ] {
        let ray = func(ksq, occupied);
        let xray = func(ksq, occupied & !ray);

        if (rq & ray).is_occupied() {
            lines |= ray;
        } else if (rq & xray).is_occupied() {
            lines |= xray;
        }
    }
    lines
}

#[must_use]
fn get_allowed(pos: &Position) -> Bitboard {
    let ksq = pos.ksq(pos.turn);
    let kbb = Bitboard::from_square(ksq);
    let occupied = pos.sides[Side::White] | pos.sides[Side::Black];

    let them_p = pos.pieces[Piece::Pawn] & pos.sides[!pos.turn];
    let them_n = pos.pieces[Piece::Knight] & pos.sides[!pos.turn];
    let them_bq = (pos.pieces[Piece::Bishop] | pos.pieces[Piece::Queen]) & pos.sides[!pos.turn];
    let them_rq = (pos.pieces[Piece::Rook] | pos.pieces[Piece::Queen]) & pos.sides[!pos.turn];

    // Checkers
    let checkers = {
        let mut bb = Bitboard::empty();
        bb |= if pos.turn() == Side::White {
            (kbb.ne() | kbb.nw()) & them_p
        } else {
            (kbb.se() | kbb.sw()) & them_p
        };
        bb |= them_n & Bitboard::knights(ksq);
        bb |= them_bq & magic::bishop_moves(ksq, occupied);
        bb |= them_rq & magic::rook_moves(ksq, occupied);
        bb
    };

    // Can't move anywhere if there are multiple checkers
    if checkers.count() > 1 {
        return Bitboard::empty();
    }

    // We can go anywhere if there are no checkers
    if checkers.is_empty() {
        return Bitboard::all();
    }

    // If there's one checker, then we can either capture it, or move to block if it's a bishop, rook, or queen

    // No pinning lines with pawns or knights
    if (checkers & (them_p | them_n)).is_occupied() {
        return checkers;
    }

    // Bishop pins
    for func in [
        Bitboard::ray_ne,
        Bitboard::ray_nw,
        Bitboard::ray_se,
        Bitboard::ray_sw,
    ] {
        let ray = func(ksq, occupied);
        if (them_bq & ray).is_occupied() {
            return ray;
        }
    }

    // Rook pins
    for func in [
        Bitboard::ray_n,
        Bitboard::ray_s,
        Bitboard::ray_e,
        Bitboard::ray_w,
    ] {
        let ray = func(ksq, occupied);
        if (them_rq & ray).is_occupied() {
            return ray;
        }
    }

    Bitboard::all()
}

impl Position {
    #[must_use]
    pub fn legal_moves(&self) -> Vec<Move> {
        debug_assert!(self.is_valid());

        let mut moves = vec![];
        let occupied = self.sides[Side::White] | self.sides[Side::Black];
        let empty = !occupied;
        let ksq = self.ksq(self.turn);
        let kbb = Bitboard::from_square(ksq);
        let pawns = self.pieces[Piece::Pawn] & self.sides[self.turn];

        let them_n = self.pieces[Piece::Knight] & self.sides[!self.turn];
        let them_bq =
            (self.pieces[Piece::Bishop] | self.pieces[Piece::Queen]) & self.sides[!self.turn];
        let them_rq =
            (self.pieces[Piece::Rook] | self.pieces[Piece::Queen]) & self.sides[!self.turn];

        let checkers = checkers(
            ksq,
            self.turn,
            self.pieces[Piece::Pawn] & self.sides[!self.turn],
            them_n,
            them_bq,
            them_rq,
            occupied,
        );
        let rook_pins = rook_pins(ksq, them_rq, occupied);
        let bishop_pins = bishop_pins(ksq, them_bq, occupied);
        let pins = rook_pins | bishop_pins;
        let num_checkers = checkers.count();

        let hpinned = Bitboard::rank_of(ksq) & rook_pins;
        let ne_sw_pinned = Bitboard::ne_sw_of(ksq) & bishop_pins;
        let nw_se_pinned = Bitboard::nw_se_of(ksq) & bishop_pins;

        let allowed = get_allowed(&self);

        if self.turn == Side::White {
            // Pawns - singles
            for to in ((pawns & !bishop_pins & !hpinned) << 8) & empty & allowed {
                if to.rank() == 7 {
                    moves.push(Move {
                        from: Square::from_index(to.0 - 8),
                        to,
                        promo: Some(Piece::Queen),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 - 8),
                        to,
                        promo: Some(Piece::Rook),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 - 8),
                        to,
                        promo: Some(Piece::Bishop),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 - 8),
                        to,
                        promo: Some(Piece::Knight),
                    });
                } else {
                    moves.push(Move {
                        from: Square::from_index(to.0 - 8),
                        to,
                        promo: None,
                    });
                }
            }

            // Pawns - doubles
            for to in (((((pawns & !bishop_pins & !hpinned) & Bitboard::new(0xFF00)) << 8) & empty)
                << 8)
                & empty
                & allowed
            {
                moves.push(Move {
                    from: Square::from_index(to.0 - 16),
                    to,
                    promo: None,
                });
            }

            // En passant
            if let Some(ep) = self.ep {
                let bb = Bitboard::from_square(ep);
                let us_p = self.sides[Side::White] & self.pieces[Piece::Pawn];
                let us_k = self.sides[Side::White] & self.pieces[Piece::King];

                // Capture NE
                if (us_p.ne()
                    & bb
                    & !nw_se_pinned.ne()
                    & !rook_pins.ne()
                    & (allowed | allowed.north()))
                .is_occupied()
                {
                    let blockers =
                        (self.sides[Side::White] | self.sides[Side::Black]) ^ bb.south() ^ bb.sw();
                    let ray = Bitboard::ray_ew(ep.south(), blockers);
                    let rank_has_rook = (ray & them_rq).is_occupied();
                    let rank_has_king = (ray & us_k).is_occupied();
                    if !rank_has_rook || !rank_has_king {
                        moves.push(Move {
                            from: Square::from_index(ep.0 - 9),
                            to: ep,
                            promo: None,
                        });
                    }
                }

                // Capture NW
                if (us_p.nw()
                    & bb
                    & !ne_sw_pinned.nw()
                    & !rook_pins.nw()
                    & (allowed | allowed.north()))
                .is_occupied()
                {
                    let blockers =
                        (self.sides[Side::White] | self.sides[Side::Black]) ^ bb.south() ^ bb.se();
                    let ray = Bitboard::ray_ew(ep.south(), blockers);
                    let rank_has_rook = (ray & them_rq).is_occupied();
                    let rank_has_king = (ray & us_k).is_occupied();
                    if !rank_has_rook || !rank_has_king {
                        moves.push(Move {
                            from: Square::from_index(ep.0 - 7),
                            to: ep,
                            promo: None,
                        });
                    }
                }
            }

            // Capture NE
            for to in (pawns & !pins).ne() & (self.sides[!self.turn]) & allowed {
                if to.rank() == 7 {
                    moves.push(Move {
                        from: Square::from_index(to.0 - 9),
                        to,
                        promo: Some(Piece::Queen),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 - 9),
                        to,
                        promo: Some(Piece::Rook),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 - 9),
                        to,
                        promo: Some(Piece::Bishop),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 - 9),
                        to,
                        promo: Some(Piece::Knight),
                    });
                } else {
                    moves.push(Move {
                        from: Square::from_index(to.0 - 9),
                        to,
                        promo: None,
                    });
                }
            }
            for to in (pawns & bishop_pins).ne() & self.sides[!self.turn] & allowed & bishop_pins {
                if to.rank() == 7 {
                    moves.push(Move {
                        from: Square::from_index(to.0 - 9),
                        to,
                        promo: Some(Piece::Queen),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 - 9),
                        to,
                        promo: Some(Piece::Rook),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 - 9),
                        to,
                        promo: Some(Piece::Bishop),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 - 9),
                        to,
                        promo: Some(Piece::Knight),
                    });
                } else {
                    moves.push(Move {
                        from: Square::from_index(to.0 - 9),
                        to,
                        promo: None,
                    });
                }
            }

            // Capture NW
            for to in (pawns & !pins).nw() & (self.sides[!self.turn]) & allowed {
                if to.rank() == 7 {
                    moves.push(Move {
                        from: Square::from_index(to.0 - 7),
                        to,
                        promo: Some(Piece::Queen),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 - 7),
                        to,
                        promo: Some(Piece::Rook),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 - 7),
                        to,
                        promo: Some(Piece::Bishop),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 - 7),
                        to,
                        promo: Some(Piece::Knight),
                    });
                } else {
                    moves.push(Move {
                        from: Square::from_index(to.0 - 7),
                        to,
                        promo: None,
                    });
                }
            }
            for to in (pawns & bishop_pins).nw() & self.sides[!self.turn] & allowed & bishop_pins {
                if to.rank() == 7 {
                    moves.push(Move {
                        from: Square::from_index(to.0 - 7),
                        to,
                        promo: Some(Piece::Queen),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 - 7),
                        to,
                        promo: Some(Piece::Rook),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 - 7),
                        to,
                        promo: Some(Piece::Bishop),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 - 7),
                        to,
                        promo: Some(Piece::Knight),
                    });
                } else {
                    moves.push(Move {
                        from: Square::from_index(to.0 - 7),
                        to,
                        promo: None,
                    });
                }
            }
        } else {
            // Pawns - singles
            for to in ((pawns & !bishop_pins & !hpinned) >> 8) & empty & allowed {
                if to.rank() == 0 {
                    moves.push(Move {
                        from: Square::from_index(to.0 + 8),
                        to,
                        promo: Some(Piece::Queen),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 + 8),
                        to,
                        promo: Some(Piece::Rook),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 + 8),
                        to,
                        promo: Some(Piece::Bishop),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 + 8),
                        to,
                        promo: Some(Piece::Knight),
                    });
                } else {
                    moves.push(Move {
                        from: Square::from_index(to.0 + 8),
                        to,
                        promo: None,
                    });
                }
            }

            // Pawns - doubles
            for to in ((((pawns & !bishop_pins & !hpinned & Bitboard::new(0xff000000000000)) >> 8)
                & empty)
                >> 8)
                & allowed
                & empty
            {
                moves.push(Move {
                    from: Square::from_index(to.0 + 16),
                    to,
                    promo: None,
                });
            }

            // En passant
            if let Some(ep) = self.ep {
                let bb = Bitboard::from_square(ep);
                let us_p = self.sides[Side::Black] & self.pieces[Piece::Pawn];
                let us_k = self.sides[Side::Black] & self.pieces[Piece::King];

                // Capture SE
                if (us_p.se() & bb & !ne_sw_pinned.se() & !rook_pins.se() & allowed.south())
                    .is_occupied()
                {
                    let blockers =
                        (self.sides[Side::White] | self.sides[Side::Black]) ^ bb.north() ^ bb.nw();
                    let ray = Bitboard::ray_ew(ep.north(), blockers);
                    let rank_has_rook = (ray & them_rq).is_occupied();
                    let rank_has_king = (ray & us_k).is_occupied();
                    if !rank_has_rook || !rank_has_king {
                        moves.push(Move {
                            from: Square::from_index(ep.0 + 7),
                            to: ep,
                            promo: None,
                        });
                    }
                }

                // Capture SW
                if (us_p.sw() & bb & !nw_se_pinned.sw() & !rook_pins.sw() & allowed.south())
                    .is_occupied()
                {
                    let blockers =
                        (self.sides[Side::White] | self.sides[Side::Black]) ^ bb.north() ^ bb.ne();
                    let ray = Bitboard::ray_ew(ep.north(), blockers);
                    let rank_has_rook = (ray & them_rq).is_occupied();
                    let rank_has_king = (ray & us_k).is_occupied();
                    if !rank_has_rook || !rank_has_king {
                        moves.push(Move {
                            from: Square::from_index(ep.0 + 9),
                            to: ep,
                            promo: None,
                        });
                    }
                }
            }

            // Capture SE
            for to in (pawns & !pins).se() & self.sides[!self.turn] & allowed {
                if to.rank() == 0 {
                    moves.push(Move {
                        from: Square::from_index(to.0 + 7),
                        to,
                        promo: Some(Piece::Queen),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 + 7),
                        to,
                        promo: Some(Piece::Rook),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 + 7),
                        to,
                        promo: Some(Piece::Bishop),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 + 7),
                        to,
                        promo: Some(Piece::Knight),
                    });
                } else {
                    moves.push(Move {
                        from: Square::from_index(to.0 + 7),
                        to,
                        promo: None,
                    });
                }
            }
            for to in (pawns & bishop_pins).se() & self.sides[!self.turn] & allowed & bishop_pins {
                if to.rank() == 0 {
                    moves.push(Move {
                        from: Square::from_index(to.0 + 7),
                        to,
                        promo: Some(Piece::Queen),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 + 7),
                        to,
                        promo: Some(Piece::Rook),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 + 7),
                        to,
                        promo: Some(Piece::Bishop),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 + 7),
                        to,
                        promo: Some(Piece::Knight),
                    });
                } else {
                    moves.push(Move {
                        from: Square::from_index(to.0 + 7),
                        to,
                        promo: None,
                    });
                }
            }

            // Capture SW
            for to in (pawns & !pins).sw() & self.sides[!self.turn] & allowed {
                if to.rank() == 0 {
                    moves.push(Move {
                        from: Square::from_index(to.0 + 9),
                        to,
                        promo: Some(Piece::Queen),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 + 9),
                        to,
                        promo: Some(Piece::Rook),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 + 9),
                        to,
                        promo: Some(Piece::Bishop),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 + 9),
                        to,
                        promo: Some(Piece::Knight),
                    });
                } else {
                    moves.push(Move {
                        from: Square::from_index(to.0 + 9),
                        to,
                        promo: None,
                    });
                }
            }
            for to in (pawns & bishop_pins).sw() & self.sides[!self.turn] & allowed & bishop_pins {
                if to.rank() == 0 {
                    moves.push(Move {
                        from: Square::from_index(to.0 + 9),
                        to,
                        promo: Some(Piece::Queen),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 + 9),
                        to,
                        promo: Some(Piece::Rook),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 + 9),
                        to,
                        promo: Some(Piece::Bishop),
                    });
                    moves.push(Move {
                        from: Square::from_index(to.0 + 9),
                        to,
                        promo: Some(Piece::Knight),
                    });
                } else {
                    moves.push(Move {
                        from: Square::from_index(to.0 + 9),
                        to,
                        promo: None,
                    });
                }
            }
        }

        // Knights
        for from in self.pieces[Piece::Knight] & self.sides[self.turn] & !pins {
            for to in Bitboard::knights(from) & !self.sides[self.turn] & (allowed | pins) & allowed
            {
                moves.push(Move {
                    from,
                    to,
                    promo: None,
                });
            }
        }

        // Bishops - bishop pinned
        for from in self.pieces[Piece::Bishop] & self.sides[self.turn] & bishop_pins {
            for to in
                magic::bishop_moves(from, occupied) & !self.sides[self.turn] & bishop_pins & allowed
            {
                moves.push(Move {
                    from,
                    to,
                    promo: None,
                });
            }
        }
        // Bishops - not pinned
        for from in self.pieces[Piece::Bishop] & self.sides[self.turn] & !pins {
            for to in magic::bishop_moves(from, occupied)
                & !self.sides[self.turn]
                & (allowed | pins)
                & allowed
            {
                moves.push(Move {
                    from,
                    to,
                    promo: None,
                });
            }
        }

        // Rooks - rook pinned
        for from in self.pieces[Piece::Rook] & self.sides[self.turn] & rook_pins {
            for to in
                magic::rook_moves(from, occupied) & !self.sides[self.turn] & rook_pins & allowed
            {
                moves.push(Move {
                    from,
                    to,
                    promo: None,
                });
            }
        }
        // Rooks - not pinned
        for from in self.pieces[Piece::Rook] & self.sides[self.turn] & !pins {
            for to in magic::rook_moves(from, occupied)
                & !self.sides[self.turn]
                & (allowed | pins)
                & allowed
            {
                moves.push(Move {
                    from,
                    to,
                    promo: None,
                });
            }
        }

        // Queens - bishop pinned
        for from in self.pieces[Piece::Queen] & self.sides[self.turn] & bishop_pins {
            for to in
                magic::bishop_moves(from, occupied) & !self.sides[self.turn] & bishop_pins & allowed
            {
                moves.push(Move {
                    from,
                    to,
                    promo: None,
                });
            }
        }
        // Queens - rook pinned
        for from in self.pieces[Piece::Queen] & self.sides[self.turn] & rook_pins {
            for to in
                magic::rook_moves(from, occupied) & !self.sides[self.turn] & rook_pins & allowed
            {
                moves.push(Move {
                    from,
                    to,
                    promo: None,
                });
            }
        }
        // Queens - not pinned
        for from in self.pieces[Piece::Queen] & self.sides[self.turn] & !pins {
            for to in magic::queen_moves(from, occupied) & !self.sides[self.turn] & allowed {
                moves.push(Move {
                    from,
                    to,
                    promo: None,
                });
            }
        }

        // King
        for to in kbb.adjacent() & !self.sides[self.turn] {
            if self.is_attacked2(to, ksq) {
                continue;
            }

            moves.push(Move {
                from: ksq,
                to,
                promo: None,
            });
        }

        // Castling
        if self.turn == Side::White && num_checkers == 0 {
            if num_checkers == 0
                && self.castling[Castling::Wksc].is_some()
                && empty.is_set(Square::from_index(5))
                && empty.is_set(Square::from_index(6))
                && !self.is_attacked(Square::from_index(5))
                && !self.is_attacked(Square::from_index(6))
            {
                moves.push(Move {
                    from: ksq,
                    to: self.castling[Castling::Wksc].unwrap(),
                    promo: None,
                });
            }

            if self.castling[Castling::Wqsc].is_some()
                && empty.is_set(Square::from_index(3))
                && empty.is_set(Square::from_index(2))
                && empty.is_set(Square::from_index(1))
                && !self.is_attacked(Square::from_index(3))
                && !self.is_attacked(Square::from_index(2))
            {
                moves.push(Move {
                    from: ksq,
                    to: self.castling[Castling::Wqsc].unwrap(),
                    promo: None,
                });
            }
        }

        if self.turn == Side::Black && num_checkers == 0 {
            if self.castling[Castling::Bksc].is_some()
                && empty.is_set(Square::from_index(61))
                && empty.is_set(Square::from_index(62))
                && !self.is_attacked(Square::from_index(61))
                && !self.is_attacked(Square::from_index(62))
            {
                moves.push(Move {
                    from: ksq,
                    to: self.castling[Castling::Bksc].unwrap(),
                    promo: None,
                });
            }

            if self.castling[Castling::Bqsc].is_some()
                && empty.is_set(Square::from_index(59))
                && empty.is_set(Square::from_index(58))
                && empty.is_set(Square::from_index(57))
                && !self.is_attacked(Square::from_index(59))
                && !self.is_attacked(Square::from_index(58))
            {
                moves.push(Move {
                    from: ksq,
                    to: self.castling[Castling::Bqsc].unwrap(),
                    promo: None,
                });
            }
        }

        moves
    }
}
