use chess::position::Position;
use std::{io::stdin, time::Instant};

fn main() {
    let fen = "startpos";
    let mut pos = Position::from_fen(fen).expect("Hmm");

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

        let mut parts = input.split(" ");
        while let Some(word) = parts.next() {
            if word == "quit" {
                break;
            } else if word == "moves" || word == "movelist" {
                for mv in &moves {
                    println!("{} - 1", mv.as_uci());
                }
            } else if let Some(mv) = moves.iter().find(|mv| mv.as_uci() == word) {
                pos.makemove(mv);
            } else if word == "undo" {
                if !pos.history.is_empty() {
                    pos.undomove();
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

                    pos.makemove(&mv);
                    let nodes = pos.perft(depth);
                    pos.undomove();

                    total += nodes;
                    println!(" {}", nodes);
                }
                println!("nodes {}", total);
            } else if word == "fen" {
                let fen = parts.by_ref().collect::<Vec<_>>().join(" ");
                pos = Position::from_fen(&fen).expect("Couldn't parse FEN");
            } else if word == "info" {
                // println!("rook_pins:\n{}", rook_pins);
                // println!("bishop_pins:\n{}", bishop_pins);
                // println!("pins:\n{}", pins);
                // println!("allowed:\n{}", allowed);
                // println!("checkers:\n{}", checkers);
                // println!("num_checkers: {}", num_checkers);
                // println!();
            } else {
                println!("Unrecognised");
            }

            println!("");
        }
    }
}
