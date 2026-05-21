use crate::word::syllable::{Stress, Syllable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IpaWord {
    pub syllables: Vec<Syllable>,
}

impl std::fmt::Display for IpaWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for syllable in &self.syllables {
            if !first && syllable.stress == Stress::Unstressed {
                write!(f, ".")?;
            }
            write!(f, "{syllable}")?;
            first = false;
        }
        Ok(())
    }
}

impl IpaWord {
    pub fn new(syllables: Vec<Syllable>) -> Self {
        Self { syllables }
    }
}
