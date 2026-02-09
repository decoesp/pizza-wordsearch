use rand::Rng;

use crate::difficulty::Difficulty;
use crate::direction::Direction;
use crate::filler::Filler;
use crate::grid::{Grid, PlacementResult};
use crate::word::{sort_by_length_desc, Word};

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

#[derive(Debug)]
pub struct GenerationResult {
    pub grid: Grid,
    pub placed_words: Vec<PlacementResult>,
    pub discarded_words: Vec<Word>,
}

pub struct Generator {
    config: GeneratorConfig,
}

impl Generator {
    pub fn new(config: GeneratorConfig) -> Self {
        Self { config }
    }

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

            let direction = Direction::random_from(allowed_directions, rng)?;

            if grid.can_place(word, start_row, start_col, direction) {
                return Some(grid.place_word(word, start_row, start_col, direction));
            }
        }

        None
    }
}
