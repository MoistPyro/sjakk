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
    }

    #[test]
    fn test_tiles_between() {
        let q_to_q = Board::get_tiles_between([3, 0], [3, 7]).unwrap();

        let expected_result = vec![[3, 1], [3, 2], [3, 3], [3, 4], [3, 5], [3, 6]];

        assert_eq!(q_to_q, expected_result);

        let r_to_r_diagonal = Board::get_tiles_between([0, 0], [7, 7]).unwrap();

        let expected_result = vec![[1, 1], [2, 2], [3, 3], [4, 4], [5, 5], [6, 6]];

        assert_eq!(r_to_r_diagonal, expected_result);
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    pub pieces: Vec<Piece>,
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.pieces == other.pieces
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut pieces = vec![];

        pieces.push(Piece::new([0, 0], 'R', Colour::White));
        pieces.push(Piece::new([7, 0], 'R', Colour::White));
        pieces.push(Piece::new([0, 7], 'R', Colour::Black));
        pieces.push(Piece::new([7, 7], 'R', Colour::Black));

        pieces.push(Piece::new([1, 0], 'N', Colour::White));
        pieces.push(Piece::new([6, 0], 'N', Colour::White));
        pieces.push(Piece::new([1, 7], 'N', Colour::Black));
        pieces.push(Piece::new([6, 7], 'N', Colour::Black));

        pieces.push(Piece::new([2, 0], 'B', Colour::White));
        pieces.push(Piece::new([5, 0], 'B', Colour::White));
        pieces.push(Piece::new([2, 7], 'B', Colour::Black));
        pieces.push(Piece::new([5, 7], 'B', Colour::Black));

        pieces.push(Piece::new([3, 0], 'Q', Colour::White));
        pieces.push(Piece::new([3, 7], 'Q', Colour::Black));

        pieces.push(Piece::new([4, 0], 'K', Colour::White));
        pieces.push(Piece::new([4, 7], 'K', Colour::Black));

        for i in 0..8 {
            pieces.push(Piece::new([i, 1], 'P', Colour::White));
            pieces.push(Piece::new([i, 6], 'P', Colour::Black));
        }

        Self { pieces }
    }
}

impl Board {
    pub fn find_piece_by_pos(&mut self, x: i8, y: i8) -> Option<usize> {
        let piece: &Piece = self
            .pieces
            .iter()
            .filter(|p| p.pos[0] == x)
            .filter(|p| p.pos[1] == y)
            .nth(0)?;

        self.pieces.iter().position(|x| x == piece)
    }

    fn get_tiles_between(a: [i8; 2], b: [i8; 2]) -> Option<Vec<[i8; 2]>> {
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

    pub fn check_for_collisions(
        &mut self,
        from: [i8; 2],
        to: [i8; 2],
        piece_type: PieceType,
    ) -> bool {
        match piece_type {
            PieceType::Pawn | PieceType::King | PieceType::Knight => false,
            PieceType::Queen | PieceType::Bishop | PieceType::Rook => {
                let between = Self::get_tiles_between(from, to).unwrap();

                let full_squares: Vec<[i8; 2]> = between
                    .iter()
                    .filter(|t| self.find_piece_by_pos(t[0], t[1]).is_some())
                    .map(|t| *t)
                    .collect();

                (between.len() > 0) && (full_squares.len() > 0)
            }
        }
    }
}
