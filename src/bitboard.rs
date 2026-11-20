use bevy::prelude::*;
#[derive(Debug, Default, Hash, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct BitBoard(pub u64);

pub struct Position{
    sides: [BitBoard; 2],
    pieces: [[BitBoard; 6]; 2],
    state: State,
}

pub struct Sides;

impl Sides{
    pub const WHITE: usize = 0;
    pub const BLACK: usize = 1;
}


pub struct Pieces;

impl Pieces {
    pub const PAWN: usize = 0;
    pub const BISHOP: usize = 1;
    pub const KNIGHT: usize = 2;
    pub const ROOK: usize = 3;
    pub const QUEEN: usize = 4;
    pub const KING: usize = 5; 
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct State {
    castling_rights: CastlingRights,
    en_passant_square: Option<Square>,
    half_move_counter: u8,
    side_to_move: usize,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CastlingRights(u8);

impl CastlingRights {
    fn empty() -> Self {
        Self(Castling::NO_CASTLING)
    }
    fn all() -> Self {
        Self::default()
    }
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self(Castling::ANY_CASTLING)
    }
}

/// Provides labels for the [`CastlingRights`]
pub struct Castling;
impl Castling {
    pub const NO_CASTLING: u8 = 0;
    pub const WHITE_00: u8 = 0b00000001;
    pub const WHITE_000: u8 = 0b00000010;
    pub const BLACK_00: u8 = 0b00000100;
    pub const BLACK_000: u8 = 0b00001000;

    pub const KING_SIDE: u8 = Self::BLACK_00 | Self::WHITE_00;
    pub const QUEEN_SIDE: u8 = Self::BLACK_000 | Self::WHITE_000;
    pub const WHITE_CASTLING: u8 = Self::WHITE_00 | Self::WHITE_000;
    pub const BLACK_CASTLING: u8 = Self::BLACK_00 | Self::BLACK_000;
    pub const ANY_CASTLING: u8 = Self::BLACK_CASTLING | Self::WHITE_CASTLING;
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
/// Represents a single square on the board.
/// # Representation
/// 1 is A1
/// 2 is B1
/// 64 is H8
#[derive(Component)]
pub struct Square(pub usize);

/// Labels for every [`Square`] on the board.
#[repr(usize)]
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SquareLabels {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
    None,
}

impl SquareLabels {
    pub const fn from_index(index_0_to_63: usize) -> Self {
        match index_0_to_63 {
            0 => Self::A1, 1 => Self::B1, 2 => Self::C1, 3 => Self::D1,
            4 => Self::E1, 5 => Self::F1, 6 => Self::G1, 7 => Self::H1,
            8 => Self::A2, 9 => Self::B2, 10 => Self::C2, 11 => Self::D2,
            12 => Self::E2, 13 => Self::F2, 14 => Self::G2, 15 => Self::H2,
            16 => Self::A3, 17 => Self::B3, 18 => Self::C3, 19 => Self::D3,
            20 => Self::E3, 21 => Self::F3, 22 => Self::G3, 23 => Self::H3,
            24 => Self::A4, 25 => Self::B4, 26 => Self::C4, 27 => Self::D4,
            28 => Self::E4, 29 => Self::F4, 30 => Self::G4, 31 => Self::H4,
            32 => Self::A5, 33 => Self::B5, 34 => Self::C5, 35 => Self::D5,
            36 => Self::E5, 37 => Self::F5, 38 => Self::G5, 39 => Self::H5,
            40 => Self::A6, 41 => Self::B6, 42 => Self::C6, 43 => Self::D6,
            44 => Self::E6, 45 => Self::F6, 46 => Self::G6, 47 => Self::H6,
            48 => Self::A7, 49 => Self::B7, 50 => Self::C7, 51 => Self::D7,
            52 => Self::E7, 53 => Self::F7, 54 => Self::G7, 55 => Self::H7,
            56 => Self::A8, 57 => Self::B8, 58 => Self::C8, 59 => Self::D8,
            60 => Self::E8, 61 => Self::F8, 62 => Self::G8, 63 => Self::H8,
            _ => Self::None,
        }
    }
}