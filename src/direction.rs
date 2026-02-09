use rand::seq::SliceRandom;
use rand::Rng;

/// Represents all possible directions a word can be placed in the grid.
/// Each direction has a delta (dx, dy) that determines movement per character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Horizontal,
    HorizontalReverse,
    Vertical,
    VerticalReverse,
    DiagonalDown,
    DiagonalDownReverse,
    DiagonalUp,
    DiagonalUpReverse,
}

impl Direction {
    /// Returns the row and column deltas for this direction.
    pub fn deltas(&self) -> (i32, i32) {
        match self {
            Direction::Horizontal => (0, 1),
            Direction::HorizontalReverse => (0, -1),
            Direction::Vertical => (1, 0),
            Direction::VerticalReverse => (-1, 0),
            Direction::DiagonalDown => (1, 1),
            Direction::DiagonalDownReverse => (-1, -1),
            Direction::DiagonalUp => (-1, 1),
            Direction::DiagonalUpReverse => (1, -1),
        }
    }

    /// Returns the reversed version of this direction.
    #[allow(dead_code)]
    pub fn reversed(&self) -> Direction {
        match self {
            Direction::Horizontal => Direction::HorizontalReverse,
            Direction::HorizontalReverse => Direction::Horizontal,
            Direction::Vertical => Direction::VerticalReverse,
            Direction::VerticalReverse => Direction::Vertical,
            Direction::DiagonalDown => Direction::DiagonalDownReverse,
            Direction::DiagonalDownReverse => Direction::DiagonalDown,
            Direction::DiagonalUp => Direction::DiagonalUpReverse,
            Direction::DiagonalUpReverse => Direction::DiagonalUp,
        }
    }

    /// Checks if this direction is diagonal.
    #[allow(dead_code)]
    pub fn is_diagonal(&self) -> bool {
        matches!(
            self,
            Direction::DiagonalDown
                | Direction::DiagonalDownReverse
                | Direction::DiagonalUp
                | Direction::DiagonalUpReverse
        )
    }

    /// Picks a random direction from the given list.
    pub fn random_from<R: Rng>(directions: &[Direction], rng: &mut R) -> Option<Direction> {
        directions.choose(rng).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deltas() {
        assert_eq!(Direction::Horizontal.deltas(), (0, 1));
        assert_eq!(Direction::Vertical.deltas(), (1, 0));
        assert_eq!(Direction::DiagonalDown.deltas(), (1, 1));
    }

    #[test]
    fn test_reversed() {
        assert_eq!(Direction::Horizontal.reversed(), Direction::HorizontalReverse);
    }

    #[test]
    fn test_is_diagonal() {
        assert!(!Direction::Horizontal.is_diagonal());
        assert!(Direction::DiagonalDown.is_diagonal());
    }
}
