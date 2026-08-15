use crate::{
    bitboard::Bitboard, magic, piece::Piece, position::Position, side::Side, square::Square,
};

#[must_use]
fn pawn_single(side: Side, mask: Bitboard) -> Bitboard {
    match side {
        Side::White => mask.north(),
        Side::Black => mask.south(),
    }
}

#[must_use]
fn pawn_attacks(side: Side, mask: Bitboard) -> Bitboard {
    match side {
        Side::White => mask.ne() | mask.nw(),
        Side::Black => mask.se() | mask.sw(),
    }
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
fn line_through(sq1: Square, sq2: Square, blockers: Bitboard) -> Bitboard {
    for line in [
        Bitboard::ray_n(sq1, blockers),
        Bitboard::ray_s(sq1, blockers),
        Bitboard::ray_e(sq1, blockers),
        Bitboard::ray_w(sq1, blockers),
        Bitboard::ray_ne(sq1, blockers),
        Bitboard::ray_nw(sq1, blockers),
        Bitboard::ray_se(sq1, blockers),
        Bitboard::ray_sw(sq1, blockers),
    ] {
        if line.is_set(sq2) {
            return line | Bitboard::from_square(sq1) | Bitboard::from_square(sq2);
        }
    }
    Bitboard::from_square(sq2)
}

#[must_use]
fn allowed(
    from: Square,
    ksq: Square,
    pinned: Bitboard,
    checkers: Bitboard,
    blockers: Bitboard,
) -> Bitboard {
    match checkers.count() {
        1 if pinned.is_set(from) => Bitboard::empty(),
        1 if !pinned.is_set(from) => {
            line_through(ksq, checkers.lsb().unwrap(), blockers) | checkers
        }
        0 if pinned.is_set(from) => line_through(ksq, from, Bitboard::empty()),
        0 if !pinned.is_set(from) => Bitboard::all(),
        _ => Bitboard::empty(),
    }
}

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

impl Position {
    #[must_use]
    pub fn is_legal_ep(&self) -> bool {
        if let Some(to) = self.ep {
            let ksq = self.ksq(self.turn);
            let occupied = self.sides[Side::White] | self.sides[Side::Black];
            let bb = Bitboard::from_square(to);
            let captured_bb = pawn_single(!self.turn, bb);
            let attacks = pawn_attacks(!self.turn, bb);
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
            let pawns = self.sides[self.turn] & self.pieces[Piece::Pawn];
            let is_ep_pawn_check = checkers == captured_bb;
            let rook_pins = rook_pins(ksq, them_rq, occupied);
            let bishop_pins = bishop_pins(ksq, them_bq, occupied);
            let pinned = rook_pins | bishop_pins;

            for from in attacks & pawns {
                let occupied_after = occupied ^ Bitboard::from_square(from) ^ captured_bb ^ bb;
                let bishop_rays = magic::bishop_moves(ksq, occupied_after)
                    & self.sides[!self.turn]
                    & (self.pieces[Piece::Bishop] | self.pieces[Piece::Queen]);
                let rook_rays = magic::rook_moves(ksq, occupied_after)
                    & self.sides[!self.turn]
                    & (self.pieces[Piece::Rook] | self.pieces[Piece::Queen]);
                let in_check = (bishop_rays | rook_rays).is_occupied();
                let allowed = allowed(from, ksq, pinned, checkers, occupied);

                if (is_ep_pawn_check && !pinned.is_set(from)) || (!in_check && allowed.is_set(to)) {
                    return true;
                }
            }
        }

        false
    }
}
