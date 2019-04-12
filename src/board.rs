use crate::player::Player;

pub struct Board<'players> {
    size: usize,
    cells: Vec<Cell<'players>>,
}

#[derive(Clone)]
pub struct Cell<'players> {
    reference: CellReference,
    occupant: Option<&'players Box<dyn Player>>,
}

impl<'board, 'players> Cell<'players> {
    pub fn get_marker(&'board self) -> Option<&'players str> {
        self.occupant.map(|o| o.get_marker())
    }

    pub fn get_reference(&'board self) -> &'board CellReference {
        &self.reference
    }

    pub fn get_occupant(&'board self) -> Option<&'players Box<dyn Player>> {
        self.occupant
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CellReference(usize);

impl<'board, 'players> Board<'players> {
    pub fn empty_board() -> Board<'players> {
        let mut cells: Vec<Cell> = Vec::with_capacity(9);
        for i in 0..9 {
            cells.push(Cell {
                reference: CellReference(i),
                occupant: None,
            });
        }
        Board { cells, size: 3 }
    }

    pub fn empty_cells(&'board self) -> Vec<&'board CellReference> {
        self.cells.iter()
            .filter(|c| c.occupant.is_none())
            .map(|c| &c.reference)
            .collect()
    }

    pub fn cells(&'board self) -> impl Iterator<Item=&'board Cell<'players>> {
        self.cells.iter()
    }

    pub fn segments(&'board self) -> Vec<Vec<&'board Cell<'players>>> {
        let mut segments = Vec::with_capacity(self.size * 2 + 2);
        let mut first_diagonal = Vec::with_capacity(self.size);
        let mut second_diagonal = Vec::with_capacity(self.size);
        for row_index in 0..self.size {
            let mut row = Vec::with_capacity(self.size);
            let mut column = Vec::with_capacity(self.size);
            for column_index in 0..self.size {
                row.insert(column_index, &self.cells[(row_index * self.size) + column_index]);
                column.insert(column_index, &self.cells[row_index + (column_index * self.size)]);
            }
            segments.push(row);
            segments.push(column);

            first_diagonal.push(&self.cells[(row_index * self.size) + row_index]);
            second_diagonal.push(&self.cells[(row_index * self.size) + (self.size - row_index - 1)]);
        }
        segments.push(first_diagonal);
        segments.push(second_diagonal);
        segments
    }

    pub fn occupy_cell(&'board self, occupant: &'players Box<dyn Player>, reference: &'board CellReference) -> Board<'board> {
        let mut cells = self.cells.clone();
        let CellReference(index) = *reference;
        std::mem::replace(&mut cells[index], Cell { occupant: Some(occupant), reference: CellReference(index) });

        Board {
            cells,
            size: self.size
        }
    }

    #[cfg(test)]
    pub fn with_board(player_one: &'players Box<dyn Player>, player_two: &'players Box<dyn Player>, board_string: &str) -> Board<'players> {
        let mut cells = Vec::with_capacity(9);
        for (index, marker) in board_string.trim().split_whitespace().enumerate() {
            cells.push(
                if player_one.get_marker() == marker {
                    Cell { reference: CellReference(index), occupant: Some(player_one) }
                } else if player_two.get_marker() == marker {
                    Cell { reference: CellReference(index), occupant: Some(player_two) }
                } else {
                    Cell { reference: CellReference(index), occupant: None }
                }
            )
        }
        Board { cells, size: 3 }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock_player::MockPlayer;

    pub fn format_occupant_marker<'board, 'players>(cell: &'board Cell<'players>) -> &'players str {
        cell.get_marker().unwrap_or("-")
    }

    pub fn format_board(board: &Board) -> String {
        let markers: Vec<&str> = board.cells()
            .map(|c| format_occupant_marker(c))
            .collect();
        markers.join(" ")
    }

    fn empty_cell_indices(board: &Board) -> Vec<usize> {
        board.cells().enumerate()
            .filter(|(index, c)| c.occupant.is_none())
            .map(|(index, _)| index)
            .collect()
    }

    #[test]
    fn returns_empty_cells() {
        let player_one = MockPlayer::boxed("X");
        let player_two = MockPlayer::boxed("O");
        let board = Board::with_board(&player_one, &player_two, "\
            X - X \
            O O - \
            - - - \
        ");

        assert_eq!(board.empty_cells(), vec![
            &CellReference(1),
            &CellReference(5),
            &CellReference(6),
            &CellReference(7),
            &CellReference(8),
        ]);
    }

    #[test]
    fn returns_the_occupants_for_all_winnable_segments() {
        let player_one = MockPlayer::boxed("X");
        let player_two = MockPlayer::boxed("O");
        let board = Board::with_board(&player_one, &player_two, "\
            X O X \
            X - O \
            X O O \
        ");

        let markers: Vec<Vec<&str>> = board.segments().iter()
            .map(|s| {
                s.iter()
                    .map(|c| format_occupant_marker(c))
                    .collect()

            })
            .collect();
        assert_eq!(markers, vec![
            vec!["X", "O", "X"],
            vec!["X", "X", "X"],
            vec!["X", "-", "O"],
            vec!["O", "-", "O"],
            vec!["X", "O", "O"],
            vec!["X", "O", "O"],
            vec!["X", "-", "O"],
            vec!["X", "-", "X"],
        ]);
    }

    #[test]
    pub fn makes_a_move() {
        let player_one = MockPlayer::boxed("X");
        let player_two = MockPlayer::boxed("O");
        let board = Board::with_board(&player_one, &player_two, "\
            X O X \
            X - O \
            X O O \
        ");
        let empty_cell = *board.empty_cells().first().unwrap();
        let new_board = board.occupy_cell(&player_one, empty_cell);

        assert_eq!(format_board(&new_board), "\
            X O X \
            X X O \
            X O O\
        ");
        // does not modify existing board
        assert_eq!(format_board(&board), "\
            X O X \
            X - O \
            X O O\
        ");
    }
}
