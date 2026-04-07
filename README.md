# kaz-morph

Rule-based morphological analyzer for Kazakh — written in Rust, with bindings for Python and WASM.

## The problem

Kazakh is agglutinative. Grammar is encoded as chains of suffixes on a single root:

```
қалаларымыздан
  қала  +  лар  +  ымыз  +  дан
  city     PL      our      from
  → "from our cities"
```

For a computer, `қала` and `қалаларымыздан` are unrelated strings. Search breaks, spellcheck breaks, NLP breaks. There is no open, fast, embeddable library that solves this.

kaz-morph is that library.

## Usage

```rust
use kaz_morph::Analyzer;

let a = Analyzer::new();
let r = a.analyze("қалаларымыздан");
// → lemma: "қала", pos: Noun, number: Plural,
//   possession: P1Pl, case: Ablative
```

## Status

Early development — setting up the foundation.

## License

MIT
