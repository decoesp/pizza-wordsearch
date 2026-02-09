use rand::Rng;

/// Portuguese letter frequency distribution for filling empty cells.
const PORTUGUESE_LETTERS: &str = "AEOSRIDMNTCUVLPGQBFHXJZYWK";

/// Fills empty cells in the grid using Portuguese letter frequency.
pub struct Filler {
    weights: Vec<(char, u32)>,
    total_weight: u32,
}

impl Filler {
    /// Creates a new Filler with Portuguese letter frequency weights.
    pub fn new() -> Self {
        let weights: Vec<(char, u32)> = PORTUGUESE_LETTERS
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let weight = (PORTUGUESE_LETTERS.len() - i) as u32;
                (c, weight)
            })
            .collect();
        let total_weight = weights.iter().map(|(_, w)| w).sum();
        Self { weights, total_weight }
    }

    /// Picks a random letter based on frequency weights.
    pub fn pick_letter<R: Rng>(&self, rng: &mut R) -> char {
        let mut roll = rng.gen_range(0..self.total_weight);

        for (letter, weight) in &self.weights {
            if roll < *weight {
                return *letter;
            }
            roll -= weight;
        }

        'A'
    }

    /// Fills all empty cells in the grid with random letters.
    pub fn fill_grid<R: Rng>(&self, grid: &mut crate::grid::Grid, rng: &mut R) {
        for row in 0..grid.size {
            for col in 0..grid.size {
                if grid.cells[row][col].is_none() {
                    grid.cells[row][col] = Some(self.pick_letter(rng));
                }
            }
        }
    }
}

impl Default for Filler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn test_filler_creation() {
        let filler = Filler::new();
        assert!(!filler.weights.is_empty());
        assert!(filler.total_weight > 0);
    }

    #[test]
    fn test_pick_letter_deterministic() {
        let filler = Filler::new();
        let mut rng = StdRng::seed_from_u64(42);
        let letter1 = filler.pick_letter(&mut rng);
        let mut rng2 = StdRng::seed_from_u64(42);
        let letter2 = filler.pick_letter(&mut rng2);
        assert_eq!(letter1, letter2);
    }

    #[test]
    fn test_fill_grid() {
        use crate::grid::Grid;

        let filler = Filler::new();
        let mut grid = Grid::new(5);
        let mut rng = StdRng::seed_from_u64(42);

        assert_eq!(grid.empty_count(), 25);
        filler.fill_grid(&mut grid, &mut rng);
        assert_eq!(grid.empty_count(), 0);
    }
}
