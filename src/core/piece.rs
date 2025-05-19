use strum::{Display, EnumIs};
/// Represents any type of chess piece. Color-agnostic.
#[derive(Clone, Copy, Debug, Display, EnumIs, Hash, Eq, PartialEq)]
#[repr(u8)]
pub enum PieceType {
    Empty = 0b0000,
    Pawn = 0b0001,
    Knight = 0b0010,
    Bishop = 0b0011,
    Rook = 0b0100,
    Queen = 0b0101,
    King = 0b0110,
}

impl PieceType {
    pub fn from_bits(raw: u8) -> Option<Self> {
        match raw {
            0b0000 => Some(Self::Empty),
            0b0001 => Some(Self::Pawn),
            0b0010 => Some(Self::Knight),
            0b0011 => Some(Self::Bishop),
            0b0100 => Some(Self::Rook),
            0b0101 => Some(Self::Queen),
            0b0110 => Some(Self::King),
            _ => None
        }
    }
}

/// The color of a chess piece.
#[derive(Clone, Copy, Debug, Display, EnumIs, Hash, Eq, PartialEq)]
#[repr(u8)]
pub enum PieceColor {
    White = 0b0000,
    Black = 0b1000,
}

impl PieceColor {
    pub fn from_bits(raw: u8) -> Option<Self> {
        match raw {
            0b1000 => Some(Self::Black),
            0b0000 => Some(Self::White),
            _ => None
        }
    }
}

/// A given piece. Contains color information as well as type. 
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub piece_color: PieceColor,
}

impl Piece {
    pub fn new(piece_type: PieceType, piece_color: PieceColor) -> Self {
        Self {
            piece_type,
            piece_color
        }
    }
    /// Pieces are represented in the format CTTT where C is the color bit and T are type bits.
    pub fn to_raw(&self) -> u8 {
        self.piece_color as u8 | self.piece_type as u8
    }
    /// Pieces are represented in the format CTTT where C is the color bit and T are the type bits.
    pub fn from_raw(raw: u8) -> Self {
        let piece_color = PieceColor::from_bits(raw & 0b1000).expect("Failed to create piece color from raw");
        let piece_type = PieceType::from_bits(raw & 0b0111).expect("Failed to create piece type from raw");
        Self {
            piece_type,
            piece_color
        }
    }
}