mod noun;
mod verb;
mod adjective;
mod suffixes;

use crate::irregular;
use crate::lexicon::Lexicon;
use crate::phonology;
use crate::types::*;

pub struct Analyzer {
    lexicon: Lexicon,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            lexicon: Lexicon::built_in(),
        }
    }

    pub fn with_lexicon(lexicon: Lexicon) -> Self {
        Self { lexicon }
    }

    pub fn full() -> Self {
        let data = include_str!("../../../data/lexicon/apertium.tsv");
        let mut lexicon = Lexicon::from_tsv(data);

        for stem in ["мен", "сен", "ол", "біз", "сіз", "олар"] {
            lexicon.insert(stem, Pos::Pronoun);
        }

        Self { lexicon }
    }

    pub fn analyze(&self, word: &str) -> Vec<MorphAnalysis> {
        let word = word.to_lowercase();
        let mut results = Vec::new();

        // 1. Direct lexicon hit
        if let Some(entries) = self.lexicon.lookup(&word) {
            for e in entries {
                results.push(MorphAnalysis {
                    lemma: e.lemma.clone(),
                    pos: e.pos,
                    features: Features::default(),
                    score: 1.0,
                });
            }
        }

        // 2. Noun suffix stripping
        self.try_noun(&word, &mut results);

        // 3. Verb suffix stripping
        self.try_verb(&word, &mut results);

        // 4. Adjective (bare or substantivized)
        self.try_adjective(&word, &mut results);

        // 5. Adverbs
        if let Some(entries) = self.lexicon.lookup(&word) {
            for e in entries {
                if e.pos == Pos::Adverb {
                    results.push(MorphAnalysis {
                        lemma: e.lemma.clone(),
                        pos: Pos::Adverb,
                        features: Features::default(),
                        score: 0.9,
                    });
                }
            }
        }

        // 6. Irregular forms
        for form in irregular::irregular_forms() {
            if word == form.surface {
                results.push(MorphAnalysis {
                    lemma: form.lemma.to_string(),
                    pos: form.pos,
                    features: form.features,
                    score: 1.0,
                });
            }
        }

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results.dedup_by(|a, b| {
            a.lemma == b.lemma && a.pos == b.pos && a.features == b.features
        });

        results
    }

    /// Generate stem variants by reversing consonant mutations.
    fn stem_variants(&self, stem: &str) -> Vec<String> {
        let mut variants = vec![stem.to_string()];

        let chars: Vec<char> = stem.chars().collect();
        if let Some(&last) = chars.last() {
            if let Some(devoiced) = phonology::voiced_to_devoiced(last) {
                let mut restored: String = chars[..chars.len() - 1].iter().collect();
                restored.push(devoiced);
                variants.push(restored);
            }
        }

        variants
    }
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
