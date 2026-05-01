use crate::lexicon::Lexicon;
use crate::phonology;
use crate::types::*;

pub struct Analyzer {
    lexicon: Lexicon,
}

impl Analyzer {
    /// Create an analyzer with the built-in test lexicon.
    pub fn new() -> Self {
        Self {
            lexicon: Lexicon::built_in(),
        }
    }

    /// Create an analyzer with a custom lexicon.
    pub fn with_lexicon(lexicon: Lexicon) -> Self {
        Self { lexicon }
    }

    /// Analyze a word, returning all possible interpretations.
    ///
    /// For nouns, strips suffixes right-to-left:
    /// Case → Possessive → Plural → Root
    ///
    /// Based on Kessikbayeva Figure 1:
    /// ROOT + [Plural] + [Possessive] + [Case]
    pub fn analyze(&self, word: &str) -> Vec<MorphAnalysis> {
        let word = word.to_lowercase();
        let mut results = Vec::new();

        // 1. direct lexicon hit (bare stem, nominative singular)
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

        // 2. Try noun suffix stripping
        self.try_noun(&word, &mut results);

        // 3. Try verb suffix stripping
        self.try_verb(&word, &mut results);

        // 4. try adjective (bare or substantivized)
        self.try_adjective(&word, &mut results);

        // Sort by score descending, deduplicate
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results.dedup_by(|a, b| {
            a.lemma == b.lemma && a.pos == b.pos && a.features == b.features
        });

        results
    }

    /// Try to parse a word as a noun:
    /// ROOT + [Plural] + [Possessive] + [Case]
    ///
    /// We strip from right to left, trying every combination.
    fn try_noun(&self, word: &str, results: &mut Vec<MorphAnalysis>) {
        // Layer 1: try stripping case suffix (or none)
        let case_options = self.strip_case(word);
        let mut after_case: Vec<(&str, Option<Case>)> = case_options;
        after_case.push((word, None));

        for (rest1, case) in &after_case {
            // Layer 2: try stripping possessive suffix (or none)
            let poss_options = self.strip_possessive(rest1);
            let mut after_poss: Vec<(&str, Option<Possession>)> = poss_options;
            after_poss.push((rest1, None));

            for (rest2, poss) in &after_poss {
                // Layer 3: try stripping plural suffix (or none)
                let plural_options = self.strip_plural(rest2);
                let mut after_plural: Vec<(&str, Option<Number>)> = plural_options;
                after_plural.push((rest2, None));

                for (stem, number) in &after_plural {
                    // Try to find stem in lexicon (with mutation variants)
                    self.try_lookup_stem(stem, *case, *poss, *number, results);
                }
            }
        }
    }

