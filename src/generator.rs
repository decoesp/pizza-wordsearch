use rand::Rng;

use crate::difficulty::Difficulty;
use crate::direction::Direction;
use crate::filler::Filler;
use crate::grid::{Grid, PlacementResult};
use crate::word::{sort_by_length_desc, Word};

/// Configuration for the word search generator.
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    pub grid_size: usize,
    pub difficulty: Difficulty,
    pub max_attempts_per_word: usize,
}

impl GeneratorConfig {
    pub fn new(grid_size: usize, difficulty: Difficulty) -> Self {
        Self {
            grid_size,
            difficulty,
            max_attempts_per_word: 100,
        }
    }

    pub fn with_max_attempts(mut self, attempts: usize) -> Self {
        self.max_attempts_per_word = attempts;
        self
    }
}

/// Result of the generation process.
#[derive(Debug)]
pub struct GenerationResult {
    pub grid: Grid,
    pub placed_words: Vec<PlacementResult>,
    pub discarded_words: Vec<Word>,
}

/// The main word search generator.
pub struct Generator {
    config: GeneratorConfig,
}

impl Generator {
    pub fn new(config: GeneratorConfig) -> Self {
        Self { config }
    }

    /// Generates a word search puzzle from the given words.
    pub fn generate<R: Rng>(&self, words: &[&str], rng: &mut R) -> GenerationResult {
        let mut grid = Grid::new(self.config.grid_size);
        let allowed_directions = self.config.difficulty.allowed_directions();

        let mut word_list: Vec<Word> = words
            .iter()
            .map(|w| Word::new(w))
            .filter(|w| !w.is_empty() && w.len() <= self.config.grid_size)
            .collect();
        sort_by_length_desc(&mut word_list);

        let mut placed_words = Vec::new();
        let mut discarded_words = Vec::new();

        for word in word_list {
            match self.try_place_word(&mut grid, &word, &allowed_directions, rng) {
                Some(result) => placed_words.push(result),
                None => discarded_words.push(word),
            }
        }

        let filler = Filler::new();
        filler.fill_grid(&mut grid, rng);

        GenerationResult {
            grid,
            placed_words,
            discarded_words,
        }
    }

    /// Attempts to place a word in the grid.
    fn try_place_word<R: Rng>(
        &self,
        grid: &mut Grid,
        word: &Word,
        allowed_directions: &[Direction],
        rng: &mut R,
    ) -> Option<PlacementResult> {
        if allowed_directions.is_empty() {
            return None;
        }

        for _ in 0..self.config.max_attempts_per_word {
            let start_row = rng.gen_range(0..self.config.grid_size);
            let start_col = rng.gen_range(0..self.config.grid_size);

            let direction = match Direction::random_from(allowed_directions, rng) {
                Some(d) => d,
                None => continue,
            };

            if grid.can_place(word, start_row, start_col, direction) {
                let result = grid.place_word(word, start_row, start_col, direction);
                return Some(result);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_generator_basic() {
        let config = GeneratorConfig::new(15, Difficulty::easy());
        let generator = Generator::new(config);
        let mut rng = StdRng::seed_from_u64(42);

        let words = vec!["RUST", "CODE", "PIZZA"];
        let result = generator.generate(&words, &mut rng);

        assert_eq!(result.grid.size, 15);
        assert_eq!(result.grid.empty_count(), 0);
        assert!(!result.placed_words.is_empty());
    }

    #[test]
    fn test_generator_deterministic() {
        let config = GeneratorConfig::new(10, Difficulty::medium());
        let generator = Generator::new(config.clone());

        let words = vec!["HELLO", "WORLD"];

        let mut rng1 = StdRng::seed_from_u64(12345);
        let result1 = generator.generate(&words, &mut rng1);

        let generator2 = Generator::new(config);
        let mut rng2 = StdRng::seed_from_u64(12345);
        let result2 = generator2.generate(&words, &mut rng2);

        assert_eq!(result1.grid.display(false), result2.grid.display(false));
    }
}
