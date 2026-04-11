use kaz_morph::Analyzer;
use std::env;

fn main() {
    let analyzer = Analyzer::new();
    let words: Vec<String> = env::args().skip(1).collect();

    if words.is_empty() {
        use std::io::{self, BufRead};
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line.expect("failed to read line");
            let word = line.trim();
            if word.is_empty() { continue; }
            print_word(&analyzer, word);
        }
    } else {
        for word in &words {
            print_word(&analyzer, word);
        }
    }
}

fn print_word(analyzer: &Analyzer, word: &str) {
    let results = analyzer.analyze(word);
    println!("  {word}");
    println!();

    if results.is_empty() {
        println!("  no analysis found");
    } else {
        for (i, r) in results.iter().enumerate() {
            println!("  {}. lemma: {}  pos: {:?}", i + 1, r.lemma, r.pos);
            print_features(&r.features);
            println!();
        }
    }
}

fn print_features(f: &kaz_morph::Features) {
    if let Some(n) = &f.number     { println!("     number:     {n:?}"); }
    if let Some(c) = &f.case       { println!("     case:       {c:?}"); }
    if let Some(p) = &f.possession { println!("     possession: {p:?}"); }
    if let Some(t) = &f.tense      { println!("     tense:      {t:?}"); }
    if f.negation                   { println!("     negation:   true"); }
}
