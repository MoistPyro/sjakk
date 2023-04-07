use std::fmt::Display;

use crate::types::{Castle, Colour, PieceType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Piece {
    pub pos: [i8; 2],
    pub piece_type: PieceType,
}

impl PartialOrd for Piece {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a_1d = self.pos[0] + (self.pos[1] * 8);
        let b_1d = other.pos[0] + (other.pos[1] * 8);

        a_1d.partial_cmp(&b_1d)
    }
}

impl Ord for Piece {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a_1d = self.pos[0] + (self.pos[1] * 8);
        let b_1d = other.pos[0] + (other.pos[1] * 8);

        a_1d.cmp(&b_1d)
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.piece_type)
    }
}

impl Piece {
    pub fn new<P, T>(pos: P, piece_type: T) -> Self
    where
        P: Into<[i8; 2]>,
        T: Into<PieceType>,
    {
        let pos: [i8; 2] = pos.into();
        let piece_type: PieceType = piece_type.into();

        Self { pos, piece_type }
    }

    pub fn get_move_tiles<I>(&self, castle: Castle) -> Vec<I>
    where
        Vec<I>: From<Vec<[i8; 2]>>,
    {
        match self.piece_type {
            PieceType::Pawn(Colour::White) => {
                if self.pos[1] == 1 {
                    vec![
                        [self.pos[0] + 0, self.pos[1] + 1],
                        [self.pos[0] + 0, self.pos[1] + 2],
                    ]
                } else {
                    vec![[self.pos[0] + 0, self.pos[1] + 1]]
                }
            }
            PieceType::Pawn(Colour::Black) => {
                if self.pos[1] == 6 {
                    vec![
                        [self.pos[0] + 0, self.pos[1] - 1],
                        [self.pos[0] + 0, self.pos[1] - 2],
                    ]
                } else {
                    vec![[self.pos[0] + 0, self.pos[1] - 1]]
                }
            }
            PieceType::King(_) => match castle {
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
            PieceType::Queen(_) => {
                let mut temp: Vec<[i8; 2]> = vec![];
                for i in -8..8 {
                    temp.push([self.pos[0] + i, self.pos[1] + i]);
                    temp.push([self.pos[0] + i, self.pos[1] - i]);
                    temp.push([self.pos[0] + 0, self.pos[1] + i]);
                    temp.push([self.pos[0] + i, self.pos[1] + 0]);
                }
                temp
            }
            PieceType::Bishop(_) => {
                let mut temp: Vec<[i8; 2]> = vec![];
                for i in -8..8 {
                    temp.push([self.pos[0] + i, self.pos[1] + i]);
                    temp.push([self.pos[0] + i, self.pos[1] - i]);
                }
                temp
            }
            PieceType::Knight(_) => vec![
                [self.pos[0] + 1, self.pos[1] + 2],
                [self.pos[0] - 1, self.pos[1] + 2],
                [self.pos[0] + 2, self.pos[1] + 1],
                [self.pos[0] + 2, self.pos[1] - 1],
                [self.pos[0] - 2, self.pos[1] + 1],
                [self.pos[0] - 2, self.pos[1] - 1],
                [self.pos[0] + 1, self.pos[1] - 2],
                [self.pos[0] - 1, self.pos[1] - 2],
            ],
            PieceType::Rook(_) => {
                let mut temp: Vec<[i8; 2]> = vec![];
                for i in -8..8 {
                    temp.push([self.pos[0] + 0, self.pos[1] + i]);
                    temp.push([self.pos[0] + i, self.pos[1] + 0]);
                }
                temp
            }
            PieceType::Empty => panic!("hit empty variant."),
        }
        .into()
    }

    pub fn get_capture_tiles<I>(&self, castle: Castle) -> Vec<I>
    where
        Vec<I>: From<Vec<[i8; 2]>>,
    {
        match self.piece_type {
            PieceType::Pawn(Colour::White) => vec![
                [self.pos[0] + 1, self.pos[1] + 1],
                [self.pos[0] - 1, self.pos[1] + 1],
            ]
            .into(),
            PieceType::Pawn(Colour::Black) => vec![
                [self.pos[0] + 1, self.pos[1] - 1],
                [self.pos[0] - 1, self.pos[1] - 1],
            ]
            .into(),
            _ => self.get_move_tiles(castle),
        }
    }
}
