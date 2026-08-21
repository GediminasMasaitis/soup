use super::options::Options;
use super::{go, position, setoption};
use crate::search::{self, SearchParameters};
use chess::position::Position;
use std::io::{BufRead, Write};

pub fn run(mut reader: impl BufRead, mut writer: impl Write) -> std::io::Result<()> {
    let mut options = Options::default();

    writeln!(writer, "id name soup")?;
    writeln!(writer, "id author kz04px, Gediminas Masaitis")?;
    options.print(&mut writer)?;
    writeln!(writer, "uciok")?;

    let mut input = String::new();

    loop {
        input.clear();
        if !matches!(reader.read_line(&mut input), Ok(n) if n > 0) {
            return Ok(()); // stdin closed
        }

        let mut words = input.split_whitespace();

        match words.next() {
            None => {}
            Some("quit") => return Ok(()),
            Some("setoption") => {
                setoption::parse(&mut words, |name, value| options.set(name, value));
            }
            Some(_) => break,
        }
    }

    let mut pos = Position::startpos();

    loop {
        let mut words = input.split_whitespace();

        match words.next() {
            Some("quit") => break,
            Some("isready") => writeln!(writer, "readyok")?,
            Some("position") => {
                if let Some(new_pos) = position::parse(words) {
                    pos = new_pos;
                }
            }
            Some("ucinewgame") => {}
            Some("go") => bestmove(&mut writer, &pos, &go::parse(words))?,
            _ => {}
        }

        input.clear();
        if !matches!(reader.read_line(&mut input), Ok(n) if n > 0) {
            break; // stdin closed
        }
    }

    Ok(())
}

fn bestmove(
    writer: &mut impl Write,
    pos: &Position,
    params: &SearchParameters,
) -> std::io::Result<()> {
    match search::search(pos, params) {
        Some(mv) => writeln!(writer, "bestmove {}", mv.as_uci()),
        None => writeln!(writer, "bestmove 0000"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn run_raw(lines: &[&str]) -> String {
        let input = lines.join("\n");
        let mut output = Vec::new();
        run(Cursor::new(input), &mut output).unwrap();
        String::from_utf8(output).unwrap()
    }

    fn run_lines(lines: &[&str]) -> String {
        let output = run_raw(lines);
        let (_, rest) = output.split_once("uciok\n").expect("No uciok");
        rest.to_owned()
    }

    #[test]
    fn handshake() {
        let output = run_raw(&["quit"]);
        assert!(output.starts_with("id name soup\n"));
        assert!(output.contains("id author"));
        assert!(output.contains("option name Hash type spin default 0 min 0 max 0\n"));
        assert!(output.contains("option name Threads type spin default 1 min 1 max 1\n"));
        assert!(output.ends_with("uciok\n"));
    }

    #[test]
    fn isready() {
        assert_eq!(run_lines(&["isready", "quit"]), "readyok\n");
    }

    #[test]
    fn unknown_commands_are_ignored() {
        assert_eq!(run_lines(&["banana", "", "isready", "quit"]), "readyok\n");
    }

    #[test]
    fn eof_without_quit_exits() {
        assert_eq!(run_lines(&["isready"]), "readyok\n");
    }

    #[test]
    fn go_prints_legal_bestmove() {
        let output = run_lines(&["position startpos", "go depth 3 movetime 100", "quit"]);
        let mv = output
            .trim()
            .strip_prefix("bestmove ")
            .expect("No bestmove");
        let pos = Position::startpos();
        assert!(pos.legal_moves().iter().any(|m| m.as_uci() == mv));
    }

    #[test]
    fn go_defaults_to_startpos() {
        let output = run_lines(&["go", "quit"]);
        let mv = output
            .trim()
            .strip_prefix("bestmove ")
            .expect("No bestmove");
        let pos = Position::startpos();
        assert!(pos.legal_moves().iter().any(|m| m.as_uci() == mv));
    }

    #[test]
    fn fen_forces_bestmove() {
        let output = run_lines(&["position fen 7k/8/8/8/8/8/6q1/7K w - - 0 1", "go", "quit"]);
        assert_eq!(output, "bestmove h1g2\n");
    }

    #[test]
    fn moves_are_applied_after_fen() {
        let output = run_lines(&[
            "position fen 7k/8/8/8/8/6q1/8/7K b - - 0 1 moves g3g2",
            "go",
            "quit",
        ]);
        assert_eq!(output, "bestmove h1g2\n");
    }

    #[test]
    fn go_when_checkmated_prints_null_move() {
        let output = run_lines(&[
            "position fen rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3",
            "go",
            "quit",
        ]);
        assert_eq!(output, "bestmove 0000\n");
    }

    #[test]
    fn uci_is_not_repeated() {
        assert_eq!(run_lines(&["uci", "isready", "quit"]), "readyok\n");
    }

    #[test]
    fn quit_before_isready_exits() {
        assert_eq!(run_lines(&["quit", "isready"]), "");
    }

    #[test]
    fn setoption_before_isready_is_accepted() {
        let output = run_lines(&["", "setoption name Hash value 0", "isready", "quit"]);
        assert_eq!(output, "readyok\n");
    }

    #[test]
    fn go_before_isready_is_handled() {
        assert_eq!(
            run_lines(&[
                "position fen 7k/8/8/8/8/8/6q1/7K w - - 0 1",
                "go",
                "isready",
                "quit"
            ]),
            "bestmove h1g2\nreadyok\n"
        );
    }

    #[test]
    fn setoption_after_isready_is_ignored() {
        let output = run_lines(&["isready", "setoption name Hash value 0", "isready", "quit"]);
        assert_eq!(output, "readyok\nreadyok\n");
    }

    #[test]
    #[should_panic]
    fn illegal_move_panics() {
        run_lines(&["position startpos moves e2e5", "quit"]);
    }

    #[test]
    #[should_panic]
    fn bad_fen_panics() {
        run_lines(&["position fen banana", "quit"]);
    }
}
