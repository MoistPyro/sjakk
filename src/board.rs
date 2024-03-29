use std::fmt::Display;

use crate::{
    piece::Piece,
    types::{Colour, PieceType},
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality() {
        let a = Board::default();
        let b = Board::default();

        assert_eq!(a, b);
    }

    #[test]
    fn test_find_piece() {
        let mut start_board = Board::default();

        assert_eq!(start_board.find_piece_by_pos(0, 0).unwrap(), 0);
        assert_eq!(start_board.find_piece_by_pos(7, 0).unwrap(), 1);
        assert!(start_board.find_piece_by_pos(3, 3).is_none());
    }

    #[test]
    fn test_tiles_between() {
        let q_to_q = Board::get_tiles_between([3, 0], [3, 7]).unwrap();

        let expected_result = vec![[3, 1], [3, 2], [3, 3], [3, 4], [3, 5], [3, 6]];

        assert_eq!(q_to_q, expected_result);

        let highly_spesific_case = Board::get_tiles_between([3, 0], [7, 0]).unwrap();
        let expected_result = vec![[4, 0], [5, 0], [6, 0]];

        assert_eq!(highly_spesific_case, expected_result);

        let negative_test = Board::get_tiles_between([7,0], [3,0]).unwrap();
        let expected_result = vec![[6, 0], [5, 0], [4, 0]];

        assert_eq!(negative_test, expected_result);
    }

    #[test]
    fn test_intervention() {
        let mut initial_board = Board::default();
        let r_to_r = Board::get_tiles_between([0,0], [7,0]).unwrap();
        let interventions = initial_board.get_intervening_pieces(&r_to_r);

        assert_eq!(interventions.len(), 6);

        let mut empty_board = Board::_blank();

        empty_board.pieces.push(Piece::new([7,0], PieceType::Rook(Colour::White)));
        empty_board.pieces.push(Piece::new([5,0], PieceType::Bishop(Colour::Black)));
        empty_board.pieces.push(Piece::new([3, 4], PieceType::Rook(Colour::White)));

        let wrong_rook = Board::get_tiles_between([7,0], [3,0]).unwrap();
        let correct_rook = Board::get_tiles_between([3,4], [3,0]).unwrap();

        let i1 = empty_board.get_intervening_pieces(&wrong_rook);
        assert_eq!(i1.len(), 1);

        let i2 = empty_board.get_intervening_pieces(&correct_rook);
        assert_eq!(i2.len(), 0);
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    pub pieces: Vec<Piece>,
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        let mut a = self.pieces.clone();
        let mut b = other.pieces.clone();
        a.sort() == b.sort()
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut pieces = vec![];

        pieces.push(Piece::new([0, 0], PieceType::Rook(Colour::White)));
        pieces.push(Piece::new([7, 0], PieceType::Rook(Colour::White)));
        pieces.push(Piece::new([0, 7], PieceType::Rook(Colour::Black)));
        pieces.push(Piece::new([7, 7], PieceType::Rook(Colour::Black)));

        pieces.push(Piece::new([1, 0], PieceType::Knight(Colour::White)));
        pieces.push(Piece::new([6, 0], PieceType::Knight(Colour::White)));
        pieces.push(Piece::new([1, 7], PieceType::Knight(Colour::Black)));
        pieces.push(Piece::new([6, 7], PieceType::Knight(Colour::Black)));

        pieces.push(Piece::new([2, 0], PieceType::Bishop(Colour::White)));
        pieces.push(Piece::new([5, 0], PieceType::Bishop(Colour::White)));
        pieces.push(Piece::new([2, 7], PieceType::Bishop(Colour::Black)));
        pieces.push(Piece::new([5, 7], PieceType::Bishop(Colour::Black)));

        pieces.push(Piece::new([3, 0], PieceType::Queen(Colour::White)));
        pieces.push(Piece::new([3, 7], PieceType::Queen(Colour::Black)));

        pieces.push(Piece::new([4, 0], PieceType::King(Colour::White)));
        pieces.push(Piece::new([4, 7], PieceType::King(Colour::Black)));

        for i in 0..8 {
            pieces.push(Piece::new([i, 1], PieceType::Pawn(Colour::White)));
            pieces.push(Piece::new([i, 6], PieceType::Pawn(Colour::Black)));
        }

        Self { pieces }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        let mut board_1d: [PieceType; 64] = [PieceType::Empty(Colour::White); 64];
        
        for i in 0..board_1d.len() {
            board_1d[i] = match self
            .pieces
            .iter()
            .filter(|x| x.pos == [(i % 8) as i8, (i as f64 / 8.0).floor() as i8])
            .nth(0)
            {
                Some(p) => p.piece_type,
                None => PieceType::Empty(Colour::White),
            };
        }
        
        let board_strs: Vec<String> = board_1d.iter().map(|p| format!("{p}")).collect();
        
        write!(f, "\nX a b c d e f g h X\n")?;

        write!(f, "1 {} 1\n", board_strs[0..8].join(" "))?;
        write!(f, "2 {} 2\n", board_strs[8..16].join(" "))?;
        write!(f, "3 {} 3\n", board_strs[16..24].join(" "))?;
        write!(f, "4 {} 4\n", board_strs[24..32].join(" "))?;
        write!(f, "5 {} 5\n", board_strs[32..40].join(" "))?;
        write!(f, "6 {} 6\n", board_strs[40..48].join(" "))?;
        write!(f, "7 {} 7\n", board_strs[48..56].join(" "))?;
        write!(f, "8 {} 8\n", board_strs[56..64].join(" "))?;

        write!(f, "X a b c d e f g h X\n")?;
        Ok(())
    }
}

