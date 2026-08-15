use chess::position::Position;

fn main() {
    let mut pos = Position::from_fen("startpos").expect("");
    let depth = 1;

    println!("{}", pos);

    for mv in pos.legal_moves() {
        print!("{}", mv.as_uci());
        pos.makemove(&mv);
        let nodes = pos.perft(depth);
        pos.undomove();
        println!(" - {}", nodes);
    }
}
