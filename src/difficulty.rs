use crate::direction::Direction;

/// Difficulty configuration that controls which directions are allowed.
#[derive(Debug, Clone)]
pub struct Difficulty {
    pub allow_horizontal: bool,
    pub allow_vertical: bool,
    pub allow_diagonal: bool,
    pub allow_reverse: bool,
}

impl Difficulty {
    /// Easy: horizontal and vertical only, no reverse.
    pub fn easy() -> Self {
        Self {
            allow_horizontal: true,
            allow_vertical: true,
            allow_diagonal: false,
            allow_reverse: false,
        }
    }

    /// Medium: horizontal, vertical, and diagonal, no reverse.
    pub fn medium() -> Self {
        Self {
            allow_horizontal: true,
            allow_vertical: true,
            allow_diagonal: true,
            allow_reverse: false,
        }
    }

    /// Hard: all directions including reverse.
    pub fn hard() -> Self {
        Self {
            allow_horizontal: true,
            allow_vertical: true,
            allow_diagonal: true,
            allow_reverse: true,
        }
    }

    /// Returns a list of all allowed directions based on this difficulty.
    pub fn allowed_directions(&self) -> Vec<Direction> {
        let mut directions = Vec::new();

        if self.allow_horizontal {
            directions.push(Direction::Horizontal);
            if self.allow_reverse {
                directions.push(Direction::HorizontalReverse);
            }
        }

        if self.allow_vertical {
            directions.push(Direction::Vertical);
            if self.allow_reverse {
                directions.push(Direction::VerticalReverse);
            }
        }

        if self.allow_diagonal {
            directions.push(Direction::DiagonalDown);
            directions.push(Direction::DiagonalUp);
            if self.allow_reverse {
                directions.push(Direction::DiagonalDownReverse);
                directions.push(Direction::DiagonalUpReverse);
            }
        }

        directions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easy_directions() {
        let easy = Difficulty::easy();
        let dirs = easy.allowed_directions();
        assert_eq!(dirs.len(), 2);
    }

    #[test]
    fn test_hard_directions() {
        let hard = Difficulty::hard();
        let dirs = hard.allowed_directions();
        assert_eq!(dirs.len(), 8);
    }
}
