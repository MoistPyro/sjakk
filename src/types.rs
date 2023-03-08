use std::{
    fmt::{Debug, Display},
    io::{Error, ErrorKind},
};

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

impl PieceType {
    pub fn from_notation<S>(value: S) -> Result<Self, Error>
    where
        S: AsRef<str>,
    {
        let mut piece: Option<PieceType> = None;

        for symbol in "KQBNR".chars() {
            if let Some(pos) = value
                .as_ref()
                .find(|s: char| s.is_uppercase() && s == symbol)
            {
                piece = match pos {
                    0 => Some(PieceType::from(symbol)),
                    3 => Some(PieceType::Pawn),
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            "not a valid chess move",
                        ))
                    }
                }
            }
        }

        if piece.is_none() {
            piece = match value.as_ref().find('O') {
                Some(_) => Some(PieceType::King),
                None => Some(PieceType::Pawn),
            };
        }
        piece.ok_or(Error::new(
            ErrorKind::InvalidInput,
            "not a valid chess move",
        ))
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

impl Check {
    pub fn from_notation<S>(value: S) -> Self
    where
        S: AsRef<str>,
    {
        if value.as_ref().ends_with('+') {
            Self::Check
        } else if value.as_ref().ends_with('#') {
            Self::Mate
        } else {
            Self::No
        }
    }

    pub fn is_check_or_mate(self) -> bool {
        match self {
            Check::No => false,
            _ => true,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Castle {
    #[default]
    No,
    Short([i8; 2]),
    Long([i8; 2]),
}

impl Castle {
    pub fn from_notation<S, C>(value: S, colour: C) -> Self
    where
        S: AsRef<str>,
        C: Into<Colour>,
    {
        let row: i8 = match colour.into() {
            Colour::White => 0,
            Colour::Black => 7,
        };
        match value.as_ref().chars().nth(0).unwrap() {
            'O' if value.as_ref().len() <= 3 => Castle::Short([7, row]),
            'O' if value.as_ref().len() > 3 => Castle::Long([0, row]),
            _ => Castle::No,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capture {
    Yes,
    No,
}

impl Capture {
    pub fn from_notation<S>(value: S) -> Self
    where
        S: AsRef<str>,
    {
        match value.as_ref().find('x') {
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

impl Promotion {
    pub fn from_notation<S>(value: S) -> Self
    where
        S: AsRef<str>,
    {
        let mut piece = None;

        for symbol in "KQBNR".chars() {
            if let Some(pos) = value
                .as_ref()
                .find(|s: char| s.is_uppercase() && s == symbol)
            {
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
