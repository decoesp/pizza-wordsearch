use rand::seq::SliceRandom;
use rand::Rng;

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

    pub fn random_from<R: Rng>(directions: &[Direction], rng: &mut R) -> Option<Direction> {
        directions.choose(rng).copied()
    }
}
