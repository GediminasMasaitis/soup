use chess::position::Position;
use std::time::Instant;

fn main() {
    let fens = [
        "startpos",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
    ];

    for fen in fens {
        let pos = Position::from_fen(fen).expect("FEN should parse");

        // Pure move generation, no recursion, no makemove
        let iters = 20_000_000u64;
        let t0 = Instant::now();
        let mut total: u64 = 0;
        for _ in 0..iters {
            total += pos.count_moves();
        }
        let dt = t0.elapsed();
        println!(
            "[{}] count_moves: {} calls in {:?} ({:.1} M calls/sec, checksum {})",
            fen,
            iters,
            dt,
            iters as f64 / dt.as_secs_f64() / 1e6,
            total
        );

        // Pure makemove+undo via after_move, one legal move repeated
        let mv = pos.legal_moves()[0];
        let iters2 = 50_000_000u64;
        let t0 = Instant::now();
        let mut checksum: u64 = 0;
        for _ in 0..iters2 {
            let npos = pos.after_move::<false>(&mv);
            checksum ^= npos.zobrist;
        }
        let dt = t0.elapsed();
        println!(
            "[{}] after_move: {} calls in {:?} ({:.1} M calls/sec, checksum {})",
            fen,
            iters2,
            dt,
            iters2 as f64 / dt.as_secs_f64() / 1e6,
            checksum
        );
    }
}
