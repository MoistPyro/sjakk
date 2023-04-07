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
        let expected_result = vec![[4, 0], [5, 0], [6, 0]];

        assert_eq!(negative_test, expected_result);
    }

    #[test]
    fn test_intervention() {
        let mut initial_board = Board::default();
        let r_to_r = Board::get_tiles_between([0,0], [7,0]).unwrap();
        let interventions = initial_board.get_intervening_pieces(&r_to_r);

        assert_eq!(interventions.len(), 6);

        let mut empty_board = Board::blank();

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
        write!(f, "\nX a b c d e f g h X\n")?;

        let mut board_1d: [PieceType; 64] = [PieceType::Empty; 64];

        for i in 0..board_1d.len() {
            board_1d[i] = match self
                .pieces
                .iter()
                .filter(|x| x.pos == [(i % 8) as i8, (i as f64 / 8.0).floor() as i8])
                .nth(0)
            {
                Some(p) => p.piece_type,
                None => PieceType::Empty,
            };
        }

        write!(
            f,
            "1 {} {} {} {} {} {} {} {} 1\n",
            board_1d[0],
            board_1d[1],
            board_1d[2],
            board_1d[3],
            board_1d[4],
            board_1d[5],
            board_1d[6],
            board_1d[7]
        )?;
        write!(
            f,
            "2 {} {} {} {} {} {} {} {} 2\n",
            board_1d[8],
            board_1d[9],
            board_1d[10],
            board_1d[11],
            board_1d[12],
            board_1d[13],
            board_1d[14],
            board_1d[15]
        )?;
        write!(
            f,
            "3 {} {} {} {} {} {} {} {} 3\n",
            board_1d[16],
            board_1d[17],
            board_1d[18],
            board_1d[19],
            board_1d[20],
            board_1d[21],
            board_1d[22],
            board_1d[23]
        )?;
        write!(
            f,
            "4 {} {} {} {} {} {} {} {} 4\n",
            board_1d[24],
            board_1d[25],
            board_1d[26],
            board_1d[27],
            board_1d[28],
            board_1d[29],
            board_1d[30],
            board_1d[31]
        )?;
        write!(
            f,
            "5 {} {} {} {} {} {} {} {} 5\n",
            board_1d[32],
            board_1d[33],
            board_1d[34],
            board_1d[35],
            board_1d[36],
            board_1d[37],
            board_1d[38],
            board_1d[39]
        )?;
        write!(
            f,
            "6 {} {} {} {} {} {} {} {} 6\n",
            board_1d[40],
            board_1d[41],
            board_1d[42],
            board_1d[43],
            board_1d[44],
            board_1d[45],
            board_1d[46],
            board_1d[47]
        )?;
        write!(
            f,
            "7 {} {} {} {} {} {} {} {} 7\n",
            board_1d[48],
            board_1d[49],
            board_1d[50],
            board_1d[51],
            board_1d[52],
            board_1d[53],
            board_1d[54],
            board_1d[55]
        )?;
        write!(
            f,
            "8 {} {} {} {} {} {} {} {} 8\n",
            board_1d[56],
            board_1d[57],
            board_1d[58],
            board_1d[59],
            board_1d[60],
            board_1d[61],
            board_1d[62],
            board_1d[63]
        )?;
        write!(f, "X a b c d e f g h X\n")?;
        Ok(())
    }
}

impl Board {
    pub fn blank() -> Self {
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

    fn get_tiles_between(mut a: [i8; 2], mut b: [i8; 2]) -> Option<Vec<[i8; 2]>> {
        if a[0] > b[0] || a[1] > b[1] {
            let temp = a;
            a = b;
            b = temp;
        }

        let xs = a[0]..b[0];
        let ys = a[1]..b[1];

        if xs.len() == 0 {
            Some((a[1]..b[1]).map(|i| [a[0], i]).skip(1).collect())
        } else if ys.len() == 0 {
            Some((a[0]..b[0]).map(|i| [i, a[1]]).skip(1).collect())
        } else if xs.len() == ys.len() {
            Some(xs.zip(ys).map(|(x, y)| [x, y]).skip(1).collect())
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
            PieceType::Empty => false,
        }
    }
}
