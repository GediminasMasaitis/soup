use crate::{error::ParseError, piece::Piece, square::Square};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    pub(crate) from: Square,
    pub(crate) to: Square,
    pub(crate) promo: Option<Piece>,
}

impl Move {
    #[must_use]
    pub fn from_string(s: &str) -> Result<Self, ParseError> {
        if s.len() < 4 || s.len() > 5 {
            return Err(ParseError::InvalidMove);
        }

        let from = Square::from_string(&s[0..2])?;
        let to = Square::from_string(&s[2..4])?;
        let promo = match s.chars().nth(4) {
            Some('n') => Some(Piece::Knight),
            Some('b') => Some(Piece::Bishop),
            Some('r') => Some(Piece::Rook),
            Some('q') => Some(Piece::Queen),
            Some(_) => return Err(ParseError::InvalidMove),
            None => None,
        };

        if from == to {
            return Err(ParseError::InvalidMove);
        }

        Ok(Self { from, to, promo })
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

        for uci in valid {
            assert!(Move::from_string(uci).is_ok(), "Can't parse move string");
        }
    }

    #[test]
    fn invalid() {
        let nope = [
            "", "a10", "a1a9", "a1a1", "a7a8p", "a2a1p", "a7a8P", "a2a1P", "a7a8k", "a2a1k",
            "a7a8qk",
        ];

        for uci in nope {
            assert!(
                Move::from_string(uci).is_err(),
                "Move parsing was meant to fail"
            );
        }
    }
}
