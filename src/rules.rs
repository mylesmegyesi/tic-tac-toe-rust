use std::rc::Rc;
use crate::board::{Board, Cell};
use crate::player::Player;

#[derive(Debug)]
pub enum GameState {
    Pending,
    Draw,
    Win(Rc<Player>),
}

impl PartialEq for GameState {

    // OMG there has to be an easier way
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (GameState::Pending, GameState::Pending) => true,
            (GameState::Draw, GameState::Draw) => true,
            (GameState::Win(player), GameState::Win(other_player)) => player == other_player,
            _ => false
        }
    }
}

pub fn analyze_game_state(board: &Board) -> GameState {
    for segment in board.segments() {
        match get_segment_winner(&segment) {
            Some(winner) => return GameState::Win(winner),
            _ => continue
        }
    }

    if board.empty_cells().is_empty() {
        return GameState::Draw;
    }

    GameState::Pending
}

fn get_segment_winner(segment: &Vec<&Cell>) -> Option<Rc<Player>> {
    segment.first().and_then(|cell| {
        cell.get_occupant().and_then(|first_cell_occupant| {
            let all_in_segment_match = segment[1..].iter().all(|c| {
                c.get_marker().map_or(false, |marker| first_cell_occupant.get_marker() == marker)
            });
            if all_in_segment_match {
                Some(first_cell_occupant)
            } else {
                None
            }
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock_player::MockPlayer;

    #[test]
    fn empty_board_is_pending() {
        let board = Board::empty_board();

        let result = analyze_game_state(&board);

        assert_eq!(result, GameState::Pending);
    }

    #[test]
    fn detects_player_one_win_in_first_row() {
        let player_one = MockPlayer::with_marker("X");
        let player_two = MockPlayer::with_marker("O");
        let board = Board::with_board(player_one.clone(), player_two.clone(), "\
            X X X \
            O O - \
            - - - \
        ");

        let result = analyze_game_state(&board);

        assert_eq!(result, GameState::Win(player_one));
    }

    #[test]
    fn detects_player_one_win_in_second_row() {
        let player_one = MockPlayer::with_marker("X");
        let player_two = MockPlayer::with_marker("O");
        let board = Board::with_board(player_one.clone(), player_two.clone(), "\
            O O - \
            X X X \
            - - - \
        ");

        let result = analyze_game_state(&board);

        assert_eq!(result, GameState::Win(player_one));
    }

    #[test]
    fn detects_player_one_win_in_third_row() {
        let player_one = MockPlayer::with_marker("X");
        let player_two = MockPlayer::with_marker("O");
        let board = Board::with_board(player_one.clone(), player_two.clone(), "\
            O O - \
            - - - \
            X X X \
        ");

        let result = analyze_game_state(&board);

        assert_eq!(result, GameState::Win(player_one));
    }

    #[test]
    fn detects_player_two_win_in_first_row() {
        let player_one = MockPlayer::with_marker("X");
        let player_two = MockPlayer::with_marker("O");
        let board = Board::with_board(player_one.clone(), player_two.clone(), "\
            O O O \
            - - - \
            - - - \
        ");

        let result = analyze_game_state(&board);

        assert_eq!(result, GameState::Win(player_two));
    }

    #[test]
    fn detects_player_two_win_in_second_row() {
        let player_one = MockPlayer::with_marker("X");
        let player_two = MockPlayer::with_marker("O");
        let board = Board::with_board(player_one.clone(), player_two.clone(), "\
            X X - \
            O O O \
            - - - \
        ");

        let result = analyze_game_state(&board);

        assert_eq!(result, GameState::Win(player_two));
    }

    #[test]
    fn detects_player_two_win_in_third_row() {
        let player_one = MockPlayer::with_marker("X");
        let player_two = MockPlayer::with_marker("O");
        let board = Board::with_board(player_one.clone(), player_two.clone(), "\
            - - - \
            X X - \
            O O O \
        ");

        let result = analyze_game_state(&board);

        assert_eq!(result, GameState::Win(player_two));
    }

    #[test]
    fn detects_player_one_win_in_first_column() {
        let player_one = MockPlayer::with_marker("X");
        let player_two = MockPlayer::with_marker("O");
        let board = Board::with_board(player_one.clone(), player_two.clone(), "\
            X O - \
            X O - \
            X - - \
        ");

        let result = analyze_game_state(&board);

        assert_eq!(result, GameState::Win(player_one));
    }

    #[test]
    fn detects_player_one_win_in_second_column() {
        let player_one = MockPlayer::with_marker("X");
        let player_two = MockPlayer::with_marker("O");
        let board = Board::with_board(player_one.clone(), player_two.clone(), "\
            O X - \
            O X - \
            - X - \
        ");

        let result = analyze_game_state(&board);

        assert_eq!(result, GameState::Win(player_one));
    }

    #[test]
    fn detects_player_one_win_in_third_column() {
        let player_one = MockPlayer::with_marker("X");
        let player_two = MockPlayer::with_marker("O");
        let board = Board::with_board(player_one.clone(), player_two.clone(), "\
            O - X \
            O - X \
            - - X \
        ");

        let result = analyze_game_state(&board);

        assert_eq!(result, GameState::Win(player_one));
    }

    #[test]
    fn detects_player_two_win_in_first_column() {
        let player_one = MockPlayer::with_marker("X");
        let player_two = MockPlayer::with_marker("O");
        let board = Board::with_board(player_one.clone(), player_two.clone(), "\
            O X - \
            O X - \
            O - - \
        ");

        let result = analyze_game_state(&board);

        assert_eq!(result, GameState::Win(player_two));
    }

    #[test]
    fn detects_player_two_win_in_second_column() {
        let player_one = MockPlayer::with_marker("X");
        let player_two = MockPlayer::with_marker("O");
        let board = Board::with_board(player_one.clone(), player_two.clone(), "\
            X O - \
            X O - \
            - O - \
        ");

        let result = analyze_game_state(&board);

        assert_eq!(result, GameState::Win(player_two));
    }

    #[test]
    fn detects_player_two_win_in_third_column() {
        let player_one = MockPlayer::with_marker("X");
        let player_two = MockPlayer::with_marker("O");
        let board = Board::with_board(player_one.clone(), player_two.clone(), "\
            - X O \
            - X O \
            - - O \
        ");

        let result = analyze_game_state(&board);

        assert_eq!(result, GameState::Win(player_two));
    }

    #[test]
    fn detects_player_one_win_in_first_diagonal() {
        let player_one = MockPlayer::with_marker("X");
        let player_two = MockPlayer::with_marker("O");
        let board = Board::with_board(player_one.clone(), player_two.clone(), "\
            X - - \
            O X O \
            - - X \
        ");

        let result = analyze_game_state(&board);

        assert_eq!(result, GameState::Win(player_one));
    }

    #[test]
    fn detects_player_one_win_in_second_diagonal() {
        let player_one = MockPlayer::with_marker("X");
        let player_two = MockPlayer::with_marker("O");
        let board = Board::with_board(player_one.clone(), player_two.clone(), "\
            - - X \
            O X O \
            X - - \
        ");

        let result = analyze_game_state(&board);

        assert_eq!(result, GameState::Win(player_one));
    }

    #[test]
    fn detects_player_two_win_in_first_diagonal() {
        let player_one = MockPlayer::with_marker("X");
        let player_two = MockPlayer::with_marker("O");
        let board = Board::with_board(player_one.clone(), player_two.clone(), "\
            O - - \
            X O X \
            - - O \
        ");

        let result = analyze_game_state(&board);

        assert_eq!(result, GameState::Win(player_two));
    }

    #[test]
    fn detects_player_two_win_in_second_diagonal() {
        let player_one = MockPlayer::with_marker("X");
        let player_two = MockPlayer::with_marker("O");
        let board = Board::with_board(player_one.clone(), player_two.clone(), "\
            - - O \
            X O X \
            O - - \
        ");

        let result = analyze_game_state(&board);

        assert_eq!(result, GameState::Win(player_two));
    }

    #[test]
    fn detects_draw() {
        let player_one = MockPlayer::with_marker("X");
        let player_two = MockPlayer::with_marker("O");
        let board = Board::with_board(player_one.clone(), player_two.clone(), "\
            X X O \
            O O X \
            X O X \
        ");

        let result = analyze_game_state(&board);

        assert_eq!(result, GameState::Draw);
    }

    #[test]
    fn detects_pending_when_not_full() {
        let player_one = MockPlayer::with_marker("X");
        let player_two = MockPlayer::with_marker("O");
        let board = Board::with_board(player_one.clone(), player_two.clone(), "\
            X X O \
            O O - \
            X O X \
        ");

        let result = analyze_game_state(&board);

        assert_eq!(result, GameState::Pending);
    }
}
