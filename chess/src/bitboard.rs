use crate::square::Square;
use std::{
    fmt,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub(crate) u64);
impl Bitboard {
    pub fn new(mask: u64) -> Self {
        Self(mask)
    }

    pub fn empty() -> Self {
        Self(0x0)
    }

    pub fn all() -> Self {
        !Self::empty()
    }

    pub fn from_square(sq: Square) -> Self {
        Self(1u64 << sq.0)
    }

    #[must_use]
    pub fn is_set(&self, sq: Square) -> bool {
        (self.0 >> sq.0) & 1 == 1
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0 == 0x0
    }

    #[must_use]
    pub fn is_occupied(&self) -> bool {
        !self.is_empty()
    }

    #[must_use]
    pub fn lsb(&self) -> Option<Square> {
        Some(Square::from_index(self.0.lowest_one()? as u8))
    }

    #[must_use]
    pub fn rank_of(sq: Square) -> Self {
        Self::new(0xFF << (8 * sq.rank()))
    }

    #[must_use]
    pub fn file_of(sq: Square) -> Self {
        Self::new(0x101010101010101 << sq.file())
    }

    #[must_use]
    pub fn ne_sw_of(sq: Square) -> Self {
        // Self::new(0x8040201008040200 << sq.0) | Self::new(0x40201008040201 << (63 - sq.0))
        Self::ray_ne(sq, Bitboard::empty()) | Self::ray_sw(sq, Bitboard::empty())
    }

    #[must_use]
    pub fn nw_se_of(sq: Square) -> Self {
        Self::ray_nw(sq, Bitboard::empty()) | Self::ray_se(sq, Bitboard::empty())
    }

    #[must_use]
    pub fn rook_of(sq: Square) -> Self {
        Self::rank_of(sq) | Self::file_of(sq)
    }

    #[must_use]
    pub fn count(&self) -> u32 {
        self.0.count_ones()
    }

    #[must_use]
    pub fn adjacent(self) -> Self {
        Bitboard(
            // North
            (self.0 << 8) |
            // South
            (self.0 >> 8) |
            // north west, south west, west
            (((self.0 << 7) | (self.0 >> 9) | (self.0 >> 1)) & 0x7f7f7f7f7f7f7f7f) |
            // north east, south east, east
              (((self.0 >> 7) | (self.0 << 9) | (self.0 << 1)) & 0xfefefefefefefefe),
        )
    }

    #[must_use]
    pub fn knights(sq: Square) -> Self {
        let bb = 1u64 << sq.0;
        Self(
            // right 1
            (bb << 17 | bb >> 15) & 0xfefefefefefefefe |
            // left 1
            (bb << 15 | bb >> 17) & 0x7f7f7f7f7f7f7f7f |
            // right 2
            (bb << 10 | bb >> 6) & 0xfcfcfcfcfcfcfcfc |
            // left 2
            (bb << 6 | bb >> 10) & 0x3f3f3f3f3f3f3f3f,
        )
    }

    #[must_use]
    pub fn north(&self) -> Self {
        Self(self.0 << 8)
    }

    #[must_use]
    pub fn south(&self) -> Self {
        Self(self.0 >> 8)
    }

    #[must_use]
    pub fn east(&self) -> Self {
        unimplemented!()
    }

    #[must_use]
    pub fn west(&self) -> Self {
        unimplemented!()
    }

    #[must_use]
    pub fn ne(&self) -> Self {
        (*self << 9) & Bitboard::new(0xfefefefefefefefe)
    }

    #[must_use]
    pub fn nw(&self) -> Self {
        (*self << 7) & Bitboard::new(0x7f7f7f7f7f7f7f7f)
    }

    #[must_use]
    pub fn se(&self) -> Self {
        (*self >> 7) & Bitboard::new(0xfefefefefefefefe)
    }

    #[must_use]
    pub fn sw(&self) -> Self {
        (*self >> 9) & Bitboard::new(0x7f7f7f7f7f7f7f7f)
    }

