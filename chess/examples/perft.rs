use chess::position::Position;
use std::time::Instant;

fn main() {
    let mut pos = Position::from_fen("startpos").expect("");
    let t0 = Instant::now();
    let mut ans = None;

    // for (idx, mv) in pos.legal_moves().iter().enumerate() {
    //     println!("{}) {}", idx + 1, mv.as_uci());
    // }

    for i in 1..=6 {
        let nodes = pos.perft(i);
        let dt = t0.elapsed();
        ans = Some(nodes);

        println!(
            "info depth {} nodes {} nps {} time {}",
            i,
            nodes,
            (nodes as f32 / dt.as_secs_f32()) as u64,
            dt.as_millis()
        );
    }

    if let Some(nodes) = ans {
        println!("nodes {}", nodes);
    }
}
