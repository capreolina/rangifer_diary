use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion,
};
use rangifer_diary_062::{
    fubini, fubini_gen, fubini_par, fubini_par_hack, fubini_rec,
};

pub fn fubini_compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fubinis");

    for i in [20, 50, 75, 100, 150, 200, 250].iter() {
        group.bench_with_input(BenchmarkId::new("fubini", i), i, |b, i| {
            b.iter(|| fubini(black_box(*i)))
        });

        group.bench_with_input(
            BenchmarkId::new("fubini_gen", i),
            i,
            |b, i| b.iter(|| fubini_gen(black_box(*i))),
        );

        group.bench_with_input(
            BenchmarkId::new("fubini_par", i),
            i,
            |b, i| b.iter(|| fubini_par(black_box(*i))),
        );

        group.bench_with_input(
            BenchmarkId::new("fubini_par_hack", i),
            i,
            |b, i| b.iter(|| fubini_par_hack(black_box(*i))),
        );

        group.bench_with_input(
            BenchmarkId::new("fubini_rec", i),
            i,
            |b, i| b.iter(|| fubini_rec(black_box(*i))),
        );
    }

    group.finish();
}

criterion_group!(benches, fubini_compare);
criterion_main!(benches);
