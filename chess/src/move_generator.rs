use crate::castling::Castling;
use crate::{
    attack_info::AttackInfo, bitboard::Bitboard, magic, movetype::Move, piece::Piece,
    position::Position, side::Side, square::Square,
};

#[must_use]
pub(crate) fn pawn_single(side: Side, mask: Bitboard) -> Bitboard {
    match side {
        Side::White => mask.north(),
        Side::Black => mask.south(),
    }
}

#[must_use]
pub(crate) fn pawn_double_rank(side: Side) -> Bitboard {
    match side {
        Side::White => Bitboard::new(0xFF000000),
        Side::Black => Bitboard::new(0xFF00000000),
    }
}

#[must_use]
fn pawn_promo_rank(side: Side) -> u8 {
    match side {
        Side::White => 7,
        Side::Black => 0,
    }
}

#[must_use]
pub(crate) fn pawn_attacks(side: Side, mask: Bitboard) -> Bitboard {
    match side {
        Side::White => mask.ne() | mask.nw(),
        Side::Black => mask.se() | mask.sw(),
    }
}

#[must_use]
pub(crate) fn castle_path(from: Square, dest: Square) -> Bitboard {
    if from == dest {
        return Bitboard::from_square(dest);
    }
    let from_bb = Bitboard::from_square(from);
    let dest_bb = Bitboard::from_square(dest);
    (magic::rook_moves(from, dest_bb) & magic::rook_moves(dest, from_bb)) | dest_bb
}

impl Position {
    pub fn move_generator(&self, mut func: impl FnMut(Move)) {
        debug_assert!(self.is_valid());

        let info = self.attack_info();
        let AttackInfo {
            occupied,
            ksq,
            kbb,
            them_n,
            them_bq,
            them_rq,
            checkers,
            pinned,
            bishop_pins,
            rook_pins,
            hpinned,
            ..
        } = info;
        let empty = !occupied;
        let allowed = |from: Square| info.allowed(from);

        // Pawns
        let pawn_rank_double = pawn_double_rank(self.turn);
        let pawn_rank_promo = pawn_promo_rank(self.turn);
        for from in self.sides[self.turn] & self.pieces[Piece::Pawn] {
            let bb = Bitboard::from_square(from);
            let single = pawn_single(self.turn, bb & !bishop_pins & !hpinned) & empty;
            let double = pawn_single(self.turn, single) & empty & pawn_rank_double;
            let attacks = pawn_attacks(self.turn, bb & !rook_pins) & self.sides[!self.turn];

            for to in (single | double | attacks) & allowed(from) {
                if to.rank() == pawn_rank_promo {
                    for promo in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
                        func(Move {
                            from,
                            to,
                            promo: Some(promo),
                        });
                    }
                } else {
                    func(Move {
                        from,
                        to,
                        promo: None,
                    });
                }
            }
        }

        // Pawns - en passant
        if let Some(to) = self.ep {
            let bb = Bitboard::from_square(to);
            let captured_bb = pawn_single(!self.turn, bb);
            let attacks = pawn_attacks(!self.turn, bb);
            let pawns = self.sides[self.turn] & self.pieces[Piece::Pawn];
            let is_ep_pawn_check = checkers == captured_bb;

            for from in attacks & pawns {
                let occupied_after = occupied ^ Bitboard::from_square(from) ^ captured_bb ^ bb;
                let bishop_rays = magic::bishop_moves(ksq, occupied_after)
                    & self.sides[!self.turn]
                    & (self.pieces[Piece::Bishop] | self.pieces[Piece::Queen]);
                let rook_rays = magic::rook_moves(ksq, occupied_after)
                    & self.sides[!self.turn]
                    & (self.pieces[Piece::Rook] | self.pieces[Piece::Queen]);
                let in_check = (bishop_rays | rook_rays).is_occupied();
                let allowed_mask = allowed(from);

                if (is_ep_pawn_check && !pinned.is_set(from))
                    || (!in_check && allowed_mask.is_set(to))
                {
                    func(Move {
                        from,
                        to,
                        promo: None,
                    });
                }
            }
        }