    #[must_use]
    pub fn ray_n(sq: Square, blockers: Self) -> Self {
        let mut bb = Self::from_square(sq) << 8;
        bb |= (bb & !blockers) << 8;
        bb |= (bb & !blockers) << 8;
        bb |= (bb & !blockers) << 8;
        bb |= (bb & !blockers) << 8;
        bb |= (bb & !blockers) << 8;
        bb |= (bb & !blockers) << 8;
        bb
    }

    #[must_use]
    pub fn ray_s(sq: Square, blockers: Self) -> Self {
        let mut bb = Self::from_square(sq) >> 8;
        bb |= (bb & !blockers) >> 8;
        bb |= (bb & !blockers) >> 8;
        bb |= (bb & !blockers) >> 8;
        bb |= (bb & !blockers) >> 8;
        bb |= (bb & !blockers) >> 8;
        bb |= (bb & !blockers) >> 8;
        bb
    }

    #[must_use]
    pub fn ray_e(sq: Square, blockers: Self) -> Self {
        let mut bb = (Self::from_square(sq) << 1) & Bitboard::new(0xfefefefefefefefe);
        bb |= ((bb & !blockers) << 1) & Bitboard::new(0xfefefefefefefefe);
        bb |= ((bb & !blockers) << 1) & Bitboard::new(0xfefefefefefefefe);
        bb |= ((bb & !blockers) << 1) & Bitboard::new(0xfefefefefefefefe);
        bb |= ((bb & !blockers) << 1) & Bitboard::new(0xfefefefefefefefe);
        bb |= ((bb & !blockers) << 1) & Bitboard::new(0xfefefefefefefefe);
        bb |= ((bb & !blockers) << 1) & Bitboard::new(0xfefefefefefefefe);
        bb
    }

    #[must_use]
    pub fn ray_w(sq: Square, blockers: Self) -> Self {
        let mut bb = (Self::from_square(sq) >> 1) & Bitboard::new(0x7f7f7f7f7f7f7f7f);
        bb |= ((bb & !blockers) >> 1) & Bitboard::new(0x7f7f7f7f7f7f7f7f);
        bb |= ((bb & !blockers) >> 1) & Bitboard::new(0x7f7f7f7f7f7f7f7f);
        bb |= ((bb & !blockers) >> 1) & Bitboard::new(0x7f7f7f7f7f7f7f7f);
        bb |= ((bb & !blockers) >> 1) & Bitboard::new(0x7f7f7f7f7f7f7f7f);
        bb |= ((bb & !blockers) >> 1) & Bitboard::new(0x7f7f7f7f7f7f7f7f);
        bb |= ((bb & !blockers) >> 1) & Bitboard::new(0x7f7f7f7f7f7f7f7f);
        bb
    }

    #[must_use]
    pub fn ray_ne(sq: Square, blockers: Self) -> Self {
        let mut bb = (Self::from_square(sq) << 9) & Bitboard::new(0xfefefefefefefefe);
        bb |= ((bb & !blockers) << 9) & Bitboard::new(0xfefefefefefefefe);
        bb |= ((bb & !blockers) << 9) & Bitboard::new(0xfefefefefefefefe);
        bb |= ((bb & !blockers) << 9) & Bitboard::new(0xfefefefefefefefe);
        bb |= ((bb & !blockers) << 9) & Bitboard::new(0xfefefefefefefefe);
        bb |= ((bb & !blockers) << 9) & Bitboard::new(0xfefefefefefefefe);
        bb |= ((bb & !blockers) << 9) & Bitboard::new(0xfefefefefefefefe);
        bb
    }

