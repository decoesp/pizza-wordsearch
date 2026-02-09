use crate::direction::Direction;

#[derive(Debug, Clone)]
pub struct Difficulty {
    pub allow_horizontal: bool,
    pub allow_vertical: bool,
    pub allow_diagonal: bool,
    pub allow_reverse: bool,
}

impl Difficulty {
    pub fn easy() -> Self {
        Self {
            allow_horizontal: true,
            allow_vertical: true,
            allow_diagonal: false,
            allow_reverse: false,
        }
    }

    pub fn medium() -> Self {
        Self {
            allow_horizontal: true,
            allow_vertical: true,
            allow_diagonal: true,
            allow_reverse: false,
        }
    }

    pub fn hard() -> Self {
        Self {
            allow_horizontal: true,
            allow_vertical: true,
            allow_diagonal: true,
            allow_reverse: true,
        }
    }

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
