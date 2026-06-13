use pyo3::prelude::*;

/// Python wrapper for MorphAnalysis result.
#[pyclass(from_py_object)]
#[derive(Clone)]
struct MorphResult {
    #[pyo3(get)]
    lemma: String,
    #[pyo3(get)]
    pos: String,
    #[pyo3(get)]
    case: Option<String>,
    #[pyo3(get)]
    number: Option<String>,
    #[pyo3(get)]
    possession: Option<String>,
    #[pyo3(get)]
    tense: Option<String>,
    #[pyo3(get)]
    voice: Option<String>,
    #[pyo3(get)]
    person: Option<String>,
    #[pyo3(get)]
    negation: bool,
}

#[pymethods]
impl MorphResult {
    fn __repr__(&self) -> String {
        format!(
            "MorphResult(lemma='{}', pos='{}')",
            self.lemma, self.pos
        )
    }
}

/// Python wrapper for the Analyzer.
#[pyclass]
struct Analyzer {
    inner: kaz_morph_core::Analyzer,
}

#[pymethods]
impl Analyzer {
    #[new]
    fn new() -> Self {
        Self {
            inner: kaz_morph_core::Analyzer::new(),
        }
    }

    /// Analyze a word, returning a list of possible analyses.
    fn analyze(&self, word: &str) -> Vec<MorphResult> {
        self.inner
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
            .collect()
    }
}

/// kaz_morph Python module
#[pymodule]
fn kaz_morph(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Analyzer>()?;
    m.add_class::<MorphResult>()?;
    Ok(())
}
