/// Part of speech.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pos {
    Noun,
    Verb,
    Adjective,
    Adverb,
    Pronoun,
    Numeral,
    Conjunction,
    Postposition,
    Particle,
    Interjection,
}

/// Grammatical case (7 cases in Kazakh).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Case {
    Nominative,   // атау
    Genitive,     // ілік
    Dative,       // барыс
    Accusative,   // табыс
    Locative,     // жатыс
    Ablative,     // шығыс
    Instrumental, // көмектес
}

/// Grammatical number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Number {
    Singular,
    Plural,
}

/// Possessive agreement (8 forms).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Possession {
    P1Sg,       // менің — мой
    P2Sg,       // сенің — твой
    P2SgFormal, // сіздің — Ваш
    P3Sg,       // оның — его/её
    P1Pl,       // біздің — наш
    P2Pl,       // сендердің — ваш
    P2PlFormal, // сіздердің — Ваш (мн.)
    P3Pl,       // олардың — их
}

/// Person (verb conjugation).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Person {
    First,
    Second,
    SecondFormal,
    Third,
}

/// Tense and mood.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tense {
    PastDefinite,      // барды
    PastNarrative,     // барған
    PastTransitional,  // барып
    PresentDefinite,   // барады
    PresentProgressive,// барда
    FutureIndefinite,  // барар
    FutureGoal,        // бармақ
    Imperative,        // бар!
}

/// Verbal voice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Voice {
    Reflexive,  // -н-  тарану
    Passive,    // -л-  жазылу
    Collective, // -с-  табысу
    Causative,  // -тыр-/-гіз-  барғызу
}

/// Morphological features bundle.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Features {
    pub number: Option<Number>,
    pub case: Option<Case>,
    pub possession: Option<Possession>,
    pub person: Option<Person>,
    pub tense: Option<Tense>,
    pub voice: Option<Voice>,
    pub negation: bool,
}

/// Result of morphological analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct MorphAnalysis {
    pub lemma: String,
    pub pos: Pos,
    pub features: Features,
    pub score: f32,
}
