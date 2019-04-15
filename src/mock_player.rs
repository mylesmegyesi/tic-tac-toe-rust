use std::cell::RefCell;
use crate::player::Player;
use crate::board::{Board, CellReference};
use std::rc::Rc;

#[derive(Debug)]
pub struct MockPlayer {
    marker: String,
    moves: RefCell<Vec<usize>>,
}

impl MockPlayer {
    pub fn new(marker: &str, moves: Vec<usize>) -> MockPlayer {
        MockPlayer { marker: marker.into(), moves: RefCell::new(moves) }
    }

    pub fn with_marker(marker: &str) -> Rc<MockPlayer> {
        Rc::new(MockPlayer::new(marker, vec![]))
    }
}

impl Player for MockPlayer {
    fn get_marker(&self) -> &str {
        &self.marker
    }

    fn get_move<'board>(&self, _: Rc<dyn Player>, board: &'board Board) -> &'board CellReference {
        let mut cells = board.cells().enumerate();
        let move_index = self.moves.borrow_mut().pop().expect("mock player has run out of moves");
        let (_, cell) = cells
            .find(|(index, _)| *index == move_index)
            .expect(&format!("mock player move {} not found in board", move_index));

        cell.get_reference()
    }

    fn to_boxed(self) -> Box<dyn Player> {
        Box::new(self)
    }
}
