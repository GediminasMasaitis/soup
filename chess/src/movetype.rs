use crate::{piece::Piece, square::Square};

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub(crate) from: Square,
    pub(crate) to: Square,
    pub(crate) promo: Option<Piece>,
}

impl Move {
    #[must_use]
    pub fn from_string(asd: &str) -> Option<Self> {
        if asd.len() < 4 || asd.len() > 5 {
            return None;
        }

        let from = Square::from_string(&asd[0..2])?;
        let to = Square::from_string(&asd[2..4])?;
        let promo = match asd.chars().nth(4) {
            Some('n') => Some(Piece::Knight),
            Some('b') => Some(Piece::Bishop),
            Some('r') => Some(Piece::Rook),
            Some('q') => Some(Piece::Queen),
            Some(_) => return None,
            None => None,
        };

        if from == to {
            return None;
        }

        Some(Self { from, to, promo })
    }

    #[must_use]
    pub fn as_uci(&self) -> String {
        format!(
            "{}{}{}",
            self.from,
            self.to,
            match self.promo {
                Some(Piece::Knight) => "n",
                Some(Piece::Bishop) => "b",
                Some(Piece::Rook) => "r",
                Some(Piece::Queen) => "q",
                Some(_) => panic!("Uh oh"),
                None => "",
            }
        )
    }
}

#[cfg(test)]
mod test {
    use crate::movetype::Move;

    #[test]
    fn valid() {
        let valid = [
            "a1a8", "a1h1", "h1h8", "a8h8", "a2a4", "c7c5", "a7a8n", "a7a8b", "a7a8r", "a7a8q",
        ];

        for asd in valid {
            assert!(Move::from_string(asd).is_some(), "Can't parse move string");
        }
    }

    #[test]
    fn invalid() {
        let nope = [
            "", "a10", "a1a9", "a1a1", "a7a8p", "a2a1p", "a7a8P", "a2a1P", "a7a8k", "a2a1k",
            "a7a8qk",
        ];

        for asd in nope {
            assert!(
                Move::from_string(asd).is_none(),
                "Move parsing was meant to fail"
            );
        }
    }
}
