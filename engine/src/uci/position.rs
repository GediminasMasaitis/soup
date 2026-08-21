use chess::position::Position;

pub fn parse<'a>(mut stream: impl Iterator<Item = &'a str>) -> Option<Position> {
    let mut pos = match stream.next()? {
        "startpos" => {
            stream.next();
            Position::startpos()
        }
        "fen" => {
            let fen = stream
                .by_ref()
                .take_while(|word| *word != "moves")
                .collect::<Vec<_>>()
                .join(" ");
            Position::from_fen(&fen).expect("Couldn't parse FEN")
        }
        _ => return None,
    };

    for word in stream {
        let moves = pos.legal_moves();
        let mv = moves
            .iter()
            .find(|mv| mv.as_uci() == word)
            .expect("Illegal move");
        pos.makemove::<true>(mv);
    }

    Some(pos)
}
