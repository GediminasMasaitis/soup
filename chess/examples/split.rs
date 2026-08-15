use chess::position::Position;

fn main() {
    let pos = Position::from_fen("startpos").expect("");
    let depth = 1;

    println!("{}", pos);

    for mv in pos.legal_moves() {
        print!("{}", mv.as_uci());
        let npos = pos.after_move::<false>(&mv);
        let nodes = npos.perft(depth);
        println!(" - {}", nodes);
    }
}
