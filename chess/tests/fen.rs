use chess::position::Position;

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
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1",
        "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2",
        "r1bq1rk1/pp2ppbp/2np1np1/8/3NP3/2N1BP2/PPPQ2PP/2KR1B1R b - - 4 9",
        "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 4",
        // DFRC
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w H - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w A - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w h - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w a - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w HA - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w ha - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w HAha - 0 1",
    ];

    for fen in fens {
        let pos = Position::from_fen(fen);
        assert_eq!(
            pos.expect("Failed to parse fen").get_fen(),
            fen,
            "Fens not equal"
        );
    }
}

#[test]
fn fen_parse_illegal() {
    let fens = [
        // Missing parts
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
        // Unknown characters
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR ? KQkq - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w ?Qkq - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w K?kq - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQ?q - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQk? - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq ? 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - ? 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 ?",
        // Out of range
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - -1 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 -1",
        // Illegal castling
        "4k3/8/8/8/8/8/8/4K3 w K - 0 1",
        "4k3/8/8/8/8/8/8/4K3 w Q - 0 1",
        "4k3/8/8/8/8/8/8/4K3 w H - 0 1",
        "4k3/8/8/8/8/8/8/4K3 w A - 0 1",
        "4k3/8/8/8/8/8/8/4K3 w h - 0 1",
        "4k3/8/8/8/8/8/8/4K3 w a - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KK - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w QQ - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w kk - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w qq - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkqq - 0 1",
        // EP - no pawn ahead
        "4k3/8/8/8/8/8/8/4K3 w - e3 0 1",
        "4k3/8/8/8/8/8/8/4K3 w - e6 0 1",
        "4k3/8/8/8/8/8/8/4K3 b - e3 0 1",
        "4k3/8/8/8/8/8/8/4K3 b - e6 0 1",
        // EP - capture is illegal
        // "4k3/8/8/8/4P3/8/8/4K3 b - e3 0 1",
        // "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
        // "5k2/8/8/8/4Pp2/8/8/4KR2 b - e3 0 1",
        // Whitespace
        // "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR  w KQkq - 0 1",
        // "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -  0 1",
        // " rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        // "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ",
        // "  rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR   w   KQkq   -   0   1  ",
        // Empty
        "8/8/8/8/8/8/8/8 w KQkq - 0 1",
    ];

    for fen in fens {
        println!("{}", fen);
        assert!(
            Position::from_fen(fen).is_none(),
            "FEN wasn't meant to parse"
        );
    }
}

#[test]
fn fen_correct_ep() {
    // let fens = [];

    // for fen in fens {
    //     let pos = Position::from_fen(fen).expect("FEN didn't parse");
    //     assert_eq!(
    //         pos.expect("Failed to parse fen").get_fen(),
    //         fen,
    //         "Fens not equal"
    //     );
    // }
}
