use crate::{
    attack_info::AttackInfo,
    bitboard::Bitboard,
    castling::Castling,
    magic,
    move_generator::{castle_path, pawn_attacks, pawn_double_rank, pawn_single},
    piece::Piece,
    position::Position,
    side::Side,
    square::Square,
};

#[must_use]
fn pawn_promo_mask(side: Side) -> Bitboard {
    match side {
        Side::White => Bitboard::new(0xFF00000000000000),
        Side::Black => Bitboard::new(0xFF),
    }
}

impl Position {
    #[must_use]
    pub fn count_moves(&self) -> u64 {
        debug_assert!(self.is_valid());

        let info = self.attack_info();
        let AttackInfo {
            occupied,
            us,
            ksq,
            kbb,
            them_bq,
            them_rq,
            checkers,
            checkmask,
            pinned,
            bishop_pins,
            rook_pins,
            hpinned,
            ..
        } = info;
        let them = self.sides[!self.turn];
        let empty = !occupied;

        let mut count = 0u64;

        // King
        for to in kbb.adjacent() & !us {
            if !self.is_attacked2(to, kbb) {
                count += 1;
            }
        }

        // Multiple checks - only the king can move
        if checkers.count() > 1 {
            return count;
        }

        // A promotion is four moves instead of one
        let promo_mask = pawn_promo_mask(self.turn);
        let count_pawn_moves = |bb: Bitboard| (bb.count() + 3 * (bb & promo_mask).count()) as u64;

        // Pawns - unpinned, counted as whole masks
        let pawn_rank_double = pawn_double_rank(self.turn);
        let pawns = self.pieces[Piece::Pawn] & us;
        let free = pawns & !pinned;
        let single = pawn_single(self.turn, free) & empty;
        let double = pawn_single(self.turn, single) & empty & pawn_rank_double;
        let (attacks_a, attacks_b) = match self.turn {
            Side::White => (free.ne(), free.nw()),
            Side::Black => (free.se(), free.sw()),
        };

        count += count_pawn_moves(single & checkmask);
        count += count_pawn_moves(attacks_a & them & checkmask);
        count += count_pawn_moves(attacks_b & them & checkmask);
        count += (double & checkmask).count() as u64;

        // Pawns - pinned, one square at a time
        for from in pawns & pinned {
            let bb = Bitboard::from_square(from);
            let single = pawn_single(self.turn, bb & !bishop_pins & !hpinned) & empty;
            let double = pawn_single(self.turn, single) & empty & pawn_rank_double;
            let attacks = pawn_attacks(self.turn, bb & !rook_pins) & them;

            count += count_pawn_moves((single | double | attacks) & info.allowed(from));
        }

        // Pawns - en passant
        if let Some(to) = self.ep {
            let bb = Bitboard::from_square(to);
            let captured_bb = pawn_single(!self.turn, bb);
            let attacks = pawn_attacks(!self.turn, bb);
            let is_ep_pawn_check = checkers == captured_bb;

            for from in attacks & pawns {
                let occupied_after = occupied ^ Bitboard::from_square(from) ^ captured_bb ^ bb;
                let bishop_rays = magic::bishop_moves(ksq, occupied_after) & them_bq;
                let rook_rays = magic::rook_moves(ksq, occupied_after) & them_rq;
                let in_check = (bishop_rays | rook_rays).is_occupied();

                if (is_ep_pawn_check && !pinned.is_set(from))
                    || (!in_check && info.allowed(from).is_set(to))
                {
                    count += 1;
                }
            }
        }

        // Knights
        for from in self.pieces[Piece::Knight] & us & !pinned {
            count += (Bitboard::knights(from) & !us & checkmask).count() as u64;
        }

        // Bishops
        for from in self.pieces[Piece::Bishop] & us {
            count +=
                (magic::bishop_moves(from, occupied) & !us & info.allowed(from)).count() as u64;
        }

        // Rooks
        for from in self.pieces[Piece::Rook] & us {
            count += (magic::rook_moves(from, occupied) & !us & info.allowed(from)).count() as u64;
        }

        // Queens
        for from in self.pieces[Piece::Queen] & us {
            count +=
                (magic::queen_moves(from, occupied) & !us & info.allowed(from)).count() as u64;
        }

        // Castling
        if checkers.is_empty() {
            let stuff = match self.turn {
                Side::White => [
                    (Castling::Wksc, Square::from_index(6), Square::from_index(5)),
                    (Castling::Wqsc, Square::from_index(2), Square::from_index(3)),
                ],
                Side::Black => [
                    (
                        Castling::Bksc,
                        Square::from_index(62),
                        Square::from_index(61),
                    ),
                    (
                        Castling::Bqsc,
                        Square::from_index(58),
                        Square::from_index(59),
                    ),
                ],
            };

            for (perm, kdest, rdest) in stuff {
                let Some(rsq) = self.castling[perm] else {
                    continue;
                };

                let blockers = occupied ^ kbb ^ Bitboard::from_square(rsq);
                let king_path = castle_path(ksq, kdest);
                let rook_path = castle_path(rsq, rdest);

                if (king_path & blockers).is_empty()
                    && (rook_path & blockers).is_empty()
                    && !king_path
                        .into_iter()
                        .any(|sq| self.is_attacked2(sq, !blockers))
                {
                    count += 1;
                }
            }
        }

        count
    }
}
