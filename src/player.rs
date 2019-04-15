use std::rc::Rc;
use crate::board::{Board, CellReference};
use std::fmt::Debug;

pub trait Player: Debug {
    fn get_marker(&self) -> &str;
    fn get_move<'board>(&self, other_player: Rc<dyn Player>, board: &'board Board) -> &'board CellReference;
    fn to_boxed(self) -> Box<dyn Player>;
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.get_marker().eq(other.get_marker())
    }
}
