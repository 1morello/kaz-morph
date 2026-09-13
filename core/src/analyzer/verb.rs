use crate::types::*;
use super::Analyzer;

impl Analyzer {
    pub(super) fn try_verb(&self, word: &str, results: &mut Vec<MorphAnalysis>) {
        let person_options = self.strip_person(word);
        let mut after_person: Vec<(&str, Option<Person>, Option<Number>)> = person_options;
        after_person.push((word, None, None));

        for (rest0, person, p_number) in &after_person {
            let mut tense_options = self.strip_past_definite(rest0);
            tense_options.extend(self.strip_past_narrative(rest0));
            tense_options.extend(self.strip_past_transitional(rest0));
            tense_options.extend(self.strip_future_indefinite(rest0));
            tense_options.extend(self.strip_future_goal(rest0));
            tense_options.extend(self.strip_present_definite(rest0));
            tense_options.extend(self.strip_present_progressive(rest0));

            for (rest1, tense) in &tense_options {
                let neg_options = self.strip_negation(rest1);
                let mut after_neg: Vec<(&str, bool)> = neg_options;
                after_neg.push((rest1, false));

                for (rest2, negation) in &after_neg {
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
}
