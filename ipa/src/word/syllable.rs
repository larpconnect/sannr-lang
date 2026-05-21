use crate::IpaString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stress {
    Unstressed,
    SecondaryStress,
    PrimaryStress,
}

impl Default for Stress {
    fn default() -> Self {
        Self::Unstressed
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Syllable {
    pub stress: Stress,
    pub content: IpaString,
}

impl std::fmt::Display for Syllable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = match self.stress {
            Stress::PrimaryStress => "ˈ",
            Stress::SecondaryStress => "ˌ",
            Stress::Unstressed => "",
        };
        write!(f, "{prefix}{}", self.content)
    }
}
