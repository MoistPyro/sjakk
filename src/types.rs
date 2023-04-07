use std::{
    fmt::{Debug, Display},
    io::{Error, ErrorKind},
};

const _WHITE_PIECES: &str = "♙♔♕♗♘♖";
const _BLACK_PIECES: &str = "♟♚♛♝♞♜";

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn(Colour),
    King(Colour),
    Queen(Colour),
    Bishop(Colour),
    Knight(Colour),
    Rook(Colour),
    Empty,
}

impl Debug for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pawn(_) => write!(f, "P"),
            Self::King(_) => write!(f, "K"),
            Self::Queen(_) => write!(f, "Q"),
            Self::Bishop(_) => write!(f, "B"),
            Self::Knight(_) => write!(f, "N"),
            Self::Rook(_) => write!(f, "R"),
            Self::Empty => write!(f, " "),
        }
    }
}

impl Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pawn(Colour::White) => write!(f, "♙"),
            Self::Pawn(Colour::Black) => write!(f, "♟"),
            Self::King(Colour::White) => write!(f, "♔"),
            Self::King(Colour::Black) => write!(f, "♚"),
            Self::Queen(Colour::White) => write!(f, "♕"),
            Self::Queen(Colour::Black) => write!(f, "♛"),
            Self::Bishop(Colour::White) => write!(f, "♗"),
            Self::Bishop(Colour::Black) => write!(f, "♝"),
            Self::Knight(Colour::White) => write!(f, "♘"),
            Self::Knight(Colour::Black) => write!(f, "♞"),
            Self::Rook(Colour::White) => write!(f, "♖"),
            Self::Rook(Colour::Black) => write!(f, "♜"),
            Self::Empty => write!(f, " "),
        }
    }
}

impl PieceType {
    pub fn from_char(symbol: char) -> Result<Self, Error>
    {
        match symbol {
            '♙' => Ok(PieceType::Pawn(Colour::White)),
            '♟' => Ok(PieceType::Pawn(Colour::Black)),
            '♔' => Ok(PieceType::King(Colour::White)),
            '♚' => Ok(PieceType::King(Colour::Black)),
            '♕' => Ok(PieceType::Queen(Colour::White)),
            '♛' => Ok(PieceType::Queen(Colour::Black)),
            '♗' => Ok(PieceType::Bishop(Colour::White)),
            '♝' => Ok(PieceType::Bishop(Colour::Black)),
            '♘' => Ok(PieceType::Knight(Colour::White)),
            '♞' => Ok(PieceType::Knight(Colour::Black)),
            '♖' => Ok(PieceType::Rook(Colour::White)),
            '♜' => Ok(PieceType::Rook(Colour::Black)),
            _ => Err(Error::new(ErrorKind::InvalidInput, "not a valid identifier for a chess piece.")),
        }
    }

    pub fn from_char_and_colour<C>(symbol: char, colour: C) -> Self
    where
        C: Into<Colour>,
    {
        match symbol {
            'K' => PieceType::King(colour.into()),
            'Q' => PieceType::Queen(colour.into()),
            'B' => PieceType::Bishop(colour.into()),
            'N' => PieceType::Knight(colour.into()),
            'R' => PieceType::Rook(colour.into()),
            _ => PieceType::Pawn(colour.into()),
        }
    }

    pub fn from_notation<S, C>(value: S, colour: C) -> Result<Self, Error>
    where
        S: AsRef<str>,
        C: Into<Colour>
    {
        let mut piece: Option<PieceType> = None;
        let colour = colour.into();

        for symbol in "KQBNR".chars() {
            if let Some(pos) = value
                .as_ref()
                .find(|s: char| s.is_uppercase() && s == symbol)
            {
                piece = match pos {
                    0 => Some(PieceType::from_char_and_colour(symbol, colour)),
                    3 => Some(PieceType::Pawn(colour)),
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
                Some(_) => Some(PieceType::King(colour)),
                None => Some(PieceType::Pawn(colour)),
            };
        }
        piece.ok_or(Error::new(
            ErrorKind::InvalidInput,
            "not a valid chess move",
        ))
    }

    pub fn get_colour(&self) -> Option<Colour> {
        match self {
            PieceType::Pawn(c)
            | PieceType::King(c)
            | PieceType::Queen(c)
            | PieceType::Bishop(c)
            | PieceType::Knight(c)
            | PieceType::Rook(c) => Some(*c),
            PieceType::Empty => None,
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
        C: Into<Colour> + Copy,
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
    pub fn from_notation<S, C>(value: S, colour: C) -> Self
    where
        S: AsRef<str>,
        C: Into<Colour> + Copy,
    {
        let mut piece = None;

        for symbol in "KQBNR".chars() {
            if let Some(pos) = value
                .as_ref()
                .find(|s: char| s.is_uppercase() && s == symbol)
            {
                if pos == 3 {
                    piece = Some(PieceType::from_char_and_colour(symbol, colour))
                };
            }
        }
        match piece {
            Some(p) => Self::Yes(p),
            None => Self::No,
        }
    }
}
