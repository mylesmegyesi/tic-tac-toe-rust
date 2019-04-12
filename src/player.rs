use crate::board::{Board, CellReference};
use std::fmt::Debug;

pub trait Player: Debug {
    fn get_marker(&self) -> &str;
    fn get_move<'a>(&'a self, other_player: &'a Box<dyn Player>, board: &'a Board) -> &'a CellReference;
    fn to_boxed(self) -> Box<dyn Player>;
}

impl PartialEq for Box<dyn Player> {
    fn eq(&self, other: &Self) -> bool {
        self.get_marker().eq(other.get_marker())
    }
}
