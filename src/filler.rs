use rand::Rng;

const PORTUGUESE_LETTERS: &str = "AEOSRIDMNTCUVLPGQBFHXJZYWK";

pub struct Filler {
    weights: Vec<(char, u32)>,
    total_weight: u32,
}

impl Filler {
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
