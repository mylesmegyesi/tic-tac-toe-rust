mod board;
mod rules;
mod player;

#[cfg(test)]
mod mock_player;

use crate::rules::{GameState, analyze_game_state};
use crate::player::Player;
use crate::board::{Board};

fn play_game<'players>(player_one: &'players Box<dyn Player>, player_two: &'players Box<dyn Player>) -> GameState<'players> {
    let mut current_player = player_one;
    let mut opposing_player = player_two;
    let mut board = Board::empty_board();
    loop {
        let state = analyze_game_state(&board);
        match state {
            GameState::Pending => {
                let player_move = current_player.get_move(opposing_player, &board);
                board = board.occupy_cell(current_player, player_move);
                let tmp = current_player;
                current_player = opposing_player;
                opposing_player = tmp;
            },
            game_state => return game_state
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mock_player::MockPlayer;

    #[test]
    fn plays_a_full_game_to_a_tie() {
        let player_one = MockPlayer::new("X", vec![0, 8, 1, 3, 6]).to_boxed();
        let player_two = MockPlayer::new("O", vec![4, 7, 2, 5]).to_boxed();

        let result = play_game(&player_one, &player_two);

        assert_eq!(result, GameState::Draw);
    }
}