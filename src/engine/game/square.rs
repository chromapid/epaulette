use std::ops::{Add, AddAssign, Sub};

#[derive(Clone, Copy)]
pub struct Square {
    pos: u8,
}

impl Square {
    pub fn new(pos: u8) -> Self {
        Self { pos: pos }
    }

    pub fn is_on_board(self) -> bool {
        (self.pos & 0x88) == 0
    }

    pub fn as_index(self) -> u8 {
        // Perhaps investigate using a lookup table instead.
        (self.pos & 0x07) | ((self.pos & 0x70) >> 1)
    }

    pub fn from_index(index: u8) -> Self {
        // Perhaps investigate using a lookup table instead.
        let pos = (index & 0x07) | ((index & 0x1c) << 1);
        Self::new(pos)
    }
}

impl Add for Square {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.pos.wrapping_add(rhs.pos))
    }
}

impl Sub for Square {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.pos.wrapping_sub(rhs.pos))
    }
}
