use chess::position::Position;
use std::{io::stdin, time::Instant};

fn main() {
    let fen = "startpos";
    let mut pos = Position::from_fen(fen).expect("Hmm");
    let mut history: Vec<Position> = vec![];

    loop {
        let moves = pos.legal_moves();

        println!("{}", pos);
        println!("FEN: {}", pos.get_fen());
        println!("Moves: {}", moves.len());

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");
        input = input.trim().to_string();

        let mut parts = input.split_whitespace();
        while let Some(word) = parts.next() {
            if word == "quit" {
                return;
            } else if word == "moves" || word == "movelist" {
                for mv in &moves {
                    println!("{}", mv.as_uci());
                }
            } else if let Some(mv) = moves.iter().find(|mv| mv.as_uci() == word) {
                history.push(pos);
                pos = pos.after_move::<true>(mv);
            } else if word == "undo" {
                if let Some(prev) = history.pop() {
                    pos = prev;
                }
            } else if word == "perft" {
                let depth = parts
                    .next()
                    .unwrap_or("0")
                    .parse::<u8>()
                    .expect("Not an integer");

                let t0 = Instant::now();
                for i in 1..=depth {
                    let nodes = pos.perft(i);
                    let dt = t0.elapsed();

                    println!("info depth {} nodes {} time {}", i, nodes, dt.as_millis());
                }
            } else if word == "split" {
                let depth = parts
                    .next()
                    .unwrap_or("0")
                    .parse::<u8>()
                    .expect("Not an integer");

                let mut total = 0;
                for mv in pos.legal_moves() {
                    print!("{}", mv.as_uci());

                    let npos = pos.after_move::<true>(&mv);
                    let nodes = npos.perft(depth);

                    total += nodes;
                    println!(" {}", nodes);
                }
                println!("nodes {}", total);
            } else if word == "fen" {
                let fen = parts.by_ref().collect::<Vec<_>>().join(" ");
                match Position::from_fen(&fen) {
                    Ok(new_pos) => {
                        history.clear();
                        pos = new_pos;
                    }
                    Err(err) => println!("Couldn't parse FEN: {err}"),
                }
            } else {
                println!("Unrecognised");
            }

            println!("");
        }
    }
}