    /// Try to parse a word as a verb:
    /// ROOT + [Negation] + [Tense]
    ///
    /// Starting simple: past definite only.
    /// gang
    fn try_verb(&self, word: &str, results: &mut Vec<MorphAnalysis>) {
        // Layer 0: strip person suffix (or none)
        let person_options = self.strip_person(word);
        let mut after_person: Vec<(&str, Option<Person>, Option<Number>)> = person_options;
        after_person.push((word, None, None));

        for (rest0, person, p_number) in &after_person {
            // Layer 1: strip tense suffix
            let mut tense_options = self.strip_past_definite(rest0);
            tense_options.extend(self.strip_past_narrative(rest0));
            tense_options.extend(self.strip_past_transitional(rest0));
            tense_options.extend(self.strip_future_indefinite(rest0));
            tense_options.extend(self.strip_future_goal(rest0));
            tense_options.extend(self.strip_present_definite(rest0));
            tense_options.extend(self.strip_present_progressive(rest0));

            for (rest1, tense) in &tense_options {
                // Layer 2: strip negation (or none)
                let neg_options = self.strip_negation(rest1);
                let mut after_neg: Vec<(&str, bool)> = neg_options;
                after_neg.push((rest1, false));

                for (rest2, negation) in &after_neg {
                    // Layer 3: strip voice (or none)
                    let voice_options = self.strip_voice(rest2);
                    let mut after_voice: Vec<(&str, Option<Voice>)> = voice_options;
                    after_voice.push((rest2, None));

                    for (stem, voice) in &after_voice {
                        let variants = self.stem_variants(stem);
                        for variant in &variants {
                            if let Some(entries) = self.lexicon.lookup(variant) {
                                for e in entries {
                                    if e.pos != Pos::Verb {
                                        continue;
                                    }
                                    let features = Features {
                                        tense: *tense,
                                        negation: *negation,
                                        voice: *voice,
                                        person: *person,
                                        number: *p_number,
                                        ..Default::default()
                                    };
                                    results.push(MorphAnalysis {
                                        lemma: e.lemma.clone(),
                                        pos: Pos::Verb,
                                        features,
                                        score: 0.6,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Try to parse a word as an adjective.
    ///
    /// Adjectives can be bare (жақсы) or substantivized,
    /// taking noun suffixes like plural, case, possessive.
    fn try_adjective(&self, word: &str, results: &mut Vec<MorphAnalysis>) {
        // Same layers as nouns, but looking for adjective stems
        let case_options = self.strip_case(word);
        let mut after_case: Vec<(&str, Option<Case>)> = case_options;
        after_case.push((word, None));

        for (rest1, case) in &after_case {
            let poss_options = self.strip_possessive(rest1);
            let mut after_poss: Vec<(&str, Option<Possession>)> = poss_options;
            after_poss.push((rest1, None));

            for (rest2, poss) in &after_poss {
                let plural_options = self.strip_plural(rest2);
                let mut after_plural: Vec<(&str, Option<Number>)> = plural_options;
                after_plural.push((rest2, None));

                for (stem, number) in &after_plural {
                    if stem.is_empty() { continue; }
                    if let Some(entries) = self.lexicon.lookup(stem) {
                        for e in entries {
                            if e.pos != Pos::Adjective {
                                continue;
                            }
                            let features = Features {
                                number: *number,
                                case: *case,
                                possession: *poss,
                                ..Default::default()
                            };
                            results.push(MorphAnalysis {
                                lemma: e.lemma.clone(),
                                pos: Pos::Adjective,
                                features,
                                score: 0.5,
                            });
                        }
                    }
                }
            }
        }
    }


    /// Look up a stem in the lexicon, also trying reverse
    /// consonant mutations (б→п, г→к, ғ→қ).
    fn try_lookup_stem(
        &self,
        stem: &str,
        case: Option<Case>,
        poss: Option<Possession>,
        number: Option<Number>,
        results: &mut Vec<MorphAnalysis>,
    ) {
        if stem.is_empty() {
            return;
        }

        let variants = self.stem_variants(stem);

        for variant in &variants {
            if let Some(entries) = self.lexicon.lookup(variant) {
                for e in entries {
                    if e.pos != Pos::Noun {
                        continue;
                    }
                    let features = Features {
                        number,
                        case,
                        possession: poss,
                        ..Default::default()
                    };

                    // Score: prefer longer matches (more suffixes consumed)
                    let suffix_len = stem.len().abs_diff(variant.len());
                    let score = 0.5 + (suffix_len as f32 * 0.05);

                    results.push(MorphAnalysis {
                        lemma: e.lemma.clone(),
                        pos: Pos::Noun,
                        features,
                        score,
                    });
                }
            }
        }
    }

    /// Generate stem variants by reversing consonant mutations.
    ///
    /// Yiner rules 10-12:
    ///   б → п  (мектеб → мектеп)
    ///   г → к  (eg → ek)
    ///   ғ → қ  (балығ → балық)
    fn stem_variants<'a>(&self, stem: &'a str) -> Vec<String> {
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

    // ────────────────────────────────────────────────────
    //  Suffix stripping tables
    //
    //  Based on Kessikbayeva Table 4 and Yiner rules 4-9.
    //  Each method returns (remaining_str, parsed_feature).
    // ────────────────────────────────────────────────────

    /// Strip case suffixes.
    ///
    /// Locative:     -да/-де, -та/-те
    /// Ablative:     -дан/-ден, -тан/-тен, -нан/-нен
    /// Dative:       -ға/-ге, -қа/-ке, -на/-не
    /// Accusative:   -ны/-ні, -ды/-ді, -ты/-ті
    /// Genitive:     -ның/-нің, -дың/-дің, -тың/-тің
    /// Instrumental: -мен/-бен/-пен, -менен/-бенен/-пенен
    fn strip_case<'a>(&self, word: &'a str) -> Vec<(&'a str, Option<Case>)> {
        let table: &[(&[&str], Case)] = &[
            // Longer suffixes first to avoid partial matches
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

    /// Strip possessive suffixes.
    ///
    /// Order: try longer suffixes first.
    ///
    /// P1Pl:       -ымыз/-іміз/-ұмыз/-үміз
    /// P2SgFormal: -ыңыз/-іңіз
    /// P2Pl:       -ларың/-лерің/-дарың/-дерің/-тарың/-терің
    /// P2Sg:       -ың/-ің
    /// P3Sg:       -сы/-сі (after vowel), -ы/-і (after consonant)
    /// P1Sg:       -ым/-ім
    fn strip_possessive<'a>(&self, word: &'a str) -> Vec<(&'a str, Option<Possession>)> {
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

    /// Strip plural suffixes: -лар/-лер, -дар/-дер, -тар/-тер
    fn strip_plural<'a>(&self, word: &'a str) -> Vec<(&'a str, Option<Number>)> {
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

    // ────────────────────────────────────────────────────
    //  Verbal suffix stripping
    // ────────────────────────────────────────────────────

    /// Strip past definite tense suffix: -ды/-ді/-ты/-ті
    fn strip_past_definite<'a>(&self, word: &'a str) -> Vec<(&'a str, Option<Tense>)> {
        let suffixes = ["ды", "ді", "ты", "ті"];
        self.strip_tense_suffix(word, &suffixes, Tense::PastDefinite)
    }

    /// Strip past narrative tense suffix: -ған/-ген/-қан/-кен
    fn strip_past_narrative<'a>(&self, word: &'a str) -> Vec<(&'a str, Option<Tense>)> {
        let suffixes = ["ған", "ген", "қан", "кен"];
        self.strip_tense_suffix(word, &suffixes, Tense::PastNarrative)
    }

    /// Strip past transitional suffix: -ып/-іп/-п
    fn strip_past_transitional<'a>(&self, word: &'a str) -> Vec<(&'a str, Option<Tense>)> {
        let suffixes = ["ып", "іп", "п"];
        self.strip_tense_suffix(word, &suffixes, Tense::PastTransitional)
    }

    /// Strip future indefinite suffix: -ар/-ер/-р
    fn strip_future_indefinite<'a>(&self, word: &'a str) -> Vec<(&'a str, Option<Tense>)> {
        let suffixes = ["ар", "ер", "р"];
        self.strip_tense_suffix(word, &suffixes, Tense::FutureIndefinite)
    }

    /// Strip future goal suffix: -мақ/-мек/-бақ/-бек/-пақ/-пек
    fn strip_future_goal<'a>(&self, word: &'a str) -> Vec<(&'a str, Option<Tense>)> {
        let suffixes = ["мақ", "мек", "бақ", "бек", "пақ", "пек"];
        self.strip_tense_suffix(word, &suffixes, Tense::FutureGoal)
    }

    /// Strip present definite tense suffix: -ады/-еді
    fn strip_present_definite<'a>(&self, word: &'a str) -> Vec<(&'a str, Option<Tense>)> {
        let suffixes = ["ады", "еді"];
        self.strip_tense_suffix(word, &suffixes, Tense::PresentDefinite)
    }

    /// Strip present progressive tense suffix: -уда/-уде/-юда/-юде
    fn strip_present_progressive<'a>(&self, word: &'a str) -> Vec<(&'a str, Option<Tense>)> {
        let suffixes = ["уда", "уде", "юда", "юде"];
        self.strip_tense_suffix(word, &suffixes, Tense::PresentProgressive)
    }

    /// Generic tense suffix stripper — all tense methods delegate here.
    fn strip_tense_suffix<'a>(
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

    /// Strip negation suffix: -ма/-ме/-ба/-бе/-па/-пе
    fn strip_negation<'a>(&self, word: &'a str) -> Vec<(&'a str, bool)> {
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

    /// Strip voice suffix.
    ///
    /// Reflexive:  -ын/-ін/-н
    /// Passive:    -ыл/-іл/-л
    /// Collective: -ыс/-іс/-с
    /// Causative:  -тыр/-тір/-дыр/-дір/-ғыз/-гіз/-қыз/-кіз
    fn strip_voice<'a>(&self, word: &'a str) -> Vec<(&'a str, Option<Voice>)> {
        let table: &[(&[&str], Voice)] = &[
            // Longer suffixes first
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

    /// Strip person/number agreement suffix.
    ///
    /// These come after the tense suffix:
    /// 1Sg: -м/-мын/-мін
    /// 2Sg: -ң/-сың/-сің
    /// 1Pl: -қ/-к/-мыз/-міз
    /// 2Pl: -ңдар/-ңдер/-сыңдар/-сіңдер
    /// 3rd person has no overt suffix (zero).
    fn strip_person<'a>(&self, word: &'a str) -> Vec<(&'a str, Option<Person>, Option<Number>)> {
        let table: &[(&[&str], Person, Number)] = &[
            // Longer suffixes first
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

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn analyzer() -> Analyzer {
        Analyzer::new()
    }

    /// Helper: find the first noun analysis.
    fn first_noun(word: &str) -> MorphAnalysis {
        let a = analyzer();
        let results = a.analyze(word);
        results
            .into_iter()
            .find(|r| r.pos == Pos::Noun)
            .unwrap_or_else(|| panic!("no noun analysis for '{word}'"))
    }

    // Bare stems

    #[test]
    fn bare_noun() {
        let r = first_noun("мектеп");
        assert_eq!(r.lemma, "мектеп");
        assert_eq!(r.features.case, None);
        assert_eq!(r.features.number, None);
    }

    // Case suffixes

    #[test]
    fn locative() {
        let r = first_noun("мектепте");
        assert_eq!(r.lemma, "мектеп");
        assert_eq!(r.features.case, Some(Case::Locative));
    }

    #[test]
    fn ablative() {
        let r = first_noun("мектептен");
        assert_eq!(r.lemma, "мектеп");
        assert_eq!(r.features.case, Some(Case::Ablative));
    }

    #[test]
    fn dative() {
        let r = first_noun("мектепке");
        assert_eq!(r.lemma, "мектеп");
        assert_eq!(r.features.case, Some(Case::Dative));
    }

    #[test]
    fn accusative() {
        let r = first_noun("мектепті");
        assert_eq!(r.lemma, "мектеп");
        assert_eq!(r.features.case, Some(Case::Accusative));
    }

    #[test]
    fn genitive() {
        let r = first_noun("мектептің");
        assert_eq!(r.lemma, "мектеп");
        assert_eq!(r.features.case, Some(Case::Genitive));
    }

    // Plural

    #[test]
    fn plural() {
        let r = first_noun("мектептер");
        assert_eq!(r.lemma, "мектеп");
        assert_eq!(r.features.number, Some(Number::Plural));
    }

    #[test]
    fn plural_locative() {
        let r = first_noun("мектептерде");
        assert_eq!(r.lemma, "мектеп");
        assert_eq!(r.features.number, Some(Number::Plural));
        assert_eq!(r.features.case, Some(Case::Locative));
    }

    // Possessive

    #[test]
    fn possessive_p1sg() {
        let r = first_noun("мектебім");
        assert_eq!(r.lemma, "мектеп");
        assert_eq!(r.features.possession, Some(Possession::P1Sg));
    }

    #[test]
    fn possessive_p3sg_mutation() {
        // п → б 
        let r = first_noun("мектебі");
        assert_eq!(r.lemma, "мектеп");
        assert_eq!(r.features.possession, Some(Possession::P3Sg));
    }

    #[test]
    fn possessive_p1pl() {
        let r = first_noun("мектебіміз");
        assert_eq!(r.lemma, "мектеп");
        assert_eq!(r.features.possession, Some(Possession::P1Pl));
    }

    // Consonant mutations

    #[test]
    fn mutation_q_to_gh() {
        // қ → ғ 
        let r = first_noun("балығы");
        assert_eq!(r.lemma, "балық");
        assert_eq!(r.features.possession, Some(Possession::P3Sg));
    }

    // Full chain: plural + possessive + case 

    #[test]
    fn full_chain() {
        // мектеп + тер + іміз + де
        let r = first_noun("мектептерімізде");
        assert_eq!(r.lemma, "мектеп");
        assert_eq!(r.features.number, Some(Number::Plural));
        assert_eq!(r.features.possession, Some(Possession::P1Pl));
        assert_eq!(r.features.case, Some(Case::Locative));
    }

    #[test]
    fn full_chain_heart() {
        // жүрек + тер + іміз + де 
        let r = first_noun("жүректерімізде");
        assert_eq!(r.lemma, "жүрек");
        assert_eq!(r.features.number, Some(Number::Plural));
        assert_eq!(r.features.possession, Some(Possession::P1Pl));
        assert_eq!(r.features.case, Some(Case::Locative));
    }
    // Vowel-ending stems

    #[test]
    fn vowel_stem_locative() {
        // қала + да (not та — stem ends in vowel)
        let r = first_noun("қалада");
        assert_eq!(r.lemma, "қала");
        assert_eq!(r.features.case, Some(Case::Locative));
    }

    #[test]
    fn vowel_stem_ablative() {
        let r = first_noun("қаладан");
        assert_eq!(r.lemma, "қала");
        assert_eq!(r.features.case, Some(Case::Ablative));
    }

    #[test]
    fn vowel_stem_dative() {
        let r = first_noun("қалаға");
        assert_eq!(r.lemma, "қала");
        assert_eq!(r.features.case, Some(Case::Dative));
    }

    #[test]
    fn vowel_stem_genitive() {
        let r = first_noun("қаланың");
        assert_eq!(r.lemma, "қала");
        assert_eq!(r.features.case, Some(Case::Genitive));
    }

    #[test]
    fn vowel_stem_accusative() {
        let r = first_noun("қаланы");
        assert_eq!(r.lemma, "қала");
        assert_eq!(r.features.case, Some(Case::Accusative));
    }

    #[test]
    fn vowel_stem_plural() {
        let r = first_noun("қалалар");
        assert_eq!(r.lemma, "қала");
        assert_eq!(r.features.number, Some(Number::Plural));
    }

    #[test]
    fn vowel_stem_possessive_p3sg() {
        // қала + сы (after vowel, с stays — Yiner rule 15)
        let r = first_noun("қаласы");
        assert_eq!(r.lemma, "қала");
        assert_eq!(r.features.possession, Some(Possession::P3Sg));
    }

    // Instrumental case

    #[test]
    fn instrumental_men() {
        let r = first_noun("қаламен");
        assert_eq!(r.lemma, "қала");
        assert_eq!(r.features.case, Some(Case::Instrumental));
    }

    #[test]
    fn instrumental_pen() {
        let r = first_noun("мектеппен");
        assert_eq!(r.lemma, "мектеп");
        assert_eq!(r.features.case, Some(Case::Instrumental));
    }

    #[test]
    fn instrumental_ben() {
        let r = first_noun("көзбен");
        assert_eq!(r.lemma, "көз");
        assert_eq!(r.features.case, Some(Case::Instrumental));
    }

    // Voiced-stem words

    #[test]
    fn voiced_stem_locative() {
        // көз + де (voiced consonant → д variant)
        let r = first_noun("көзде");
        assert_eq!(r.lemma, "көз");
        assert_eq!(r.features.case, Some(Case::Locative));
    }

    #[test]
    fn voiced_stem_plural() {
        let r = first_noun("көздер");
        assert_eq!(r.lemma, "көз");
        assert_eq!(r.features.number, Some(Number::Plural));
    }

    // Different words, same pattern

    #[test]
    fn book_dative() {
        let r = first_noun("кітапқа");
        assert_eq!(r.lemma, "кітап");
        assert_eq!(r.features.case, Some(Case::Dative));
    }

    #[test]
    fn child_plural_genitive() {
        let r = first_noun("балалардың");
        assert_eq!(r.lemma, "бала");
        assert_eq!(r.features.number, Some(Number::Plural));
        assert_eq!(r.features.case, Some(Case::Genitive));
    }

    #[test]
    fn house_possessive_p1sg() {
        let r = first_noun("үйім");
        assert_eq!(r.lemma, "үй");
        assert_eq!(r.features.possession, Some(Possession::P1Sg));
    }

    #[test]
    fn table_locative() {
        let r = first_noun("үстелде");
        assert_eq!(r.lemma, "үстел");
        assert_eq!(r.features.case, Some(Case::Locative));
    }
    

    // -- Verbs: Past Definite --

    fn first_verb(word: &str) -> MorphAnalysis {
        let a = analyzer();
        let results = a.analyze(word);
        results
            .into_iter()
            .find(|r| r.pos == Pos::Verb)
            .unwrap_or_else(|| panic!("no verb analysis for '{word}'"))
    }

    #[test]
    fn verb_past_definite() {
        let r = first_verb("барды");
        assert_eq!(r.lemma, "бар");
        assert_eq!(r.features.tense, Some(Tense::PastDefinite));
    }

    #[test]
    fn verb_past_definite_front() {
        let r = first_verb("келді");
        assert_eq!(r.lemma, "кел");
        assert_eq!(r.features.tense, Some(Tense::PastDefinite));
    }

    #[test]
    fn verb_past_definite_voiceless() {
        // айт + ты (voiceless stem → т variant)
        let r = first_verb("айтты");
        assert_eq!(r.lemma, "айт");
        assert_eq!(r.features.tense, Some(Tense::PastDefinite));
    }

    #[test]
    fn verb_past_definite_negative() {
        // бар + ма + ды
        let r = first_verb("бармады");
        assert_eq!(r.lemma, "бар");
        assert_eq!(r.features.tense, Some(Tense::PastDefinite));
        assert!(r.features.negation);
    }

    #[test]
    fn verb_past_definite_negative_front() {
        // кел + ме + ді
        let r = first_verb("келмеді");
        assert_eq!(r.lemma, "кел");
        assert_eq!(r.features.tense, Some(Tense::PastDefinite));
        assert!(r.features.negation);
    }

    // Verbs: Past Narrative

    #[test]
    fn verb_past_narrative() {
        let r = first_verb("барған");
        assert_eq!(r.lemma, "бар");
        assert_eq!(r.features.tense, Some(Tense::PastNarrative));
    }

    #[test]
    fn verb_past_narrative_front() {
        let r = first_verb("келген");
        assert_eq!(r.lemma, "кел");
        assert_eq!(r.features.tense, Some(Tense::PastNarrative));
    }

    #[test]
    fn verb_past_narrative_voiceless() {
        // айт + қан (voiceless stem → қ variant)
        let r = first_verb("айтқан");
        assert_eq!(r.lemma, "айт");
        assert_eq!(r.features.tense, Some(Tense::PastNarrative));
    }

    #[test]
    fn verb_past_narrative_negative() {
        // бар + ма + ған
        let r = first_verb("бармаған");
        assert_eq!(r.lemma, "бар");
        assert_eq!(r.features.tense, Some(Tense::PastNarrative));
        assert!(r.features.negation);
    }

    // Verbs: Past Transitional 

    #[test]
    fn verb_past_transitional() {
        // бар + ып
        let r = first_verb("барып");
        assert_eq!(r.lemma, "бар");
        assert_eq!(r.features.tense, Some(Tense::PastTransitional));
    }

    #[test]
    fn verb_past_transitional_front() {
        // кел + іп
        let r = first_verb("келіп");
        assert_eq!(r.lemma, "кел");
        assert_eq!(r.features.tense, Some(Tense::PastTransitional));
    }

    // Verbs: Future Indefinite

    #[test]
    fn verb_future_indefinite() {
        // бар + ар
        let r = first_verb("барар");
        assert_eq!(r.lemma, "бар");
        assert_eq!(r.features.tense, Some(Tense::FutureIndefinite));
    }

    #[test]
    fn verb_future_indefinite_front() {
        // кел + ер
        let r = first_verb("келер");
        assert_eq!(r.lemma, "кел");
        assert_eq!(r.features.tense, Some(Tense::FutureIndefinite));
    }

    // Verbs: Future Goal

    #[test]
    fn verb_future_goal() {
        // бар + мақ
        let r = first_verb("бармақ");
        assert_eq!(r.lemma, "бар");
        assert_eq!(r.features.tense, Some(Tense::FutureGoal));
    }

    #[test]
    fn verb_future_goal_front() {
        // кел + мек
        let r = first_verb("келмек");
        assert_eq!(r.lemma, "кел");
        assert_eq!(r.features.tense, Some(Tense::FutureGoal));
    }

    // Verbs: Voice

    #[test]
    fn verb_passive_past() {
        // жаз + ыл + ды (был написан)
        let r = first_verb("жазылды");
        assert_eq!(r.lemma, "жаз");
        assert_eq!(r.features.voice, Some(Voice::Passive));
        assert_eq!(r.features.tense, Some(Tense::PastDefinite));
    }

    #[test]
    fn verb_causative_past() {
        // бар + ғыз + ды (заставил пойти)
        let r = first_verb("барғызды");
        assert_eq!(r.lemma, "бар");
        assert_eq!(r.features.voice, Some(Voice::Causative));
        assert_eq!(r.features.tense, Some(Tense::PastDefinite));
    }

    #[test]
    fn verb_causative_negative() {
        // бар + ғыз + ба + ды (не заставил пойти)
        let r = first_verb("барғызбады");
        assert_eq!(r.lemma, "бар");
        assert_eq!(r.features.voice, Some(Voice::Causative));
        assert_eq!(r.features.negation, true);
        assert_eq!(r.features.tense, Some(Tense::PastDefinite));
    }

    // Verbs: Person/Number 

    #[test]
    fn verb_past_1sg() {
        // бар + ды + м
        let r = first_verb("бардым");
        assert_eq!(r.lemma, "бар");
        assert_eq!(r.features.tense, Some(Tense::PastDefinite));
        assert_eq!(r.features.person, Some(Person::First));
        assert_eq!(r.features.number, Some(Number::Singular));
    }

    #[test]
    fn verb_past_2sg() {
        // бар + ды + ң
        let r = first_verb("бардың");
        assert_eq!(r.lemma, "бар");
        assert_eq!(r.features.tense, Some(Tense::PastDefinite));
        assert_eq!(r.features.person, Some(Person::Second));
    }

    #[test]
    fn verb_past_1pl() {
        // бар + ды + қ
        let r = first_verb("бардық");
        assert_eq!(r.lemma, "бар");
        assert_eq!(r.features.tense, Some(Tense::PastDefinite));
        assert_eq!(r.features.person, Some(Person::First));
        assert_eq!(r.features.number, Some(Number::Plural));
    }

    #[test]
    fn verb_past_front_1sg() {
        // кел + ді + м
        let r = first_verb("келдім");
        assert_eq!(r.lemma, "кел");
        assert_eq!(r.features.tense, Some(Tense::PastDefinite));
        assert_eq!(r.features.person, Some(Person::First));
    }

    #[test]
    fn verb_past_negative_1sg() {
        // бар + ма + ды + м
        let r = first_verb("бармадым");
        assert_eq!(r.lemma, "бар");
        assert_eq!(r.features.tense, Some(Tense::PastDefinite));
        assert_eq!(r.features.negation, true);
        assert_eq!(r.features.person, Some(Person::First));
    }

    #[test]
    fn verb_past_2pl() {
        // кел + ді + ңдер
        let r = first_verb("келдіңдер");
        assert_eq!(r.lemma, "кел");
        assert_eq!(r.features.tense, Some(Tense::PastDefinite));
        assert_eq!(r.features.person, Some(Person::Second));
        assert_eq!(r.features.number, Some(Number::Plural));
    }

    // Verbs: Present Definite 

    #[test]
    fn verb_present_definite() {
        let r = first_verb("барады");
        assert_eq!(r.lemma, "бар");
        assert_eq!(r.features.tense, Some(Tense::PresentDefinite));
    }

    #[test]
    fn verb_present_definite_front() {
        let r = first_verb("келеді");
        assert_eq!(r.lemma, "кел");
        assert_eq!(r.features.tense, Some(Tense::PresentDefinite));
    }

    // Verbs: Present Progressive

    #[test]
    fn verb_present_progressive() {
        let r = first_verb("баруда");
        assert_eq!(r.lemma, "бар");
        assert_eq!(r.features.tense, Some(Tense::PresentProgressive));
    }

    #[test]
    fn verb_present_progressive_front() {
        let r = first_verb("келуде");
        assert_eq!(r.lemma, "кел");
        assert_eq!(r.features.tense, Some(Tense::PresentProgressive));
    }

}
