# kaz-morph

Rule-based morphological analyzer for Kazakh: written in Rust, with bindings for Python and WASM.

## The problem

Kazakh is agglutinative. Grammar is encoded as chains of suffixes on a single root:

```
жүректерімізде
  жүрек  +  тер  +  іміз  +  де
  heart     PL      our      in
  -> "in our hearts"
```

For a computer, `жүрек` and `жүректерімізде` are unrelated strings. A spellchecker can't tell whether a five-suffix chain follows vowel harmony or breaks it. On top of that, an NLP pipeline has no idea that `барды`, `бармады`, and `барғандар` are all forms of `бару`.

Without structural understanding of words, every downstream tool for Kazakh is either broken or faking it.

kaz-morph is here to fix this.

## Usage

```rust
use kaz_morph::Analyzer;

let a = Analyzer::new();
let r = a.analyze("жүректерімізде");
// → lemma: "жүрек", pos: Noun, number: Plural,
//   possession: P1Pl, case: Locative
```

## Status

Early development — setting up the foundation.

## License

MIT
