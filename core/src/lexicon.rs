use std::collections::HashMap;

use crate::types::Pos;

/// A single entry in the lexicon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexEntry {
    pub lemma: String,
    pub pos: Pos,
}

/// In-memory lexicon of Kazakh root words.
///
/// Currently backed by a `HashMap` for simplicity.
/// Will be replaced by an FST (finite state transducer)
/// when we scale to 20k+ entries from Apertium-kaz.
#[derive(Debug, Clone)]
pub struct Lexicon {
    entries: HashMap<String, Vec<LexEntry>>,
}

impl Lexicon {
    /// Create an empty lexicon.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Insert a root into the lexicon.
    pub fn insert(&mut self, stem: &str, pos: Pos) {
        self.entries
            .entry(stem.to_string())
            .or_default()
            .push(LexEntry {
                lemma: stem.to_string(),
                pos,
            });
    }

    /// Look up all entries for a given stem.
    pub fn lookup(&self, stem: &str) -> Option<&[LexEntry]> {
        self.entries.get(stem).map(Vec::as_slice)
    }

    /// Number of unique stem forms.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Build a small lexicon for testing.
    ///
    /// Covers enough roots to validate noun analysis
    /// in Phase 1: cases, plural, possessive, mutations.
    pub fn built_in() -> Self {
        let mut lex = Self::new();

        // -- Nouns
        let nouns = [
            "адам",   // человек
            "ағаш",   // дерево
            "алма",   // яблоко
            "ана",    // мать
            "ата",    // дед, предок
            "бала",   // ребёнок
            "балық",  // рыба
            "бас",    // голова
            "жер",    // земля
            "жол",    // дорога
            "жүрек",  // сердце
            "кітап",  // книга
            "көз",    // глаз
            "қала",   // город
            "қол",    // рука
            "мектеп", // школа
            "су",     // вода
            "тау",    // гора
            "тауық",  // курица
            "үй",    // дом
            "үстел",  // стол
        ];
        for stem in nouns {
            lex.insert(stem, Pos::Noun);
        }

        // Nouns with vowel drop (Yiner rule 14):
        // ауыз, мойын, мұрын — stem vowel drops
        // instead of suffix vowel.
        let nouns_vowel_drop = [
            "ауыз",  // рот
            "мойын", // шея
            "мұрын", // нос
        ];
        for stem in nouns_vowel_drop {
            lex.insert(stem, Pos::Noun);
        }

        // -- Verbs
        let verbs = [
            "ал",    // брать
            "айт",   // говорить
            "бар",   // идти
            "бер",   // давать
            "біл",   // знать
            "же",    // есть
            "жаз",   // писать
            "жүр",   // ходить
            "іш",    // пить
            "кел",   // приходить
            "көр",   // видеть
            "оқы",   // читать
            "отыр",  // сидеть
            "тұр",   // стоять
            "жат",   // лежать
        ];
        for stem in verbs {
            lex.insert(stem, Pos::Verb);
        }

        // Adjectives gang
        let adjectives = [
            "үлкен", // большой
            "кіші",  // маленький
            "жақсы", // хороший
            "жаман", // плохой
            "жылы",  // тёплый
        ];
        for stem in adjectives {
            lex.insert(stem, Pos::Adjective);
        }

        // -- Adverbs 
        let adverbs = [
            "тез",    // быстро
            "баяу",   // медленно
            "жылдам", // быстро
            "ерте",   // рано
            "кеш",    // поздно
            "бүгін",  // сегодня
            "кеше",   // вчера
            "ертең",  // завтра
        ];
        for stem in adverbs {
            lex.insert(stem, Pos::Adverb);
        }

        // -- Pronouns
        let pronouns = [
            "мен",   // я
            "сен",   // ты
            "ол",    // он/она
            "біз",   // мы
            "сіз",   // Вы
            "олар",  // они
        ];
        for stem in pronouns {
            lex.insert(stem, Pos::Pronoun);
        }

        lex
    }
}

impl Default for Lexicon {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_existing() {
        let lex = Lexicon::built_in();
        let results = lex.lookup("мектеп").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].pos, Pos::Noun);
    }

    #[test]
    fn lookup_missing() {
        let lex = Lexicon::built_in();
        assert!(lex.lookup("xxxxxx").is_none());
    }

    #[test]
    fn lookup_verb() {
        let lex = Lexicon::built_in();
        let results = lex.lookup("бар").unwrap();
        assert_eq!(results[0].pos, Pos::Verb);
    }

    #[test]
    fn multiple_pos() {
        let mut lex = Lexicon::built_in();
        // жүз — и существительное (лицо), и числительное (сто)
        lex.insert("жүз", Pos::Noun);
        lex.insert("жүз", Pos::Numeral);
        let results = lex.lookup("жүз").unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn built_in_size() {
        let lex = Lexicon::built_in();
        assert!(lex.len() >= 30);
    }
}
