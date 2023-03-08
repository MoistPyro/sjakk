use std::fmt::Display;

use crate::types::{Castle, Colour, PieceType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Piece {
    pub pos: [i8; 2],
    pub piece_type: PieceType,
    pub colour: Colour,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.piece_type)
    }
}

impl Piece {
    pub fn new<P, T, C>(pos: P, piece_type: T, colour: C) -> Self
    where
        P: Into<[i8; 2]>,
        T: Into<PieceType>,
        C: Into<Colour>,
    {
        let pos: [i8; 2] = pos.into();
        let piece_type: PieceType = piece_type.into();
        let colour: Colour = colour.into();

        Self {
            pos,
            piece_type,
            colour,
        }
    }

    pub fn get_move_tiles<I>(&self, castle: Castle) -> Vec<I>
    where
        Vec<I>: From<Vec<[i8; 2]>>,
    {
        match self.piece_type {
            PieceType::Pawn => match self.colour {
                Colour::White => {
                    if self.pos[1] == 1 {
                        vec![
                            [self.pos[0] + 0, self.pos[1] + 1],
                            [self.pos[0] + 0, self.pos[1] + 2],
                        ]
                    } else {
                        vec![[self.pos[0] + 0, self.pos[1] + 1]]
                    }
                }
                Colour::Black => {
                    if self.pos[1] == 6 {
                        vec![
                            [self.pos[0] + 0, self.pos[1] - 1],
                            [self.pos[0] + 0, self.pos[1] - 2],
                        ]
                    } else {
                        vec![[self.pos[0] + 0, self.pos[1] - 1]]
                    }
                }
            },
            PieceType::King => match castle {
                Castle::No => vec![
                    [self.pos[0] + 0, self.pos[1] + 1],
                    [self.pos[0] + 1, self.pos[1] + 1],
                    [self.pos[0] + 1, self.pos[1] + 0],
                    [self.pos[0] + 1, self.pos[1] - 1],
                    [self.pos[0] + 0, self.pos[1] - 1],
                    [self.pos[0] - 1, self.pos[1] - 1],
                    [self.pos[0] - 1, self.pos[1] + 0],
                    [self.pos[0] - 1, self.pos[1] + 1],
                ],
                Castle::Short(pos) => vec![[6, pos[1]]],
                Castle::Long(pos) => vec![[2, pos[1]]],
            },
            PieceType::Queen => {
                let mut temp: Vec<[i8; 2]> = vec![];
                for i in -8..8 {
                    temp.push([self.pos[0] + i, self.pos[1] + i]);
                    temp.push([self.pos[0] + i, self.pos[1] - i]);
                    temp.push([self.pos[0] + 0, self.pos[1] + i]);
                    temp.push([self.pos[0] + i, self.pos[1] + 0]);
                }
                temp
            }
            PieceType::Bishop => {
                let mut temp: Vec<[i8; 2]> = vec![];
                for i in -8..8 {
                    temp.push([self.pos[0] + i, self.pos[1] + i]);
                    temp.push([self.pos[0] + i, self.pos[1] - i]);
                }
                temp
            }
            PieceType::Knight => vec![
                [self.pos[0] + 1, self.pos[1] + 2],
                [self.pos[0] - 1, self.pos[1] + 2],
                [self.pos[0] + 2, self.pos[1] + 1],
                [self.pos[0] + 2, self.pos[1] - 1],
                [self.pos[0] - 2, self.pos[1] + 1],
                [self.pos[0] - 2, self.pos[1] - 1],
                [self.pos[0] + 1, self.pos[1] - 2],
                [self.pos[0] - 1, self.pos[1] - 2],
            ],
            PieceType::Rook => {
                let mut temp: Vec<[i8; 2]> = vec![];
                for i in -8..8 {
                    temp.push([self.pos[0] + 0, self.pos[1] + i]);
                    temp.push([self.pos[0] + i, self.pos[1] + 0]);
                }
                temp
            }
        }
        .into()
    }

    pub fn get_capture_tiles<I>(&self, castle: Castle) -> Vec<I>
    where
        Vec<I>: From<Vec<[i8; 2]>>,
    {
        match self.piece_type {
            PieceType::Pawn => match self.colour {
                Colour::White => vec![
                    [self.pos[0] + 1, self.pos[1] + 1],
                    [self.pos[0] - 1, self.pos[1] + 1],
                ]
                .into(),
                Colour::Black => vec![
                    [self.pos[0] + 1, self.pos[1] - 1],
                    [self.pos[0] - 1, self.pos[1] - 1],
                ]
                .into(),
            },
            _ => self.get_move_tiles(castle),
        }
    }
}
