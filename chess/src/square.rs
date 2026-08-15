use crate::error::ParseError;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Square(pub(crate) u8);

impl Square {
    #[must_use]
    pub fn from_index(idx: u8) -> Self {
        Self(idx)
    }

    #[must_use]
    pub fn from_indices(x: u8, y: u8) -> Self {
        Self::from_index(8 * y + x)
    }

    #[must_use]
    pub fn from_string(s: &str) -> Result<Self, ParseError> {
        let [file, rank] = s.as_bytes() else {
            return Err(ParseError::InvalidSquare);
        };

        if !(b'a'..=b'h').contains(file) {
            return Err(ParseError::InvalidSquare);
        }

        if !(b'1'..=b'8').contains(rank) {
            return Err(ParseError::InvalidSquare);
        }

        Ok(Self::from_indices(file - b'a', rank - b'1'))
    }

    #[must_use]
    fn to_string(&self) -> String {
        format!(
            "{}{}",
            (b'a' + self.0 % 8) as char,
            (b'1' + self.0 / 8) as char
        )
    }

    #[must_use]
    pub fn rank(&self) -> u8 {
        self.0 / 8
    }

    #[must_use]
    pub fn file(&self) -> u8 {
        self.0 % 8
    }

    #[must_use]
    pub fn north(&self) -> Self {
        Self(self.0 + 8)
    }

    #[must_use]
    pub fn south(&self) -> Self {
        Self(self.0 - 8)
    }
}

impl<T> std::ops::Index<Square> for [T; 64] {
    type Output = T;

    fn index(&self, sq: Square) -> &T {
        &self[sq.0 as usize]
    }
}

impl<T> std::ops::IndexMut<Square> for [T; 64] {
    fn index_mut(&mut self, sq: Square) -> &mut T {
        &mut self[sq.0 as usize]
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn legal() {
        let valid = ["a1", "a8", "h1", "h8", "e4", "c5"];

        for a in valid {
            assert!(Square::from_string(a).is_ok(), "No worky");
        }
    }

    #[test]
    fn illegal() {
        let invalid = ["", "a0", "a9", "h9", "i1", "a1a1", "a10", "aa", "11"];

        for a in invalid {
            assert!(Square::from_string(a).is_err(), "No worky");
        }
    }
}
