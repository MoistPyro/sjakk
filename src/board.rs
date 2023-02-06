use crate::{piece::Piece, types::Colour};

#[derive(Debug)]
pub struct Board {
    pub pieces: Vec<Piece>,
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
        pieces.push(Piece::new([1, 7], 'N', Colour::Black));

        pieces.push(Piece::new([2, 0], 'B', Colour::White));
        pieces.push(Piece::new([5, 0], 'B', Colour::White));
        pieces.push(Piece::new([2, 7], 'B', Colour::Black));
        pieces.push(Piece::new([5, 7], 'B', Colour::Black));

        pieces.push(Piece::new([3, 0], 'Q', Colour::White));
        pieces.push(Piece::new([3, 7], 'Q', Colour::Black));

        pieces.push(Piece::new([4, 0], 'K', Colour::White));
        pieces.push(Piece::new([4, 7], 'K', Colour::Black));

        for i in 0..7 {
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
}
