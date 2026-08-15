use chess::position::Position;

const FENS: [&str; 15] = [
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
    "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 4",
    "n1n5/PPPk4/8/8/8/8/4Kppp/5N1N w - - 0 1",
    "4k3/8/8/8/8/8/8/4K2R w K - 0 1",
    "4k3/8/8/8/8/8/8/R3K3 w Q - 0 1",
    "r3k2r/8/8/8/8/8/8/4K3 b kq - 0 1",
    // 960 castling
    "2r1kr2/8/8/8/8/8/8/1R2K1R1 w GBfc - 0 1",
    "rkr5/8/8/8/8/8/8/5RKR w HFca - 0 1",
    "5rkr/8/8/8/8/8/8/RKR5 w CAhf - 0 1",
    "3rkr2/8/8/8/8/8/8/R3K2R w HAfd - 0 1",
    "4k3/8/8/8/8/8/8/4KR2 w F - 0 1",
    "4kr2/8/8/8/8/8/8/4K3 w f - 0 1",
    "4k3/8/8/8/8/8/8/2R1K3 w C - 0 1",
    "2r1k3/8/8/8/8/8/8/4K3 w c - 0 1",
];

fn check_hash(fen: &str) {
    let pos = Position::from_fen(fen).expect("FEN should parse");
    let before = pos.zobrist;

    // Null move
    let npos = pos.after_null::<true>();
    assert_ne!(before, npos.zobrist, "Zobrist nullmove fail: {}", fen);

    // Moves
    for mv in pos.legal_moves() {
        let npos = pos.after_move::<true>(&mv);
        assert_ne!(before, npos.zobrist, "Zobrist makemove fail: {}", fen);
    }
}

#[test]
fn hash_double_nullmove() {
    for fen in &FENS {
        let mut pos = Position::from_fen(fen).expect("FEN should parse");
        let before = pos.zobrist;
        let ep = pos.ep;

        pos.makenull::<true>();
        pos.makenull::<true>();

        let after = pos.zobrist;

        if let Some(sq) = ep {
            assert_eq!(
                before,
                after ^ Position::ep_key(sq),
                "Zobrist double nullmove fail: {}",
                fen
            );
        } else {
            assert_eq!(before, after, "Zobrist double nullmove fail: {}", fen);
        }
    }
}

#[test]
fn hash_changes_after_move() {
    for fen in &FENS {
        check_hash(fen);
    }
}
