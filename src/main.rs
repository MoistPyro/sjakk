mod board;
mod piece;
mod turn;
mod types;

use std::fs::read_to_string;
use std::io::Error;

use types::PieceType;

use crate::board::Board;
use crate::piece::Piece;
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
        let mock_turn_1: Turn = Turn::new(
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
        );

        assert_eq!(r.iter().nth(0).unwrap(), &mock_turn_1);
    }

    #[test]
    fn test_find_first() {
        let temp_move_1 = Move::new_from_notation("e4", 'w').unwrap();
        let temp_move_2 = Move::new_from_notation("Nc3", 'w').unwrap();
        let temp_move_3 = Move::new_from_notation("g6", 'b').unwrap();
        let temp_move_4 = Move::new_from_notation("Bg7", 'b').unwrap();
        let mut mock_board = Board::default();

        assert_eq!(find_moved_piece(&mut mock_board, temp_move_1).unwrap(), 24);
        assert_eq!(find_moved_piece(&mut mock_board, temp_move_2).unwrap(), 4);
        assert_eq!(find_moved_piece(&mut mock_board, temp_move_3).unwrap(), 29);
        mock_board.pieces.remove(29);

        assert_eq!(find_moved_piece(&mut mock_board, temp_move_4).unwrap(), 11);
    }

    #[test]
    fn test_make_a_move() {
        let default_board = Board::default();
        let mut after_move_board = Board::default();

        let i = after_move_board.find_piece_by_pos(4, 1).unwrap();
        after_move_board.pieces.remove(i);

        let i = after_move_board.find_piece_by_pos(6, 6).unwrap();
        after_move_board.pieces.remove(i);

        after_move_board
            .pieces
            .push(Piece::new([4, 3], PieceType::Pawn, Colour::White));
        after_move_board
            .pieces
            .push(Piece::new([6, 5], PieceType::Pawn, Colour::Black));

        assert_eq!(
            after_move_board,
            make_a_move(default_board, &Turn::new_from_notation("e4 g6").unwrap(), 0)
        );
    }
}

fn format_move_description(turn_number: usize, mv: &Move) -> Option<String> {
    let destination: String = format!("{}, {}", mv.to?[0], mv.to?[1]);
    let mut out = format!(
        "{:2}. {:?} moved {} to {}",
        turn_number + 1,
        mv.colour,
        mv.piece,
        destination
    );

    if mv.check.is_check_or_mate() {
        out += &format!("  {:?}", mv.check)
    }

    Some(out)
}

fn find_moved_piece(board: &mut Board, mv: Move) -> Option<usize> {
    let piece_type_equality = |p: &&Piece| p.piece_type == mv.piece;

    let colour_equality = |p: &&Piece| p.colour == mv.colour;

    let legal_moves_equality = |p: &&Piece| match mv.capture {
        Capture::Yes => p.get_capture_tiles(mv.castle).contains(&mv.to.unwrap()),
        Capture::No => p.get_move_tiles(mv.castle).contains(&mv.to.unwrap()),
    };

    let ambiguity_remover = |p: &&Piece| {
        if let Some(x) = mv.from {
            p.pos[0] == x as i8
        } else {
            true
        }
    };

    let blocking_piece_checker = |p: &&Piece| match p.piece_type {
        PieceType::Pawn | PieceType::King | PieceType::Knight => true,
        _ => {
            let mut temp_board: Board = board.clone();
            !temp_board.check_for_collisions(p.pos, mv.to.unwrap(), p.piece_type)
        }
    };

    let all_possible_capturers: Vec<&Piece> = board
        .pieces
        .iter()
        .filter(piece_type_equality)
        .filter(colour_equality)
        .filter(legal_moves_equality)
        .filter(ambiguity_remover)
        .filter(blocking_piece_checker)
        .collect();

    assert!(all_possible_capturers.len() == 1);

    board
        .pieces
        .iter()
        .position(|x| x == all_possible_capturers[0])
}

fn execute_move(mut board: Board, index: usize, to: [i8; 2]) -> Board {
    let mut moved_piece = board.pieces.remove(index);
    moved_piece.pos = to;
    board.pieces.push(moved_piece);

    board
}

fn make_a_move(mut board: Board, turn: &Turn, turn_number: usize) -> Board {
    for mv in *turn {
        println!("{}", format_move_description(turn_number, &mv).unwrap());

        // If this is a castle, find and move the Rook.
        match mv.castle {
            types::Castle::No => (),
            types::Castle::Short(pos) => {
                let castle_index = board.find_piece_by_pos(pos[0], pos[1]).unwrap();
                let castle_to = [5, pos[1]];
                board = execute_move(board, castle_index, castle_to);
            }
            types::Castle::Long(pos) => {
                let castle_index = board.find_piece_by_pos(pos[0], pos[1]).unwrap();
                let castle_to = [3, pos[1]];
                board = execute_move(board, castle_index, castle_to);
            }
        }

        let captured_piece_index: Option<usize> =
            board.find_piece_by_pos(mv.to.unwrap()[0], mv.to.unwrap()[1]);

        if let Some(i) = captured_piece_index {
            board.pieces.remove(i);
        }

        let moved_piece_index: usize = find_moved_piece(&mut board, mv).unwrap();
        board = execute_move(board, moved_piece_index, mv.to.unwrap());
    }

    board
}

fn read_game(path: &str) -> Result<Vec<Turn>, Error> {
    Ok(read_to_string(path)?
        .lines()
        .map(|notation| Turn::new_from_notation(notation))
        .collect::<Result<Vec<Turn>, Error>>()?)
}

fn main() -> Result<(), Error> {
    let list_of_turns: Vec<Turn> = read_game(GAME_FILE)?;

    let initial_board: Board = Board::default();

    let final_board: Board = list_of_turns
        .iter()
        .enumerate()
        .fold(initial_board, |b, (i, t)| make_a_move(b, t, i));

    println!("{:?}", final_board);
    Ok(())
}

/*
programmet sjekker nå om det er brikker i veien, men kræsjer fortsatt.
*/
