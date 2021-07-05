#![feature(duration_constants)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use solver::config::{Config, ConfigDescriptor};
use solver::rules::{Cages, Rules};
use solver::Sudoku;

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
            b.iter(|| black_box(&s.clone()).solve(None, None))
        });
    }

    group.finish();
}

fn killer_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Killer");

    let sudoku = Sudoku::from(
        "8.1...9....927.....5....4.3.............5.....3............3......8...4..8.5.4...",
    );

    group.bench_function("sudoku", |b| {
        let mut config = ConfigDescriptor {
            rules: Rules {
                cages: Cages {
                    cages: vec![
                        15, 7, 15, 25, 5, 13, 6, 12, 4, 11, 9, 8, 5, 12, 23, 33, 16, 12, 9, 1, 5,
                        12, 14, 22, 13, 18, 9, 8, 8, 25, 8, 6, 7, 9,
                    ],
                    cells: [
                        1, 1, 2, 2, 3, 3, 4, 4, 4, 5, 6, 6, 2, 3, 7, 4, 8, 9, 5, 10, 10, 11, 12,
                        13, 13, 8, 9, 14, 15, 15, 16, 16, 16, 16, 17, 17, 14, 15, 18, 18, 18, 16,
                        19, 19, 17, 20, 21, 21, 22, 22, 16, 19, 23, 24, 25, 26, 26, 22, 27, 28, 28,
                        23, 24, 25, 26, 29, 30, 30, 30, 31, 31, 24, 32, 26, 29, 33, 33, 30, 31, 34,
                        24,
                    ],
                },
            },
            ..Default::default()
        };
        config.add_rules_solvers();
        let config = Config::new(config);
        b.iter(|| {
            black_box(sudoku.solve(Some(config.clone()), None));
        })
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = solver_benchmark, killer_benchmark
}
criterion_main!(benches);
