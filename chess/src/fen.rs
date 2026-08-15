use crate::{
    bitboard::Bitboard, castling::Castling, error::ParseError, piece::Piece, position::Position,
    side::Side, square::Square,
};
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum FenError {
    MissingField(&'static str),
    MalformedBoard,
    UnknownPieceChar(char),
    InvalidSideToMove,
    MissingKingForCastling,
    InvalidCastlingChar(char),
    DuplicateCastlingRight(char),
    MixedCastlingNotation,
    InvalidHalfmoveClock,
    InvalidFullmoveNumber,
    ZeroFullmoveNumber,
    TrailingFields,
}

impl fmt::Display for FenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingField(field) => write!(f, "missing {field} field"),
            Self::MalformedBoard => write!(f, "malformed piece placement field"),
            Self::UnknownPieceChar(c) => write!(f, "unknown piece character '{c}'"),
            Self::InvalidSideToMove => write!(f, "side to move must be 'w' or 'b'"),
            Self::MissingKingForCastling => {
                write!(f, "castling rights given but a king is missing")
            }
            Self::InvalidCastlingChar(c) => write!(f, "invalid castling character '{c}'"),
            Self::DuplicateCastlingRight(c) => {
                write!(f, "castling right '{c}' given more than once")
            }
            Self::MixedCastlingNotation => {
                write!(f, "mixed standard and Chess960 castling notation")
            }
            Self::InvalidHalfmoveClock => write!(f, "halfmove clock is not a valid number"),
            Self::InvalidFullmoveNumber => write!(f, "fullmove number is not a valid number"),
            Self::ZeroFullmoveNumber => write!(f, "fullmove number must be at least 1"),
            Self::TrailingFields => write!(f, "too many fields"),
        }
    }
}

impl std::error::Error for FenError {}

impl Position {
    pub fn from_fen(fen: &str) -> Result<Self, ParseError> {
        if fen == "startpos" {
            return Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        }

        let mut pos = Position::new();
        let mut parts = fen.split_whitespace();
        let mut wksq = None;
        let mut bksq = None;

        // Piece locations
        let part = parts
            .next()
            .ok_or(ParseError::InvalidFen(FenError::MissingField("piece placement")))?;

        let mut y = 7i32;
        let mut x = 0i32;
        for c in part.chars() {
            if c == '/' {
                if x != 8 {
                    return Err(ParseError::InvalidFen(FenError::MalformedBoard));
                }
                x = 0;
                y -= 1;
                continue;
            }

            if !(0..8).contains(&x) || !(0..8).contains(&y) {
                return Err(ParseError::InvalidFen(FenError::MalformedBoard));
            }

            if let '1'..='8' = c {
                x += c.to_digit(10).ok_or(ParseError::InvalidFen(FenError::MalformedBoard))? as i32;
                continue;
            }

            let piece = match c.to_ascii_lowercase() {
                'p' => Piece::Pawn,
                'n' => Piece::Knight,
                'b' => Piece::Bishop,
                'r' => Piece::Rook,
                'q' => Piece::Queen,
                'k' => Piece::King,
                _ => return Err(ParseError::InvalidFen(FenError::UnknownPieceChar(c))),
            };
            let side = if c.is_ascii_uppercase() {
                Side::White
            } else {
                Side::Black
            };

            let sq = Square::from_indices(x as u8, y as u8);
            let bb = Bitboard::from_square(sq);
            pos.pieces[piece] ^= bb;
            pos.sides[side] ^= bb;

            if piece == Piece::King {
                match side {
                    Side::White => wksq = Some(sq),
                    Side::Black => bksq = Some(sq),
                }
            }

            x += 1;
        }

        if x != 8 || y != 0 {
            return Err(ParseError::InvalidFen(FenError::MalformedBoard));
        }

        // Side to move
        let part = parts
            .next()
            .ok_or(ParseError::InvalidFen(FenError::MissingField("side to move")))?;
        pos.turn = match part {
            "w" => Side::White,
            "b" => Side::Black,
            _ => return Err(ParseError::InvalidFen(FenError::InvalidSideToMove)),
        };

        // Castling
        let part = parts
            .next()
            .ok_or(ParseError::InvalidFen(FenError::MissingField("castling rights")))?;
        if part != "-" {
            let wf = (b'A'
                + wksq
                    .ok_or(ParseError::InvalidFen(FenError::MissingKingForCastling))?
                    .file()) as char;
            let bf = (b'a'
                + bksq
                    .ok_or(ParseError::InvalidFen(FenError::MissingKingForCastling))?
                    .file()) as char;

            for c in part.chars() {
                pos.is_960 |= !"KQkq".contains(c);

                let (perm, idx) = match c {
                    'K' => (Castling::Wksc, 7),
                    'Q' => (Castling::Wqsc, 0),
                    'k' => (Castling::Bksc, 63),
                    'q' => (Castling::Bqsc, 56),
                    'A'..='H' if c > wf => (Castling::Wksc, c as u8 - b'A'),
                    'A'..='H' if c < wf => (Castling::Wqsc, c as u8 - b'A'),
                    'a'..='h' if c > bf => (Castling::Bksc, 56 + c as u8 - b'a'),
                    'a'..='h' if c < bf => (Castling::Bqsc, 56 + c as u8 - b'a'),
                    _ => return Err(ParseError::InvalidFen(FenError::InvalidCastlingChar(c))),
                };

                // Permission already set
                if pos.castling[perm].is_some() {
                    return Err(ParseError::InvalidFen(FenError::DuplicateCastlingRight(c)));
                }

                pos.castling[perm] = Some(Square::from_index(idx));
            }

            // Mixed regular and 960 castling permissions
            if pos.is_960 && part.chars().any(|c| "KQkq".contains(c)) {
                return Err(ParseError::InvalidFen(FenError::MixedCastlingNotation));
            }
        }

        // En passant
        let part = parts
            .next()
            .ok_or(ParseError::InvalidFen(FenError::MissingField("en passant")))?;
        pos.ep = match part {
            "-" => None,
            sq => Some(Square::from_string(sq)?),
        };

        // Halfmoves
        let part = parts
            .next()
            .ok_or(ParseError::InvalidFen(FenError::MissingField("halfmove clock")))?;
        pos.halfmoves = part
            .parse::<usize>()
            .map_err(|_| ParseError::InvalidFen(FenError::InvalidHalfmoveClock))?;

        // Fullmoves
        let part = parts
            .next()
            .ok_or(ParseError::InvalidFen(FenError::MissingField("fullmove number")))?;
        pos.fullmoves = part
            .parse::<usize>()
            .map_err(|_| ParseError::InvalidFen(FenError::InvalidFullmoveNumber))?;

        // Should be nothing left
        if parts.next().is_some() {
            return Err(ParseError::InvalidFen(FenError::TrailingFields));
        }

        if pos.fullmoves == 0 {
            return Err(ParseError::InvalidFen(FenError::ZeroFullmoveNumber));
        }

        pos.zobrist = pos.calculate_zobrist();

        let errors = pos.errors();
        if errors.is_empty() {
            Ok(pos)
        } else {
            Err(ParseError::InvalidPosition(errors))
        }
    }

