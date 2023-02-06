mod board;
mod piece;
mod turn;
mod types;

use std::fs::read_to_string;
use std::io::Error;

use board::Board;
use piece::Piece;

use crate::turn::{Move, Turn};
use crate::types::Capture;

const GAME_FILE: &str = "game.txt";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    #[test]
    fn test_read_game() {
        let r = read_game("test_game.txt").unwrap();
        let mock_turn_1: Turn = Turn {
            value: [
                Move {
                    piece: PieceType::Pawn,
                    castle: Castle::No,
                    colour: Colour::White,
                    capture: Capture::No,
                    promotion: Promotion::No,
                    check: Check::No,
                    from: None,
                    to: Some([4, 3]),
                },
                Move {
                    piece: PieceType::Pawn,
                    castle: Castle::No,
                    colour: Colour::Black,
                    capture: Capture::No,
                    promotion: Promotion::No,
                    check: Check::No,
                    from: None,
                    to: Some([3, 5]),
                },
            ],
        };

        assert_eq!(r.iter().nth(0).unwrap(), &mock_turn_1);
    }
}

fn debug(i: usize, mv: Move) {
    let destination: String = format!("{}, {}", mv.to.unwrap()[0], mv.to.unwrap()[1]);
    let mut out = format!(
        "{:2}. {:?} moved {} to {}",
        i + 1,
        mv.colour,
        mv.piece,
        destination
    );

    if mv.check.into() {
        out += &format!("  {:?}", mv.check)
    }
    println!("{}", out);
}

fn make_a_move(mut board: Board, turn: &Turn, i: usize) -> Board {
    for mv in *turn {
        debug(i, mv);

        let first = board
            .pieces
            .iter()
            .filter(|p| p.piece_type == mv.piece)
            .filter(|p| match mv.capture {
                Capture::Yes => p.get_capture_tiles().contains(&mv.to.unwrap()),
                Capture::No => p.get_move_tiles().contains(&mv.to.unwrap()),
            })
            .filter(|p| match mv.from {
                Some(x) => p.pos[0] == x as i8,
                None => true,
            })
            .nth(0)
            .unwrap();

        let first_index: usize = board.pieces.iter().position(|x| x == first).unwrap();
        let second_index: Option<usize> =
            board.find_piece_by_pos(mv.to.unwrap()[0], mv.to.unwrap()[1]);

        if let Some(i) = second_index {
            board.pieces.remove(i);
        }

        let mut moved_piece: Piece = board.pieces.remove(first_index);
        moved_piece.pos = mv.to.unwrap();
        board.pieces.push(moved_piece);
    }

    board
}

fn read_game(path: &str) -> Result<Vec<Turn>, Error> {
    Ok(read_to_string(path)?
        .lines()
        .map(|notation| Turn::from(notation))
        .collect())
}

fn main() -> Result<(), Error> {
    let list_of_turns: Vec<Turn> = read_game(GAME_FILE)?;

    let initial_board = Board::default();
    let final_board = list_of_turns
        .iter()
        .enumerate()
        .fold(initial_board, |b, (i, t)| make_a_move(b, t, i));

    println!("{:?}", final_board);
    Ok(())
}
