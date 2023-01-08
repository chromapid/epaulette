use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, Not};

use super::Square;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bitboard {
    info: u64,
}

impl Bitboard {
    pub fn new(bits: u64) -> Self {
        Self { info: bits }
    }

    pub fn is_empty(self) -> bool {
        self.info == 0
    }

    pub fn at_square(self, square: Square) -> bool {
        !((self & Self::from(square)).is_empty())
    }
}

impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<Square> for Bitboard {
    fn from(value: Square) -> Self {
        Self::new(1u64 << value.as_index())
    }
}

impl Default for Bitboard {
    fn default() -> Self {
        Self::new(0)
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(self.info & rhs.info)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::new(self.info | rhs.info)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::new(self.info ^ rhs.info)
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Self::new(!self.info)
    }
}
