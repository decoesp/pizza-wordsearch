use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone)]
pub struct Word {
    pub original: String,
    pub normalized: String,
}

impl Word {
    pub fn new(input: &str) -> Self {
        let original = input.to_string();
        let normalized = input
            .nfd()
            .filter(|c| c.is_ascii_alphabetic())
            .collect::<String>()
            .to_uppercase();
        Self { original, normalized }
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

pub fn sort_by_length_desc(words: &mut [Word]) {
    words.sort_by(|a, b| b.len().cmp(&a.len()));
}
