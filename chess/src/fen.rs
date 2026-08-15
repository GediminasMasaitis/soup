use crate::{
    bitboard::Bitboard, castling::Castling, piece::Piece, position::Position, side::Side,
    square::Square,
};

impl Position {
    pub fn from_fen(fen: &str) -> Option<Self> {
        if fen == "startpos" {
            return Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        }

        let mut pos = Position::new();
        let mut parts = fen.split_whitespace();
        let mut wksq = None;
        let mut bksq = None;

        // Piece locations
        let part = parts.next()?;

        for (idx, rank) in part.split('/').enumerate() {
            let y = 7 - idx;
            let mut x = 0;

            for c in rank.chars() {
                let sq = Square::from_indices(x, y as u8);
                let bb = Bitboard::from_square(sq);

                match c {
                    'P' => {
                        pos.pieces[Piece::Pawn] ^= bb;
                        pos.sides[Side::White] ^= bb;
                        x += 1;
                    }
                    'N' => {
                        pos.pieces[Piece::Knight] ^= bb;
                        pos.sides[Side::White] ^= bb;
                        x += 1;
                    }
                    'B' => {
                        pos.pieces[Piece::Bishop] ^= bb;
                        pos.sides[Side::White] ^= bb;
                        x += 1;
                    }
                    'R' => {
                        pos.pieces[Piece::Rook] ^= bb;
                        pos.sides[Side::White] ^= bb;
                        x += 1;
                    }
                    'Q' => {
                        pos.pieces[Piece::Queen] ^= bb;
                        pos.sides[Side::White] ^= bb;
                        x += 1;
                    }
                    'K' => {
                        pos.pieces[Piece::King] ^= bb;
                        pos.sides[Side::White] ^= bb;
                        wksq = Some(sq);
                        x += 1;
                    }
                    'p' => {
                        pos.pieces[Piece::Pawn] ^= bb;
                        pos.sides[Side::Black] ^= bb;
                        x += 1;
                    }
                    'n' => {
                        pos.pieces[Piece::Knight] ^= bb;
                        pos.sides[Side::Black] ^= bb;
                        x += 1;
                    }
                    'b' => {
                        pos.pieces[Piece::Bishop] ^= bb;
                        pos.sides[Side::Black] ^= bb;
                        x += 1;
                    }
                    'r' => {
                        pos.pieces[Piece::Rook] ^= bb;
                        pos.sides[Side::Black] ^= bb;
                        x += 1;
                    }
                    'q' => {
                        pos.pieces[Piece::Queen] ^= bb;
                        pos.sides[Side::Black] ^= bb;
                        x += 1;
                    }
                    'k' => {
                        pos.pieces[Piece::King] ^= bb;
                        pos.sides[Side::Black] ^= bb;
                        bksq = Some(sq);
                        x += 1;
                    }
                    '1'..='8' => {
                        x += c.to_digit(10)? as u8;
                    }
                    _ => return None,
                }
            }

            assert_eq!(x, 8, "Uhh");
        }

        // Side to move
        let part = parts.next()?;
        pos.turn = match part {
            "w" => Side::White,
            "b" => Side::Black,
            _ => return None,
        };

        // Castling
        let part = parts.next()?;
        if part != "-" {
            let wf = (b'A' + wksq?.file()) as char;
            let bf = (b'a' + bksq?.file()) as char;

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
                    _ => return None,
                };

                // Permission already set
                if pos.castling[perm].is_some() {
                    return None;
                }

                pos.castling[perm] = Some(Square::from_index(idx));
            }
        }

        // En passant
        let part = parts.next()?;
        pos.ep = match part {
            "-" => None,
            sq => Some(Square::from_string(sq)?),
        };

        // Halfmoves
        let part = parts.next()?;
        pos.halfmoves = part.parse::<usize>().ok()?;

        // Fullmoves
        let part = parts.next()?;
        pos.fullmoves = part.parse::<usize>().ok()?;

        // Should be nothing left
        if parts.next().is_some() {
            return None;
        }

        if pos.fullmoves == 0 {
            return None;
        }

        if pos.is_valid() { Some(pos) } else { None }
    }

    pub fn get_fen(&self) -> String {
        let mut fen = String::new();

        // Pieces
        for y in (0..8).rev() {
            let mut empty = 0;

            for x in 0..8 {
                let sq = Square::from_indices(x, y);
                let piece = self.piece_on(sq);

                if piece.is_none() {
                    empty += 1;
                    continue;
                }

                if empty > 0 {
                    fen += &empty.to_string();
                    empty = 0;
                }

                let c = match piece.unwrap() {
                    Piece::Pawn => "p",
                    Piece::Knight => "n",
                    Piece::Bishop => "b",
                    Piece::Rook => "r",
                    Piece::Queen => "q",
                    Piece::King => "k",
                };

                if self.sides[Side::White].is_set(sq) {
                    fen.push_str(&c.to_ascii_uppercase());
                } else {
                    fen.push_str(c);
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
            fen.push_str(" -");
        } else if self.is_960 {
            let mut asd = " ".to_string();
            if let Some(sq) = self.castling[Castling::Wksc] {
                asd += &((b'A' + sq.file()) as char).to_string();
            }
            if let Some(sq) = self.castling[Castling::Wqsc] {
                asd += &((b'A' + sq.file()) as char).to_string();
            }
            if let Some(sq) = self.castling[Castling::Bksc] {
                asd += &((b'a' + sq.file()) as char).to_string();
            }
            if let Some(sq) = self.castling[Castling::Bqsc] {
                asd += &((b'a' + sq.file()) as char).to_string();
            }
            fen.push_str(&asd);
        } else {
            let mut asd = " ".to_string();
            if self.castling[Castling::Wksc].is_some() {
                asd += "K";
            }
            if self.castling[Castling::Wqsc].is_some() {
                asd += "Q";
            }
            if self.castling[Castling::Bksc].is_some() {
                asd += "k";
            }
            if self.castling[Castling::Bqsc].is_some() {
                asd += "q";
            }
            fen.push_str(&asd);
        }

        // En passant
        if let Some(sq) = self.ep
            && self.legal_ep()
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
