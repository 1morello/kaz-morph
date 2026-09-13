use crate::types::*;
use super::Analyzer;

impl Analyzer {
    pub(super) fn try_noun(&self, word: &str, results: &mut Vec<MorphAnalysis>) {
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
                    self.try_lookup_noun(stem, *case, *poss, *number, results);
                }
            }
        }
    }

    fn try_lookup_noun(
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
}
