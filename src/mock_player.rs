use std::cell::RefCell;
use crate::player::Player;
use crate::board::{Board, CellReference};

#[derive(Debug)]
pub struct MockPlayer {
    marker: String,
    moves: RefCell<Vec<usize>>,
}

impl MockPlayer {
    pub fn boxed(marker: &str) -> Box<dyn Player> {
        MockPlayer::with_marker(marker).to_boxed()
    }

    pub fn new(marker: &str, moves: Vec<usize>) -> MockPlayer {
        MockPlayer { marker: marker.into(), moves: RefCell::new(moves) }
    }

    pub fn with_marker(marker: &str) -> MockPlayer {
        MockPlayer { marker: marker.into(), moves: RefCell::new(vec![]) }
    }
}

impl Player for MockPlayer {
    fn get_marker(&self) -> &str {
        &self.marker
    }

    fn get_move<'a>(&'a self, other_player: &'a Box<dyn Player>, board: &'a Board) -> &'a CellReference {
        let mut cells = board.cells().enumerate();
        let move_index = self.moves.borrow_mut().pop().expect("mock player has run out of moves");
        let (_, cell) = cells
            .find(|(index, cell)| *index == move_index)
            .expect(&format!("mock player move {} not found in board", move_index));

        cell.get_reference()
    }

    fn to_boxed(self) -> Box<dyn Player> {
        Box::new(self)
    }
}
