use std::str::FromStr;

use serde::Serialize;

#[derive(Debug, Serialize, Default, Clone)]
pub struct Morph {
    pub lemma: String,
    pub inflection: String,
    pub english: Option<String>,
}

pub struct ParseMorphError;

impl FromStr for Morph {
    type Err = ParseMorphError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let morph = s
            .split(",")
            .map(|t| t.trim().to_lowercase())
            .collect::<Vec<String>>();

        Ok(Morph {
            lemma: morph.get(0).ok_or(ParseMorphError)?.clone(),
            inflection: morph.get(1).ok_or(ParseMorphError)?.clone(),
            english: None,
        })
    }
}
