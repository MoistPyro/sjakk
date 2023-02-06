use std::{
    array::IntoIter,
    io::{Error, ErrorKind},
};

use crate::types::{Capture, Castle, Check, Colour, PieceType, Promotion};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Turn {
    pub value: [Move; 2],
}

impl From<&str> for Turn {
    fn from(value: &str) -> Self {
        const COLOURS: [char; 2] = ['w', 'b'];

        let value: Result<Vec<Move>, Error> = value
            .split(" ")
            .zip(COLOURS)
            .map(|(s, c)| Move::new_from_notation(s, c))
            .collect();

        let binding = value.unwrap();
        let mut it = binding.iter();
        let r: [Move; 2] = [*it.next().unwrap(), *it.next().unwrap()];
        Self { value: r }
    }
}

impl IntoIterator for Turn {
    type Item = Move;

    type IntoIter = std::array::IntoIter<Move, 2>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::into_iter(self.value.into_iter())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move {
    pub piece: PieceType,
    pub castle: Castle,
    pub colour: Colour,
    pub capture: Capture,
    pub promotion: Promotion,
    pub check: Check,
    pub from: Option<usize>,
    pub to: Option<[i8; 2]>,
}

impl Move {
    pub fn new_from_notation<'a, S, C>(notation: S, colour: C) -> Result<Self, Error>
    where
        S: Into<&'a str>,
        C: Into<Colour>,
    {
        let notation = notation.into();

        let piece = PieceType::from(notation);
        let castle = Castle::from(notation);
        let colour = colour.into();
        let capture = Capture::from(notation);
        let promotion = Promotion::from(notation);
        let check = Check::from(notation);
        let from = None;
        let to = None;

        Ok(Self {
            piece,
            castle,
            colour,
            capture,
            promotion,
            check,
            from,
            to,
        }
        .remove_ambiguity(notation)?
        .set_destination(notation))
    }

    fn remove_ambiguity<'a, S>(mut self, notation: S) -> Result<Self, Error>
    where
        S: Into<&'a str>,
    {
        let notation = notation.into();
        let length = if self.check.into() { 3 } else { 2 };

        self.from = match self.piece {
            PieceType::Pawn => {
                if notation.len() > length {
                    "abcdefgh".find(
                        notation
                            .chars()
                            .nth(0)
                            .ok_or(Error::new(ErrorKind::InvalidInput, "notation is too short"))?,
                    )
                } else {
                    None
                }
            }
            PieceType::King | PieceType::Queen => None,
            _ => {
                if notation.len() > length + 1 {
                    "abcdefgh".find(
                        notation
                            .chars()
                            .nth(1)
                            .ok_or(Error::new(ErrorKind::InvalidInput, "notation is too short"))?,
                    )
                } else {
                    None
                }
            }
        };
        Ok(self)
    }

    fn set_destination<'a, S>(mut self, notation: S) -> Self
    where
        S: Into<&'a str>,
    {
        let notation: &str = notation.into();
        let offset: usize = self.get_offset();

        self.to = if let PieceType::King = self.piece {
            match self.castle {
                Castle::No => Move::format_destination(notation, offset),
                Castle::Short(_) => match self.colour {
                    Colour::White => Some([5, 0]),
                    Colour::Black => Some([5, 7]),
                },
                Castle::Long(_) => match self.colour {
                    Colour::White => Some([3, 0]),
                    Colour::Black => Some([3, 7]),
                },
            }
        } else {
            Move::format_destination(notation, offset)
        };

        self
    }

    fn get_offset(&self) -> usize {
        let mut offset = match self.capture {
            Capture::Yes => match self.from {
                Some(_) => 3,
                None => 2,
            },
            Capture::No => match self.from {
                Some(_) => 2,
                None => 1,
            },
        };

        if let PieceType::Pawn = self.piece {
            offset -= 1;
        }

        offset
    }

    fn format_destination(notation: &str, offset: usize) -> Option<[i8; 2]> {
        Some([
            "abcdefgh".find(notation.chars().nth(offset)?)? as i8,
            notation.chars().nth(offset + 1)?.to_digit(10)? as i8 - 1,
        ])
    }
}
