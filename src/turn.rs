use std::io::{Error, ErrorKind};

use crate::types::{Capture, Castle, Check, Colour, PieceType, Promotion};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Turn {
    pub value: [Move; 2],
}

impl Turn {
    pub fn new_from_notation<S>(value: S) -> Result<Self, Error>
    where
        S: AsRef<str>,
    {
        const COLOURS: [char; 2] = ['w', 'b'];

        let moves_from_str: Vec<Move> = value
            .as_ref()
            .split(" ")
            .zip(COLOURS)
            .map(|(s, c)| Move::new_from_notation(s, c))
            .collect::<Result<Vec<Move>, Error>>()?;

        Ok(Self {
            value: [
                *moves_from_str.get(0).unwrap(),
                *moves_from_str.get(1).unwrap(),
            ],
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move {
    pub piece: PieceType,
    pub castle: Castle,
    pub capture: Capture,
    pub promotion: Promotion,
    pub check: Check,
    pub from: Option<usize>,
    pub to: Option<[i8; 2]>,
}

impl Move {
    pub fn new_from_notation<S, C>(notation: S, colour: C) -> Result<Self, Error>
    where
        S: AsRef<str>,
        C: Into<Colour> + Copy,
    {
        let piece: PieceType = PieceType::from_notation(&notation, colour)?;
        let castle: Castle = Castle::from_notation(&notation, colour);
        let capture: Capture = Capture::from_notation(&notation);
        let promotion: Promotion = Promotion::from_notation(&notation, colour);
        let check: Check = Check::from_notation(&notation);
        let from: Option<usize> = None;
        let to: Option<[i8; 2]> = None;

        Ok(Self {
            piece,
            castle,
            capture,
            promotion,
            check,
            from,
            to,
        }
        .remove_ambiguity(&notation)?
        .set_destination(&notation))
    }

    fn remove_ambiguity<S>(mut self, notation: S) -> Result<Self, Error>
    where
        S: AsRef<str>,
    {
        let notation = notation.as_ref();
        let length = if self.check.is_check_or_mate() { 3 } else { 2 };

        self.from = match self.piece {
            PieceType::Pawn(_) => {
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
            PieceType::King(_) | PieceType::Queen(_) => None,
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

    fn set_destination<S>(mut self, notation: S) -> Self
    where
        S: AsRef<str>,
    {
        let offset: usize = self.get_offset();

        self.to = if let PieceType::King(_) = self.piece {
            match self.castle {
                Castle::No => Move::format_destination(notation, offset),
                Castle::Short(pos) => Some([6, pos[1]]),
                Castle::Long(pos) => Some([2, pos[1]]),
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

        if let PieceType::Pawn(_) = self.piece {
            offset -= 1;
        }

        offset
    }

    fn format_destination<S>(notation: S, offset: usize) -> Option<[i8; 2]>
    where
        S: AsRef<str>,
    {
        Some([
            "abcdefgh".find(notation.as_ref().chars().nth(offset)?)? as i8,
            notation.as_ref().chars().nth(offset + 1)?.to_digit(10)? as i8 - 1,
        ])
    }
}
