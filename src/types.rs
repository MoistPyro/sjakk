use std::fmt::{Debug, Display};

const _WHITE_PIECES: &str = "♙♔♕♗♘♖";
const _BLACK_PIECES: &str = "♟♚♛♝♞♜";

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
}

impl From<char> for PieceType {
    fn from(value: char) -> Self {
        match value {
            'P' | '♙' | '♟' => Self::Pawn,
            'K' | '♔' | '♚' => Self::King,
            'Q' | '♕' | '♛' => Self::Queen,
            'B' | '♗' | '♝' => Self::Bishop,
            'N' | '♘' | '♞' => Self::Knight,
            'R' | '♖' | '♜' => Self::Rook,
            _ => panic!(),
        }
    }
}

impl From<&str> for PieceType {
    fn from(value: &str) -> Self {
        let mut piece: Option<PieceType> = None;

        for symbol in "KQBNR".chars() {
            if let Some(pos) = value.find(|s: char| s.is_uppercase() && s == symbol) {
                piece = match pos {
                    0 => Some(PieceType::from(symbol)),
                    3 => Some(PieceType::Pawn),
                    _ => panic!(),
                }
            }
        }

        if let None = piece {
            piece = match value.find('O') {
                Some(_) => Some(PieceType::King),
                None => Some(PieceType::Pawn),
            };
        }
        piece.unwrap()
    }
}

impl Debug for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pawn => write!(f, "P"),
            Self::King => write!(f, "K"),
            Self::Queen => write!(f, "Q"),
            Self::Bishop => write!(f, "B"),
            Self::Knight => write!(f, "N"),
            Self::Rook => write!(f, "R"),
        }
    }
}

impl Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pawn => write!(f, "♙"),
            Self::King => write!(f, "♔"),
            Self::Queen => write!(f, "♕"),
            Self::Bishop => write!(f, "♗"),
            Self::Knight => write!(f, "♘"),
            Self::Rook => write!(f, "♖"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colour {
    White,
    Black,
}

impl From<char> for Colour {
    fn from(value: char) -> Self {
        match value {
            'w' | 'W' | '♙' | '♔' | '♕' | '♗' | '♘' | '♖' => Self::White,
            'b' | 'B' | '♟' | '♚' | '♛' | '♝' | '♞' | '♜' => Self::Black,
            _ => panic!(),
        }
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Check {
    #[default]
    No,
    Check,
    Mate,
}

impl From<&str> for Check {
    fn from(value: &str) -> Self {
        if value.ends_with('+') {
            Self::Check
        } else if value.ends_with('#') {
            Self::Mate
        } else {
            Self::No
        }
    }
}

impl From<Check> for bool {
    fn from(value: Check) -> Self {
        match value {
            Check::No => false,
            _ => true,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Castle {
    #[default]
    No,
    Short(PieceType),
    Long(PieceType),
}

impl From<&str> for Castle {
    fn from(value: &str) -> Self {
        match value.chars().nth(0).unwrap() {
            'O' if value.len() <= 3 => Castle::Short(PieceType::Rook),
            'O' if value.len() > 3 => Castle::Long(PieceType::Rook),
            _ => Castle::No,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capture {
    Yes,
    No,
}

impl From<&str> for Capture {
    fn from(value: &str) -> Self {
        match value.find('x') {
            Some(_) => Capture::Yes,
            None => Capture::No,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Promotion {
    Yes(PieceType),
    No,
}

impl From<&str> for Promotion {
    fn from(value: &str) -> Self {
        let mut piece = None;

        for symbol in "KQBNR".chars() {
            if let Some(pos) = value.find(|s: char| s.is_uppercase() && s == symbol) {
                if pos == 3 {
                    piece = Some(PieceType::from(symbol))
                };
            }
        }
        match piece {
            Some(p) => Self::Yes(p),
            None => Self::No,
        }
    }
}