        // Knights
        for from in self.pieces[Piece::Knight] & self.sides[self.turn] & !pinned {
            for to in Bitboard::knights(from) & !self.sides[self.turn] & allowed(from) {
                func(Move {
                    from,
                    to,
                    promo: None,
                });
            }
        }

        // Bishops
        for from in self.pieces[Piece::Bishop] & self.sides[self.turn] {
            for to in magic::bishop_moves(from, occupied) & !self.sides[self.turn] & allowed(from) {
                func(Move {
                    from,
                    to,
                    promo: None,
                });
            }
        }

        // Rooks
        for from in self.pieces[Piece::Rook] & self.sides[self.turn] {
            for to in magic::rook_moves(from, occupied) & !self.sides[self.turn] & allowed(from) {
                func(Move {
                    from,
                    to,
                    promo: None,
                });
            }
        }

        // Queens
        for from in self.pieces[Piece::Queen] & self.sides[self.turn] {
            for to in magic::queen_moves(from, occupied) & !self.sides[self.turn] & allowed(from) {
                func(Move {
                    from,
                    to,
                    promo: None,
                });
            }
        }

        // King
        let king_candidates = kbb.adjacent() & !self.sides[self.turn];
        if king_candidates.is_occupied() {
            let them_ksq = self.ksq(!self.turn);
            let occ_without_our_king = occupied ^ kbb;
            let mut them_attacks = if self.turn == Side::White {
                let p = self.pieces[Piece::Pawn] & self.sides[!self.turn];
                p.se() | p.sw()
            } else {
                let p = self.pieces[Piece::Pawn] & self.sides[!self.turn];
                p.ne() | p.nw()
            };
            for sq in them_n {
                them_attacks |= Bitboard::knights(sq);
            }
            for sq in them_bq {
                them_attacks |= magic::bishop_moves(sq, occ_without_our_king);
            }
            for sq in them_rq {
                them_attacks |= magic::rook_moves(sq, occ_without_our_king);
            }
            them_attacks |= Bitboard::from_square(them_ksq).adjacent();

            for to in king_candidates & !them_attacks {
                func(Move {
                    from: ksq,
                    to,
                    promo: None,
                });
            }
        }

        if self.turn == Side::White && checkers.count() == 0 {
            let stuff = [
                (Castling::Wksc, Square::from_index(6), Square::from_index(5)),
                (Castling::Wqsc, Square::from_index(2), Square::from_index(3)),
            ];

            for (perm, kdest, rdest) in stuff {
                if let Some(rsq) = self.castling[perm] {
                    let blockers =
                        occupied ^ Bitboard::from_square(ksq) ^ Bitboard::from_square(rsq);

                    let king_path = castle_path(ksq, kdest);
                    let king_path_clear = (king_path & blockers).is_empty();

                    let rook_path = castle_path(rsq, rdest);
                    let rook_path_clear = (rook_path & blockers).is_empty();

                    if king_path_clear
                        && rook_path_clear
                        && !king_path
                            .into_iter()
                            .any(|sq| self.is_attacked2(sq, !blockers))
                    {
                        func(Move {
                            from: ksq,
                            to: rsq,
                            promo: None,
                        });
                    }
                }
            }
        }

        if self.turn == Side::Black && checkers.count() == 0 {
            let stuff = [
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
            ];

            for (perm, kdest, rdest) in stuff {
                if let Some(rsq) = self.castling[perm] {
                    let blockers =
                        occupied ^ Bitboard::from_square(ksq) ^ Bitboard::from_square(rsq);

                    let king_path = castle_path(ksq, kdest);
                    let king_path_clear = (king_path & blockers).is_empty();

                    let rook_path = castle_path(rsq, rdest);
                    let rook_path_clear = (rook_path & blockers).is_empty();

                    if king_path_clear
                        && rook_path_clear
                        && !king_path
                            .into_iter()
                            .any(|sq| self.is_attacked2(sq, !blockers))
                    {
                        func(Move {
                            from: ksq,
                            to: rsq,
                            promo: None,
                        });
                    }
                }
            }
        }
    }
}
