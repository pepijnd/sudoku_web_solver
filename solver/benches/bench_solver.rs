#![feature(duration_constants)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use solver::{Sudoku, solvers::CageSolver};

static INPUT: &[(&str, &str)] = &[
    (
        "9.4.728.....8.36..8..9.....6.9....1..83..7.....7.....22...385.....729..6...6.....",
        "964572831172843659835961274629485713483217965517396482246138597358729146791654328",
    ),
    (
        "..61.4.9.35...9......25.........5..8......2...324...718...9.3...95...7...4.7.1...",
        "726134895358679142419258637971325468684917253532486971867592314195843726243761589",
    ),
    (
        ".....48..79.58........9.....75....4.1.62.............751.3..2....48....16.24..5..",
        "253614879791583426468792315875936142146257938329148657517369284934825761682471593",
    ),
];

fn solver_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Solver");

    for (i, (input, _)) in INPUT.iter().enumerate() {
        group.bench_with_input(format!("sudoku_{:?}", i), input, |b, i| {
            let s = Sudoku::from(*i);
            b.iter(|| black_box(&s.clone()).solve())
        });
    }

    group.finish();
}

fn sums_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sums");

    group.bench_function("sums", |b| {
        b.iter(|| {
            for s in 1..=9 {
                for t in 1..=45 {
                    black_box(CageSolver::sums(s, t));
                }
            }
        })
    });

    group.finish();
}

criterion_group!{
    name = benches;
    config = Criterion::default();
    targets = solver_benchmark, sums_benchmark
}
criterion_main!(benches);
