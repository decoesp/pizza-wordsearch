use crate::direction::Direction;
use crate::word::Word;

/// Represents the word search grid.
#[derive(Debug, Clone)]
pub struct Grid {
    pub cells: Vec<Vec<Option<char>>>,
    pub size: usize,
}

/// Result of a word placement attempt.
#[derive(Debug, Clone)]
pub struct PlacementResult {
    pub word: Word,
    pub row: usize,
    pub col: usize,
    pub direction: Direction,
}

impl Grid {
    /// Creates a new empty grid of the specified size.
    pub fn new(size: usize) -> Self {
        let cells = vec![vec![None; size]; size];
        Self { cells, size }
    }

    /// Gets the character at a specific position, if any.
    pub fn get(&self, row: usize, col: usize) -> Option<char> {
        self.cells.get(row).and_then(|r| r.get(col).copied().flatten())
    }

    /// Sets a character at a specific position.
    pub fn set(&mut self, row: usize, col: usize, ch: char) {
        if row < self.size && col < self.size {
            self.cells[row][col] = Some(ch);
        }
    }

    /// Checks if a position is within grid bounds.
    pub fn in_bounds(&self, row: i32, col: i32) -> bool {
        row >= 0 && col >= 0 && (row as usize) < self.size && (col as usize) < self.size
    }

    /// Validates if a word can be placed at the given position and direction.
    pub fn can_place(&self, word: &Word, start_row: usize, start_col: usize, direction: Direction) -> bool {
        let (dr, dc) = direction.deltas();
        let chars = word.chars();

        for (i, ch) in chars.iter().enumerate() {
            let row = start_row as i32 + dr * i as i32;
            let col = start_col as i32 + dc * i as i32;

            if !self.in_bounds(row, col) {
                return false;
            }

            let existing = self.get(row as usize, col as usize);
            if let Some(existing_char) = existing {
                if existing_char != *ch {
                    return false;
                }
            }
        }

        true
    }

    /// Places a word at the given position and direction.
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

    /// Counts the number of empty cells.
    #[allow(dead_code)]
    pub fn empty_count(&self) -> usize {
        self.cells.iter().flatten().filter(|c| c.is_none()).count()
    }

    /// Displays the grid as a formatted string.
    #[allow(dead_code)]
    pub fn display(&self, show_empty_as_dot: bool) -> String {
        let mut output = String::new();
        for row in &self.cells {
            for cell in row {
                let ch = match cell {
                    Some(c) => *c,
                    None => if show_empty_as_dot { '.' } else { ' ' },
                };
                output.push(ch);
                output.push(' ');
            }
            output.push('\n');
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_grid() {
        let grid = Grid::new(10);
        assert_eq!(grid.size, 10);
        assert_eq!(grid.empty_count(), 100);
    }

    #[test]
    fn test_can_place_horizontal() {
        let grid = Grid::new(10);
        let word = Word::new("RUST");
        assert!(grid.can_place(&word, 0, 0, Direction::Horizontal));
        assert!(!grid.can_place(&word, 0, 7, Direction::Horizontal));
    }

    #[test]
    fn test_place_word() {
        let mut grid = Grid::new(10);
        let word = Word::new("RUST");
        grid.place_word(&word, 0, 0, Direction::Horizontal);

        assert_eq!(grid.get(0, 0), Some('R'));
        assert_eq!(grid.get(0, 3), Some('T'));
    }
}
