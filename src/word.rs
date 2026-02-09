use unicode_normalization::UnicodeNormalization;

/// Represents a normalized word ready for placement in the grid.
#[derive(Debug, Clone)]
pub struct Word {
    pub original: String,
    pub normalized: String,
}

impl Word {
    /// Creates a new Word from a string, normalizing it for grid placement.
    pub fn new(input: &str) -> Self {
        let original = input.to_string();
        let normalized = Self::normalize(input);
        Self { original, normalized }
    }

    /// Normalizes a string: uppercase, remove accents, keep only ASCII alphabetic.
    fn normalize(input: &str) -> String {
        input
            .nfd()
            .filter(|c| c.is_ascii_alphabetic())
            .collect::<String>()
            .to_uppercase()
    }

    pub fn len(&self) -> usize {
        self.normalized.len()
    }

    pub fn is_empty(&self) -> bool {
        self.normalized.is_empty()
    }

    pub fn chars(&self) -> Vec<char> {
        self.normalized.chars().collect()
    }
}

/// Sorts words by length in descending order (longest first).
pub fn sort_by_length_desc(words: &mut [Word]) {
    words.sort_by(|a, b| b.len().cmp(&a.len()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_accents() {
        let word = Word::new("café");
        assert_eq!(word.normalized, "CAFE");
    }

    #[test]
    fn test_normalize_portuguese() {
        let word = Word::new("coração");
        assert_eq!(word.normalized, "CORACAO");
    }

    #[test]
    fn test_sort_by_length() {
        let mut words = vec![Word::new("a"), Word::new("abc"), Word::new("ab")];
        sort_by_length_desc(&mut words);
        assert_eq!(words[0].normalized, "ABC");
    }
}
