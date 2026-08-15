use crate::{
    bitboard::Bitboard, magic, piece::Piece, position::Position, side::Side, square::Square,
};

#[derive(Clone, Copy)]
pub(crate) struct AttackInfo {
    pub occupied: Bitboard,
    pub us: Bitboard,
    pub ksq: Square,
    pub kbb: Bitboard,
    pub them_n: Bitboard,
    pub them_bq: Bitboard,
    pub them_rq: Bitboard,
    pub checkers: Bitboard,
    pub checkmask: Bitboard,
    pub pinned: Bitboard,
    pub bishop_pins: Bitboard,
    pub rook_pins: Bitboard,
    pub hpinned: Bitboard,
    pin_lines: [(Bitboard, Bitboard); 4],
    num_pins: usize,
}

impl AttackInfo {
    #[must_use]
    pub(crate) fn allowed(&self, from: Square) -> Bitboard {
        if self.pinned.is_set(from) {
            if self.checkers.is_occupied() {
                Bitboard::empty()
            } else {
                let from_bb = Bitboard::from_square(from);
                self.pin_lines[..self.num_pins]
                    .iter()
                    .find(|(sq, _)| *sq == from_bb)
                    .map_or(Bitboard::empty(), |(_, line)| *line)
            }
        } else {
            self.checkmask
        }
    }
}

impl Position {
    #[must_use]
    pub(crate) fn attack_info(&self) -> AttackInfo {
        let occupied = self.sides[Side::White] | self.sides[Side::Black];
        let us = self.sides[self.turn];
        let ksq = self.ksq(self.turn);
        let kbb = Bitboard::from_square(ksq);

        let them_n = self.pieces[Piece::Knight] & self.sides[!self.turn];
        let them_bq =
            (self.pieces[Piece::Bishop] | self.pieces[Piece::Queen]) & self.sides[!self.turn];
        let them_rq =
            (self.pieces[Piece::Rook] | self.pieces[Piece::Queen]) & self.sides[!self.turn];

        // Checks - pawns and knights
        let mut checkers = if self.turn == Side::White {
            (kbb.ne() | kbb.nw()) & self.pieces[Piece::Pawn] & self.sides[!self.turn]
        } else {
            (kbb.se() | kbb.sw()) & self.pieces[Piece::Pawn] & self.sides[!self.turn]
        };
        checkers |= them_n & Bitboard::knights(ksq);

        let mut bishop_pins = Bitboard::empty();
        let mut rook_pins = Bitboard::empty();
        let mut hpinned = Bitboard::empty();
        let mut pin_lines: [(Bitboard, Bitboard); 4] = [(Bitboard::empty(), Bitboard::empty()); 4];
        let mut num_pins = 0usize;

        let occ_without_us = occupied & !us;

        if them_bq.is_occupied() {
            for pinner in magic::bishop_moves(ksq, occ_without_us) & them_bq {
                let pbb = Bitboard::from_square(pinner);
                let between = magic::bishop_moves(ksq, pbb) & magic::bishop_moves(pinner, kbb);
                let blockers = between & occupied;
                match blockers.count() {
                    0 => checkers |= pbb,
                    1 if (blockers & us).is_occupied() => {
                        bishop_pins |= blockers;
                        pin_lines[num_pins] = (blockers, between | pbb);
                        num_pins += 1;
                    }
                    _ => {}
                }
            }
        }

        if them_rq.is_occupied() {
            for pinner in magic::rook_moves(ksq, occ_without_us) & them_rq {
                let pbb = Bitboard::from_square(pinner);
                let between = magic::rook_moves(ksq, pbb) & magic::rook_moves(pinner, kbb);
                let blockers = between & occupied;
                match blockers.count() {
                    0 => checkers |= pbb,
                    1 if (blockers & us).is_occupied() => {
                        rook_pins |= blockers;
                        if pinner.rank() == ksq.rank() {
                            hpinned |= blockers;
                        }
                        pin_lines[num_pins] = (blockers, between | pbb);
                        num_pins += 1;
                    }
                    _ => {}
                }
            }
        }

        let pinned = bishop_pins | rook_pins;

        let checkmask = match checkers.count() {
            0 => Bitboard::all(),
            1 => {
                let checker_sq = checkers.lsb().unwrap();
                let cbb = Bitboard::from_square(checker_sq);
                if ksq.rank() == checker_sq.rank() || ksq.file() == checker_sq.file() {
                    (magic::rook_moves(ksq, cbb) & magic::rook_moves(checker_sq, kbb)) | checkers
                } else if (checkers & them_bq).is_occupied() {
                    (magic::bishop_moves(ksq, cbb) & magic::bishop_moves(checker_sq, kbb))
                        | checkers
                } else {
                    checkers
                }
            }
            _ => Bitboard::empty(),
        };

        AttackInfo {
            occupied,
            us,
            ksq,
            kbb,
            them_n,
            them_bq,
            them_rq,
            checkers,
            checkmask,
            pinned,
            bishop_pins,
            rook_pins,
            hpinned,
            pin_lines,
            num_pins,
        }
    }
}
