mod bitboard;
mod moveinfo;
mod piece;
mod square;

pub use self::bitboard::Bitboard;
pub use self::square::Square;

pub struct Squareboard<T: Copy> {
    squares: [T; 64],
}

#[derive(Clone, Copy)]
#[repr(align(2))]
pub struct AttackCounts {
    friendly: u8,
    enemy: u8,
}

pub struct GameState {
    attacks: Squareboard<AttackCounts>,
    occupied: Bitboard,
    friendly: Bitboard,
    pinned: Bitboard,
    en_passant_square: Square,
    castle_rights: u8,
}
