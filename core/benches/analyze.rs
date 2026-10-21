use criterion::{criterion_group, criterion_main, Criterion};
use kaz_morph::Analyzer;

fn bench_single_word(c: &mut Criterion) {
    let a = Analyzer::new();
    c.bench_function("bare noun", |b| {
        b.iter(|| a.analyze("мектеп"))
    });
}

fn bench_suffixed_noun(c: &mut Criterion) {
    let a = Analyzer::new();
    c.bench_function("noun full chain", |b| {
        b.iter(|| a.analyze("мектептерімізде"))
    });
}

fn bench_verb(c: &mut Criterion) {
    let a = Analyzer::new();
    c.bench_function("verb past definite", |b| {
        b.iter(|| a.analyze("бармадым"))
    });
}

fn bench_full_lexicon_load(c: &mut Criterion) {
    c.bench_function("full lexicon load", |b| {
        b.iter(|| Analyzer::full())
    });
}

fn bench_full_lexicon_analyze(c: &mut Criterion) {
    let a = Analyzer::full();
    c.bench_function("full lexicon analyze", |b| {
        b.iter(|| a.analyze("жүректерімізде"))
    });
}

fn bench_batch(c: &mut Criterion) {
    let a = Analyzer::new();
    let words = vec![
        "мектеп", "мектепте", "мектептерімізде",
        "барды", "бармады", "келген",
        "жақсы", "жақсылар", "маған",
        "бүгін",
    ];
    c.bench_function("batch 10 words", |b| {
        b.iter(|| {
            for w in &words {
                a.analyze(w);
            }
        })
    });
}

criterion_group!(
    benches,
    bench_single_word,
    bench_suffixed_noun,
    bench_verb,
    bench_full_lexicon_load,
    bench_full_lexicon_analyze,
    bench_batch,
);
criterion_main!(benches);
