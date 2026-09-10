use wasm_bindgen::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
struct MorphResult {
    lemma: String,
    pos: String,
    case: Option<String>,
    number: Option<String>,
    possession: Option<String>,
    tense: Option<String>,
    voice: Option<String>,
    person: Option<String>,
    negation: bool,
}

#[wasm_bindgen]
pub struct Analyzer {
    inner: kaz_morph_core::Analyzer,
}

#[wasm_bindgen]
impl Analyzer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: kaz_morph_core::Analyzer::new(),
        }
    }

    /// Analyze a word, returns a JS array of objects.
    pub fn analyze(&self, word: &str) -> JsValue {
        let results: Vec<MorphResult> = self.inner
            .analyze(word)
            .into_iter()
            .map(|r| MorphResult {
                lemma: r.lemma,
                pos: format!("{:?}", r.pos),
                case: r.features.case.map(|c| format!("{c:?}")),
                number: r.features.number.map(|n| format!("{n:?}")),
                possession: r.features.possession.map(|p| format!("{p:?}")),
                tense: r.features.tense.map(|t| format!("{t:?}")),
                voice: r.features.voice.map(|v| format!("{v:?}")),
                person: r.features.person.map(|p| format!("{p:?}")),
                negation: r.features.negation,
            })
            .collect();

        serde_wasm_bindgen::to_value(&results).unwrap()
    }
}
