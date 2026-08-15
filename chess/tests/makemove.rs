use chess::position::Position;

fn consistent(fen: &str) {
    let pos = Position::from_fen(fen).expect("FEN should parse");
    let before = pos;

    // Null moves
    let _ = pos.after_null::<true>();
    assert!(pos == before, "pos mutated by after_null: {}", fen);

    // Moves
    for mv in pos.legal_moves() {
        let _ = pos.after_move::<true>(&mv);
        assert!(pos == before, "pos mutated by after_move: {}", fen);
    }
}

#[test]
fn make_leaves_original_unchanged() {
    let fens = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 4",
        "n1n5/PPPk4/8/8/8/8/4Kppp/5N1N w - - 0 1",
        "4k3/8/8/8/8/8/8/4K2R w K - 0 1",
        "4k3/8/8/8/8/8/8/R3K3 w Q - 0 1",
        "r3k2r/8/8/8/8/8/8/4K3 b kq - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w HAha - 0 1",
    ];

    for fen in fens {
        consistent(fen);
    }
}
