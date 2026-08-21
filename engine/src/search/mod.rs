use chess::movetype::Move;
use chess::position::Position;

#[derive(Default)]
pub struct SearchParameters {
    pub wtime: Option<u64>,
    pub btime: Option<u64>,
    pub winc: Option<u64>,
    pub binc: Option<u64>,
    pub movetime: Option<u64>,
    pub depth: Option<u32>,
    pub nodes: Option<u64>,
    pub infinite: bool,
}

pub fn search(pos: &Position, _params: &SearchParameters) -> Option<Move> {
    let moves = pos.legal_moves();
    if moves.is_empty() {
        return None;
    }
    Some(moves[rand::random_range(0..moves.len())])
}
