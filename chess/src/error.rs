use crate::{fen::FenError, is_valid::PositionalError};
use std::fmt;

#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidSquare,
    InvalidMove,
    InvalidFen(FenError),
    InvalidPosition(Vec<PositionalError>),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSquare => write!(f, "invalid square"),
            Self::InvalidMove => write!(f, "invalid move"),
            Self::InvalidFen(error) => write!(f, "invalid FEN: {error}"),
            Self::InvalidPosition(errors) => {
                write!(f, "invalid position: ")?;
                for (i, error) in errors.iter().enumerate() {
                    if i > 0 {
                        write!(f, "; ")?;
                    }
                    write!(f, "{error}")?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidFen(error) => Some(error),
            Self::InvalidPosition(errors) => errors.first().map(|e| e as _),
            _ => None,
        }
    }
}
