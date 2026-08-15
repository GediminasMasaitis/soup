use chess::position::Position;

fn parse_invalid(fens: &[&str]) {
    for fen in fens {
        assert!(
            Position::from_fen(fen).is_err(),
            "FENs shouldn't parse: {}",
            fen
        );
    }
}

#[test]
fn startpos() {
    assert_eq!(
        Position::startpos().get_fen(),
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "startpos get_fen() mismatch"
    );
}

#[test]
fn fen_parse_legal() {
    let fens = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1",
        "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2",
        "r1bq1rk1/pp2ppbp/2np1np1/8/3NP3/2N1BP2/PPPQ2PP/2KR1B1R b - - 4 9",
        "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 4",
        // DFRC
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w H - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w A - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w h - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w a - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w HA - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w ha - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w HAha - 0 1",
    ];

    for fen in fens {
        let pos = Position::from_fen(fen).expect("Failed to parse fen");
        assert_eq!(pos.get_fen(), *fen, "FENs not equal: {}", fen);
    }
}

#[test]
fn fen_missing_parts() {
    let fens = [
        "",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0",
        "w KQkq - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR KQkq - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq 0 1",
    ];

    parse_invalid(&fens);
}

#[test]
fn fen_unknown_characters() {
    let fens = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPP?PPPP/RNBQKBNR w KQkq - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR ? KQkq - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w ?Qkq - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w K?kq - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQ?q - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQk? - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq ? 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - ? 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 ?",
    ];

    parse_invalid(&fens);
}

#[test]
fn fen_out_of_range() {
    let fens = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - -1 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 -1",
    ];

    parse_invalid(&fens);
}

#[test]
fn fen_wrong_piece_counts() {
    let fens = [
        // Multiple kings
        "4k3/8/8/8/8/8/8/2K1K3 w - - 0 1",
        "2k1k3/8/8/8/8/8/8/4K3 w - - 0 1",
        // More than 16 pieces
        "rnbqkbnr/pppppppp/8/8/Q7/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "rnbqkbnr/pppppppp/8/8/R7/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "rnbqkbnr/pppppppp/8/8/B7/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "rnbqkbnr/pppppppp/8/8/N7/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "rnbqkbnr/pppppppp/8/8/P7/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        // More than can be promoted
        "rnbqkbnr/pppppppp/8/8/Q7/8/QQQQQQQQ/1NBQKBNR w - - 0 1",
        "rnbqkbnr/pppppppp/8/8/R7/8/RRRRRRRR/RNBQKBNR w - - 0 1",
        "rnbqkbnr/pppppppp/8/8/B7/8/BBBBBBBB/1NBQKBNR w - - 0 1",
        "rnbqkbnr/pppppppp/8/8/N7/8/NNNNNNNN/1NBQKBNR w - - 0 1",
    ];

    parse_invalid(&fens);
}

#[test]
fn fen_illegal_castling() {
    let fens = [
        // No rooks
        "4k3/8/8/8/8/8/8/4K3 w K - 0 1",
        "4k3/8/8/8/8/8/8/4K3 w Q - 0 1",
        "4k3/8/8/8/8/8/8/4K3 w k - 0 1",
        "4k3/8/8/8/8/8/8/4K3 w q - 0 1",
        "4k3/8/8/8/8/8/8/4K3 w H - 0 1",
        "4k3/8/8/8/8/8/8/4K3 w A - 0 1",
        "4k3/8/8/8/8/8/8/4K3 w h - 0 1",
        "4k3/8/8/8/8/8/8/4K3 w a - 0 1",
        "4k3/8/8/8/8/8/8/4K3 w KQkq - 0 1",
        "4k3/8/8/8/8/8/8/4K3 w HAha - 0 1",
        // Permission combinations
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KK - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w QQ - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w kk - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w qq - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkqq - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w HH - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w AA - 0 1",
        // Permission out of range
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w N - 0 1",
        // King/Rook off back rank
        "r6r/4k3/8/8/8/8/4K3/R6R w KQkq - 0 1",
        "4k3/r6r/8/8/8/8/R6R/4K3 w KQkq - 0 1",
        // Rook in the wrong place
        "4k3/8/8/8/8/8/8/4K1R1 w K - 0 1",
        "4k3/8/8/8/8/8/8/1R2K3 w Q - 0 1",
        // King can't be on the edge if castling opposite
        "4k3/8/8/8/8/8/8/K6R w H - 0 1",
        "4k3/8/8/8/8/8/8/R6K w A - 0 1",
        // Both rooks on one side
        "rr2k3/8/8/8/8/8/8/RR2K3 w BAba - 0 1",
        "4k1rr/8/8/8/8/8/8/4K1RR w HGhg - 0 1",
        // Regular & FRC mix
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQha - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w HAkq - 0 1",
    ];

    parse_invalid(&fens);
}

