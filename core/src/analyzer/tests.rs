use super::*;

fn analyzer() -> Analyzer {
    Analyzer::new()
}

fn first_noun(word: &str) -> MorphAnalysis {
    let a = analyzer();
    let results = a.analyze(word);
    results
        .into_iter()
        .find(|r| r.pos == Pos::Noun)
        .unwrap_or_else(|| panic!("no noun analysis for '{word}'"))
}

fn first_verb(word: &str) -> MorphAnalysis {
    let a = analyzer();
    let results = a.analyze(word);
    results
        .into_iter()
        .find(|r| r.pos == Pos::Verb)
        .unwrap_or_else(|| panic!("no verb analysis for '{word}'"))
}

fn first_adj(word: &str) -> MorphAnalysis {
    let a = analyzer();
    let results = a.analyze(word);
    results
        .into_iter()
        .find(|r| r.pos == Pos::Adjective)
        .unwrap_or_else(|| panic!("no adjective analysis for '{word}'"))
}

fn first_pronoun(word: &str) -> MorphAnalysis {
    let a = analyzer();
    let results = a.analyze(word);
    results
        .into_iter()
        .find(|r| r.pos == Pos::Pronoun)
        .unwrap_or_else(|| panic!("no pronoun analysis for '{word}'"))
}

#[test]
fn bare_noun() {
    let r = first_noun("мектеп");
    assert_eq!(r.lemma, "мектеп");
    assert_eq!(r.features.case, None);
    assert_eq!(r.features.number, None);
}

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

#[test]
fn possessive_p1sg() {
    let r = first_noun("мектебім");
    assert_eq!(r.lemma, "мектеп");
    assert_eq!(r.features.possession, Some(Possession::P1Sg));
}

#[test]
fn possessive_p3sg_mutation() {
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

#[test]
fn mutation_q_to_gh() {
    let r = first_noun("балығы");
    assert_eq!(r.lemma, "балық");
    assert_eq!(r.features.possession, Some(Possession::P3Sg));
}

#[test]
fn full_chain() {
    let r = first_noun("мектептерімізде");
    assert_eq!(r.lemma, "мектеп");
    assert_eq!(r.features.number, Some(Number::Plural));
    assert_eq!(r.features.possession, Some(Possession::P1Pl));
    assert_eq!(r.features.case, Some(Case::Locative));
}

#[test]
fn full_chain_heart() {
    let r = first_noun("жүректерімізде");
    assert_eq!(r.lemma, "жүрек");
    assert_eq!(r.features.number, Some(Number::Plural));
    assert_eq!(r.features.possession, Some(Possession::P1Pl));
    assert_eq!(r.features.case, Some(Case::Locative));
}

#[test]
fn vowel_stem_locative() {
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
    let r = first_noun("қаласы");
    assert_eq!(r.lemma, "қала");
    assert_eq!(r.features.possession, Some(Possession::P3Sg));
}

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

#[test]
fn voiced_stem_locative() {
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
    let r = first_verb("айтты");
    assert_eq!(r.lemma, "айт");
    assert_eq!(r.features.tense, Some(Tense::PastDefinite));
}

#[test]
fn verb_past_definite_negative() {
    let r = first_verb("бармады");
    assert_eq!(r.lemma, "бар");
    assert_eq!(r.features.tense, Some(Tense::PastDefinite));
    assert!(r.features.negation);
}

#[test]
fn verb_past_definite_negative_front() {
    let r = first_verb("келмеді");
    assert_eq!(r.lemma, "кел");
    assert_eq!(r.features.tense, Some(Tense::PastDefinite));
    assert!(r.features.negation);
}

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
    let r = first_verb("айтқан");
    assert_eq!(r.lemma, "айт");
    assert_eq!(r.features.tense, Some(Tense::PastNarrative));
}

#[test]
fn verb_past_narrative_negative() {
    let r = first_verb("бармаған");
    assert_eq!(r.lemma, "бар");
    assert_eq!(r.features.tense, Some(Tense::PastNarrative));
    assert!(r.features.negation);
}

#[test]
fn verb_past_transitional() {
    let r = first_verb("барып");
    assert_eq!(r.lemma, "бар");
    assert_eq!(r.features.tense, Some(Tense::PastTransitional));
}

#[test]
fn verb_past_transitional_front() {
    let r = first_verb("келіп");
    assert_eq!(r.lemma, "кел");
    assert_eq!(r.features.tense, Some(Tense::PastTransitional));
}

#[test]
fn verb_future_indefinite() {
    let r = first_verb("барар");
    assert_eq!(r.lemma, "бар");
    assert_eq!(r.features.tense, Some(Tense::FutureIndefinite));
}

#[test]
fn verb_future_indefinite_front() {
    let r = first_verb("келер");
    assert_eq!(r.lemma, "кел");
    assert_eq!(r.features.tense, Some(Tense::FutureIndefinite));
}

#[test]
fn verb_future_goal() {
    let r = first_verb("бармақ");
    assert_eq!(r.lemma, "бар");
    assert_eq!(r.features.tense, Some(Tense::FutureGoal));
}

#[test]
fn verb_future_goal_front() {
    let r = first_verb("келмек");
    assert_eq!(r.lemma, "кел");
    assert_eq!(r.features.tense, Some(Tense::FutureGoal));
}