    #[must_use]
    pub fn ray_se(sq: Square, blockers: Self) -> Self {
        let mut bb = (Self::from_square(sq) >> 7) & Bitboard::new(0xfefefefefefefefe);
        bb |= ((bb & !blockers) >> 7) & Bitboard::new(0xfefefefefefefefe);
        bb |= ((bb & !blockers) >> 7) & Bitboard::new(0xfefefefefefefefe);
        bb |= ((bb & !blockers) >> 7) & Bitboard::new(0xfefefefefefefefe);
        bb |= ((bb & !blockers) >> 7) & Bitboard::new(0xfefefefefefefefe);
        bb |= ((bb & !blockers) >> 7) & Bitboard::new(0xfefefefefefefefe);
        bb |= ((bb & !blockers) >> 7) & Bitboard::new(0xfefefefefefefefe);
        bb
    }

    #[must_use]
    pub fn ray_nw(sq: Square, blockers: Self) -> Self {
        let mut bb = (Self::from_square(sq) << 7) & Bitboard::new(0x7f7f7f7f7f7f7f7f);
        bb |= ((bb & !blockers) << 7) & Bitboard::new(0x7f7f7f7f7f7f7f7f);
        bb |= ((bb & !blockers) << 7) & Bitboard::new(0x7f7f7f7f7f7f7f7f);
        bb |= ((bb & !blockers) << 7) & Bitboard::new(0x7f7f7f7f7f7f7f7f);
        bb |= ((bb & !blockers) << 7) & Bitboard::new(0x7f7f7f7f7f7f7f7f);
        bb |= ((bb & !blockers) << 7) & Bitboard::new(0x7f7f7f7f7f7f7f7f);
        bb |= ((bb & !blockers) << 7) & Bitboard::new(0x7f7f7f7f7f7f7f7f);
        bb
    }

    #[must_use]
    pub fn ray_sw(sq: Square, blockers: Self) -> Self {
        let mut bb = (Self::from_square(sq) >> 9) & Bitboard::new(0x7f7f7f7f7f7f7f7f);
        bb |= ((bb & !blockers) >> 9) & Bitboard::new(0x7f7f7f7f7f7f7f7f);
        bb |= ((bb & !blockers) >> 9) & Bitboard::new(0x7f7f7f7f7f7f7f7f);
        bb |= ((bb & !blockers) >> 9) & Bitboard::new(0x7f7f7f7f7f7f7f7f);
        bb |= ((bb & !blockers) >> 9) & Bitboard::new(0x7f7f7f7f7f7f7f7f);
        bb |= ((bb & !blockers) >> 9) & Bitboard::new(0x7f7f7f7f7f7f7f7f);
        bb |= ((bb & !blockers) >> 9) & Bitboard::new(0x7f7f7f7f7f7f7f7f);
        bb
    }

    #[must_use]
    pub fn ray_ew(sq: Square, blockers: Self) -> Self {
        Self::ray_e(sq, blockers) | Self::ray_w(sq, blockers)
    }

    #[must_use]
    pub fn ray_ns(sq: Square, blockers: Self) -> Self {
        Self::ray_n(sq, blockers) | Self::ray_s(sq, blockers)
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl Shl<usize> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shr<usize> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in (0..8).rev() {
            for x in 0..8 {
                let idx = 8 * y + x;
                write!(f, "{}", (self.0 >> idx) & 1)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{bitboard::Bitboard, square::Square};

    #[test]
    fn rays_north() {
        assert_eq!(
            Bitboard::ray_n(Square::from_index(0), Bitboard::empty()),
            Bitboard::new(0x101010101010100)
        );
        assert_eq!(
            Bitboard::ray_n(Square::from_index(0), Bitboard::new(0x1000000)),
            Bitboard::new(0x1010100)
        );
    }

    #[test]
    fn line_ne_sw() {
        assert_eq!(
            Bitboard::ne_sw_of(Square::from_index(0)),
            Bitboard::new(0x8040201008040200),
        );
        assert_eq!(
            Bitboard::ne_sw_of(Square::from_index(63)),
            Bitboard::new(0x40201008040201),
        );
        assert_eq!(
            Bitboard::ne_sw_of(Square::from_index(36)),
            Bitboard::new(0x8040200008040201),
        );
    }
}