    pub fn get_fen(&self) -> String {
        let mut fen = String::new();

        // Pieces
        for y in (0..8).rev() {
            let mut empty = 0;

            for x in 0..8 {
                let sq = Square::from_indices(x, y);

                let Some(piece) = self.piece_on(sq) else {
                    empty += 1;
                    continue;
                };

                if empty > 0 {
                    fen += &empty.to_string();
                    empty = 0;
                }

                let c = match piece {
                    Piece::Pawn => 'p',
                    Piece::Knight => 'n',
                    Piece::Bishop => 'b',
                    Piece::Rook => 'r',
                    Piece::Queen => 'q',
                    Piece::King => 'k',
                };

                if self.sides[Side::White].is_set(sq) {
                    fen.push(c.to_ascii_uppercase());
                } else {
                    fen.push(c);
                }
            }

            if empty > 0 {
                fen += &empty.to_string();
            }

            if y > 0 {
                fen += "/";
            }
        }

        // Side to move
        fen += match self.turn {
            Side::White => " w",
            Side::Black => " b",
        };

        // Castling
        if self.castling == [None; 4] {
            fen += " -";
        } else {
            fen += " ";

            for (perm, base, standard) in [
                (Castling::Wksc, b'A', 'K'),
                (Castling::Wqsc, b'A', 'Q'),
                (Castling::Bksc, b'a', 'k'),
                (Castling::Bqsc, b'a', 'q'),
            ] {
                if let Some(sq) = self.castling[perm] {
                    fen.push(if self.is_960 {
                        (base + sq.file()) as char
                    } else {
                        standard
                    });
                }
            }
        }

        // En passant
        if let Some(sq) = self.ep
            && self.is_legal_ep()
        {
            fen += " ";
            fen += &sq.to_string();
        } else {
            fen += " -";
        }

        // Halfmoves
        fen += " ";
        fen += &self.halfmoves.to_string();

        // Fullmoves
        fen += " ";
        fen += &self.fullmoves.to_string();

        fen
    }
}
