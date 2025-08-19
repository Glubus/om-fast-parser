use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use om_fast_parser::OsuParser;
use std::path::Path;

fn benchmark_parse_file(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser");
    
    group.bench_function("parse_smk_osu", |b| {
        b.iter(|| {
            let mut parser = OsuParser::new();
            parser.parse_file(Path::new("assets/smk.osu")).unwrap();
            black_box(parser.hit_objects.len())
        });
    });

    group.bench_function("parse_veneno_osu", |b| {
        b.iter(|| {
            let mut parser = OsuParser::new();
            parser.parse_file(Path::new("assets/veneno.osu")).unwrap();
            black_box(parser.hit_objects.len())
        });
    });
    
    group.finish();
}

criterion_group!(benches, benchmark_parse_file);
criterion_main!(benches);