impl Board {
    pub fn _blank() -> Self {
        Self { pieces: vec![] }
    }

    pub fn find_piece_by_pos(&mut self, x: i8, y: i8) -> Option<usize> {
        let piece: &Piece = self
            .pieces
            .iter()
            .filter(|p| p.pos[0] == x)
            .filter(|p| p.pos[1] == y)
            .nth(0)?;

        self.pieces.iter().position(|x| x == piece)
    }

    pub fn get_tiles_between(a: [i8; 2], b: [i8; 2]) -> Option<Vec<[i8; 2]>> {
        let xs: Vec<i8> = if a[0] > b[0] {
            (b[0]+1..=a[0]).rev().collect()
        } else {
            (a[0]..b[0]).into_iter().collect()
        };

        let ys: Vec<i8> = if a[1] > b[1] {
            (b[1]+1..=a[1]).rev().collect()
        } else {
            (a[1]..b[1]).into_iter().collect()
        };

        if xs.len() == 0 {
            Some(ys.iter().map(|i| [a[0], *i]).skip(1).collect())
        } else if ys.len() == 0 {
            Some(xs.iter().map(|i| [*i, a[1]]).skip(1).collect())
        } else if xs.len() == ys.len() {
            Some(xs.iter().zip(ys).map(|(x, y)| [*x, y]).skip(1).collect())
        } else {
            None
        }
    }

    pub fn get_intervening_pieces(&mut self, between: &Vec<[i8; 2]>) -> Vec<[i8; 2]> {
        between.iter()
            .filter(|t| self.find_piece_by_pos(t[0], t[1]).is_some())
            .map(|t| *t)
            .collect()
    }

    pub fn check_for_collisions(
        &mut self,
        from: [i8; 2],
        to: [i8; 2],
        piece_type: PieceType,
    ) -> bool {
        match piece_type {
            PieceType::Pawn(_) | PieceType::King(_) | PieceType::Knight(_) => false,
            PieceType::Queen(_) | PieceType::Bishop(_) | PieceType::Rook(_) => {
                let between = Self::get_tiles_between(from, to).unwrap();

                let full_squares: Vec<[i8; 2]> = self.get_intervening_pieces(&between);

                (between.len() > 0) && (full_squares.len() > 0)
            }
            PieceType::Empty(_) => false,
        }
    }
}
