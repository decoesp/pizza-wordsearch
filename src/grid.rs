use crate::direction::Direction;
use crate::word::Word;

#[derive(Debug, Clone)]
pub struct Grid {
    pub cells: Vec<Vec<Option<char>>>,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub struct PlacementResult {
    pub word: Word,
    pub row: usize,
    pub col: usize,
    pub direction: Direction,
}

impl Grid {
    pub fn new(size: usize) -> Self {
        let cells = vec![vec![None; size]; size];
        Self { cells, size }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<char> {
        self.cells.get(row).and_then(|r| r.get(col).copied().flatten())
    }

    pub fn set(&mut self, row: usize, col: usize, ch: char) {
        if row < self.size && col < self.size {
            self.cells[row][col] = Some(ch);
        }
    }

    fn in_bounds(&self, row: i32, col: i32) -> bool {
        row >= 0 && col >= 0 && (row as usize) < self.size && (col as usize) < self.size
    }

    pub fn can_place(&self, word: &Word, start_row: usize, start_col: usize, direction: Direction) -> bool {
        let (dr, dc) = direction.deltas();
        let chars = word.chars();

        for (i, ch) in chars.iter().enumerate() {
            let row = start_row as i32 + dr * i as i32;
            let col = start_col as i32 + dc * i as i32;

            if !self.in_bounds(row, col) {
                return false;
            }

            if let Some(existing_char) = self.get(row as usize, col as usize) {
                if existing_char != *ch {
                    return false;
                }
            }
        }

        true
    }

    pub fn place_word(
        &mut self,
        word: &Word,
        start_row: usize,
        start_col: usize,
        direction: Direction,
    ) -> PlacementResult {
        let (dr, dc) = direction.deltas();
        let chars = word.chars();

        for (i, ch) in chars.iter().enumerate() {
            let row = (start_row as i32 + dr * i as i32) as usize;
            let col = (start_col as i32 + dc * i as i32) as usize;
            self.set(row, col, *ch);
        }

        PlacementResult {
            word: word.clone(),
            row: start_row,
            col: start_col,
            direction,
        }
    }
}