#[test]
fn verb_passive_past() {
    let r = first_verb("жазылды");
    assert_eq!(r.lemma, "жаз");
    assert_eq!(r.features.voice, Some(Voice::Passive));
    assert_eq!(r.features.tense, Some(Tense::PastDefinite));
}

#[test]
fn verb_causative_past() {
    let r = first_verb("барғызды");
    assert_eq!(r.lemma, "бар");
    assert_eq!(r.features.voice, Some(Voice::Causative));
    assert_eq!(r.features.tense, Some(Tense::PastDefinite));
}

#[test]
fn verb_causative_negative() {
    let r = first_verb("барғызбады");
    assert_eq!(r.lemma, "бар");
    assert_eq!(r.features.voice, Some(Voice::Causative));
    assert_eq!(r.features.negation, true);
    assert_eq!(r.features.tense, Some(Tense::PastDefinite));
}

#[test]
fn verb_past_1sg() {
    let r = first_verb("бардым");
    assert_eq!(r.lemma, "бар");
    assert_eq!(r.features.tense, Some(Tense::PastDefinite));
    assert_eq!(r.features.person, Some(Person::First));
    assert_eq!(r.features.number, Some(Number::Singular));
}

#[test]
fn verb_past_2sg() {
    let r = first_verb("бардың");
    assert_eq!(r.lemma, "бар");
    assert_eq!(r.features.tense, Some(Tense::PastDefinite));
    assert_eq!(r.features.person, Some(Person::Second));
}

#[test]
fn verb_past_1pl() {
    let r = first_verb("бардық");
    assert_eq!(r.lemma, "бар");
    assert_eq!(r.features.tense, Some(Tense::PastDefinite));
    assert_eq!(r.features.person, Some(Person::First));
    assert_eq!(r.features.number, Some(Number::Plural));
}

#[test]
fn verb_past_front_1sg() {
    let r = first_verb("келдім");
    assert_eq!(r.lemma, "кел");
    assert_eq!(r.features.tense, Some(Tense::PastDefinite));
    assert_eq!(r.features.person, Some(Person::First));
}

#[test]
fn verb_past_negative_1sg() {
    let r = first_verb("бармадым");
    assert_eq!(r.lemma, "бар");
    assert_eq!(r.features.tense, Some(Tense::PastDefinite));
    assert_eq!(r.features.negation, true);
    assert_eq!(r.features.person, Some(Person::First));
}

#[test]
fn verb_past_2pl() {
    let r = first_verb("келдіңдер");
    assert_eq!(r.lemma, "кел");
    assert_eq!(r.features.tense, Some(Tense::PastDefinite));
    assert_eq!(r.features.person, Some(Person::Second));
    assert_eq!(r.features.number, Some(Number::Plural));
}

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

#[test]
fn bare_adjective() {
    let r = first_adj("жақсы");
    assert_eq!(r.lemma, "жақсы");
    assert_eq!(r.features.case, None);
}

#[test]
fn adjective_substantivized_plural() {
    let r = first_adj("жақсылар");
    assert_eq!(r.lemma, "жақсы");
    assert_eq!(r.features.number, Some(Number::Plural));
}

#[test]
fn adjective_substantivized_dative() {
    let r = first_adj("жақсыға");
    assert_eq!(r.lemma, "жақсы");
    assert_eq!(r.features.case, Some(Case::Dative));
}

#[test]
fn adjective_substantivized_locative() {
    let r = first_adj("жаманда");
    assert_eq!(r.lemma, "жаман");
    assert_eq!(r.features.case, Some(Case::Locative));
}

#[test]
fn bare_adverb() {
    let a = analyzer();
    let results = a.analyze("тез");
    let r = results.iter().find(|r| r.pos == Pos::Adverb)
        .expect("no adverb analysis for 'тез'");
    assert_eq!(r.lemma, "тез");
}

#[test]
fn bare_adverb_today() {
    let a = analyzer();
    let results = a.analyze("бүгін");
    let r = results.iter().find(|r| r.pos == Pos::Adverb)
        .expect("no adverb analysis for 'бүгін'");
    assert_eq!(r.lemma, "бүгін");
}

#[test]
fn bare_pronoun() {
    let r = first_pronoun("мен");
    assert_eq!(r.lemma, "мен");
}

#[test]
fn pronoun_men_dative() {
    let r = first_pronoun("маған");
    assert_eq!(r.lemma, "мен");
    assert_eq!(r.features.case, Some(Case::Dative));
}

#[test]
fn pronoun_sen_dative() {
    let r = first_pronoun("саған");
    assert_eq!(r.lemma, "сен");
    assert_eq!(r.features.case, Some(Case::Dative));
}

#[test]
fn pronoun_ol_genitive() {
    let r = first_pronoun("оның");
    assert_eq!(r.lemma, "ол");
    assert_eq!(r.features.case, Some(Case::Genitive));
}

#[test]
fn pronoun_biz_instrumental() {
    let r = first_pronoun("бізбен");
    assert_eq!(r.lemma, "біз");
    assert_eq!(r.features.case, Some(Case::Instrumental));
}

#[test]
fn pronoun_olar_accusative() {
    let r = first_pronoun("оларды");
    assert_eq!(r.lemma, "олар");
    assert_eq!(r.features.case, Some(Case::Accusative));
}