#[test]
fn fen_illegal_ep() {
    let fens = [
        // EP - no pawn ahead
        "4k3/8/8/8/8/8/8/4K3 w - e3 0 1",
        "4k3/8/8/8/8/8/8/4K3 w - e6 0 1",
        "4k3/8/8/8/8/8/8/4K3 b - e3 0 1",
        "4k3/8/8/8/8/8/8/4K3 b - e6 0 1",
        // EP - wrong rank
        "4k3/8/8/4P3/8/8/8/4K3 b - e4 0 1",
        // EP - blocked
        "rnbqkbnr/pppp1ppp/4N3/4p3/8/8/PPPPPPPP/RNBQKB1R w KQkq e6 0 1",
        "rnbqkb1r/pppppppp/8/8/4P3/4n3/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
        "rnbqkbnr/pppp1ppp/4N3/3Pp3/8/8/PPP1PPPP/RNBQKB1R w KQkq e6 0 1",
        "rnbqkb1r/ppp1pppp/8/8/3pP3/4n3/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
    ];

    parse_invalid(&fens);
}

#[test]
fn fen_malformed() {
    let fens = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR/ w KQkq - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP//RNBQKBNR w KQkq - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/9/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/7/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1",
    ];

    parse_invalid(&fens);
}

#[test]
fn fen_other() {
    let fens = [
        // Empty
        "8/8/8/8/8/8/8/8 w KQkq - 0 1",
        // Touching kings
        "8/8/8/8/8/8/8/3kK3 w - - 0 1",
    ];

    parse_invalid(&fens);
}

#[test]
fn fen_correct_ep() {
    let tests = [
        // Not set
        (
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        ),
        // Legal
        (
            "4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 1",
            "4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 1",
        ),
        (
            "4k3/8/8/4pP2/3K4/8/8/8 w - e6 0 1",
            "4k3/8/8/4pP2/3K4/8/8/8 w - e6 0 1",
        ),
        // Illegal - rook pin
        (
            "4k3/4r3/8/3pP3/8/8/8/4K3 w - d6 0 1",
            "4k3/4r3/8/3pP3/8/8/8/4K3 w - - 0 1",
        ),
        // Illegal - rook pin horizontal
        (
            "4k3/8/8/K2pP2r/8/8/8/8 w - d6 0 1",
            "4k3/8/8/K2pP2r/8/8/8/8 w - - 0 1",
        ),
        // Illegal - bishop pin
        (
            "4k3/b7/8/2Pp4/8/8/8/6K1 w - d6 0 1",
            "4k3/b7/8/2Pp4/8/8/8/6K1 w - - 0 1",
        ),
        // Illegal - in check
        (
            "4k3/8/5n2/3pP3/4K3/8/8/8 w - d6 0 1",
            "4k3/8/5n2/3pP3/4K3/8/8/8 w - - 0 1",
        ),
    ];

    for (fen, corrected) in tests {
        let pos = Position::from_fen(fen).expect("FEN didn't parse");
        assert_eq!(pos.get_fen(), corrected, "Fens not equal");
    }
}
