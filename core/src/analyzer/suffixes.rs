use crate::types::*;
use super::Analyzer;

impl Analyzer {
    // Nominal suffixes

    pub(super) fn strip_case<'a>(&self, word: &'a str) -> Vec<(&'a str, Option<Case>)> {
        let table: &[(&[&str], Case)] = &[
            (&["менен", "бенен", "пенен"], Case::Instrumental),
            (&["мен", "бен", "пен"], Case::Instrumental),
            (&["ның", "нің", "дың", "дің", "тың", "тің"], Case::Genitive),
            (&["дан", "ден", "тан", "тен", "нан", "нен"], Case::Ablative),
            (&["да", "де", "та", "те"], Case::Locative),
            (&["ға", "ге", "қа", "ке", "на", "не"], Case::Dative),
            (&["ны", "ні", "ды", "ді", "ты", "ті"], Case::Accusative),
        ];

        let mut results = Vec::new();
        for (suffixes, case) in table {
            for sfx in *suffixes {
                if let Some(stem) = word.strip_suffix(sfx) {
                    if !stem.is_empty() {
                        results.push((stem, Some(*case)));
                    }
                }
            }
        }
        results
    }

    pub(super) fn strip_possessive<'a>(&self, word: &'a str) -> Vec<(&'a str, Option<Possession>)> {
        let table: &[(&[&str], Possession)] = &[
            (&["ымыз", "іміз", "ұмыз", "үміз"], Possession::P1Pl),
            (&["ыңыз", "іңіз", "ұңыз", "үңіз"], Possession::P2SgFormal),
            (&["ың", "ің", "ұң", "үң"], Possession::P2Sg),
            (&["сы", "сі"], Possession::P3Sg),
            (&["ы", "і"], Possession::P3Sg),
            (&["ым", "ім"], Possession::P1Sg),
        ];

        let mut results = Vec::new();
        for (suffixes, poss) in table {
            for sfx in *suffixes {
                if let Some(stem) = word.strip_suffix(sfx) {
                    if !stem.is_empty() {
                        results.push((stem, Some(*poss)));
                    }
                }
            }
        }
        results
    }

    pub(super) fn strip_plural<'a>(&self, word: &'a str) -> Vec<(&'a str, Option<Number>)> {
        let suffixes = ["лар", "лер", "дар", "дер", "тар", "тер"];

        let mut results = Vec::new();
        for sfx in suffixes {
            if let Some(stem) = word.strip_suffix(sfx) {
                if !stem.is_empty() {
                    results.push((stem, Some(Number::Plural)));
                }
            }
        }
        results
    }

    // Verbal suffixes

    pub(super) fn strip_past_definite<'a>(&self, word: &'a str) -> Vec<(&'a str, Option<Tense>)> {
        self.strip_tense_suffix(word, &["ды", "ді", "ты", "ті"], Tense::PastDefinite)
    }

    pub(super) fn strip_past_narrative<'a>(&self, word: &'a str) -> Vec<(&'a str, Option<Tense>)> {
        self.strip_tense_suffix(word, &["ған", "ген", "қан", "кен"], Tense::PastNarrative)
    }

    pub(super) fn strip_past_transitional<'a>(&self, word: &'a str) -> Vec<(&'a str, Option<Tense>)> {
        self.strip_tense_suffix(word, &["ып", "іп", "п"], Tense::PastTransitional)
    }

    pub(super) fn strip_future_indefinite<'a>(&self, word: &'a str) -> Vec<(&'a str, Option<Tense>)> {
        self.strip_tense_suffix(word, &["ар", "ер", "р"], Tense::FutureIndefinite)
    }

    pub(super) fn strip_future_goal<'a>(&self, word: &'a str) -> Vec<(&'a str, Option<Tense>)> {
        self.strip_tense_suffix(word, &["мақ", "мек", "бақ", "бек", "пақ", "пек"], Tense::FutureGoal)
    }

    pub(super) fn strip_present_definite<'a>(&self, word: &'a str) -> Vec<(&'a str, Option<Tense>)> {
        self.strip_tense_suffix(word, &["ады", "еді"], Tense::PresentDefinite)
    }

    pub(super) fn strip_present_progressive<'a>(&self, word: &'a str) -> Vec<(&'a str, Option<Tense>)> {
        self.strip_tense_suffix(word, &["уда", "уде", "юда", "юде"], Tense::PresentProgressive)
    }

    pub(super) fn strip_tense_suffix<'a>(
        &self,
        word: &'a str,
        suffixes: &[&str],
        tense: Tense,
    ) -> Vec<(&'a str, Option<Tense>)> {
        let mut results = Vec::new();
        for sfx in suffixes {
            if let Some(stem) = word.strip_suffix(sfx) {
                if !stem.is_empty() {
                    results.push((stem, Some(tense)));
                }
            }
        }
        results
    }

    pub(super) fn strip_negation<'a>(&self, word: &'a str) -> Vec<(&'a str, bool)> {
        let suffixes = ["ма", "ме", "ба", "бе", "па", "пе"];

        let mut results = Vec::new();
        for sfx in suffixes {
            if let Some(stem) = word.strip_suffix(sfx) {
                if !stem.is_empty() {
                    results.push((stem, true));
                }
            }
        }
        results
    }

    pub(super) fn strip_voice<'a>(&self, word: &'a str) -> Vec<(&'a str, Option<Voice>)> {
        let table: &[(&[&str], Voice)] = &[
            (&["тыр", "тір", "дыр", "дір", "ғыз", "гіз", "қыз", "кіз"], Voice::Causative),
            (&["ын", "ін"], Voice::Reflexive),
            (&["ыл", "іл"], Voice::Passive),
            (&["ыс", "іс"], Voice::Collective),
            (&["н"], Voice::Reflexive),
            (&["л"], Voice::Passive),
            (&["с"], Voice::Collective),
        ];

        let mut results = Vec::new();
        for (suffixes, voice) in table {
            for sfx in *suffixes {
                if let Some(stem) = word.strip_suffix(sfx) {
                    if !stem.is_empty() {
                        results.push((stem, Some(*voice)));
                    }
                }
            }
        }
        results
    }

    pub(super) fn strip_person<'a>(&self, word: &'a str) -> Vec<(&'a str, Option<Person>, Option<Number>)> {
        let table: &[(&[&str], Person, Number)] = &[
            (&["сыңдар", "сіңдер"], Person::Second, Number::Plural),
            (&["ңдар", "ңдер"], Person::Second, Number::Plural),
            (&["мыз", "міз"], Person::First, Number::Plural),
            (&["мын", "мін"], Person::First, Number::Singular),
            (&["сың", "сің"], Person::Second, Number::Singular),
            (&["м"], Person::First, Number::Singular),
            (&["ң"], Person::Second, Number::Singular),
            (&["қ", "к"], Person::First, Number::Plural),
        ];

        let mut results = Vec::new();
        for (suffixes, person, number) in table {
            for sfx in *suffixes {
                if let Some(stem) = word.strip_suffix(sfx) {
                    if !stem.is_empty() {
                        results.push((stem, Some(*person), Some(*number)));
                    }
                }
            }
        }
        results
    }
}
