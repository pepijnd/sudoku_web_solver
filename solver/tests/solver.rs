use solver::{solvers::Solver, sudoku::Solution, Sudoku};

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
    (
        "...6..8....35.4...65..217...6..............5..7138..2...7.1.6.4.1.......9....3..7",
        "742639815183574269659821743365192478298746351471385926537218694814967532926453187",
    ),
    (
        "....3.76.5....91.29.........49..53.......327...52..........75.4..1.4.....6.......",
        "124538769583679142976124853249715386618493275735286491392867514851942637467351928",
    ),
    (
        "...8.....86....95.3.9..24.8.....132.6.......52...................19.3.4...2..4.3.",
        "125849763864137952379562418958471326643298175217356894436715289581923647792684531",
    ),
    (
        ".5.....43.1.....82.84..7.1..71.9....4.........23.1..6.8..1...26...7.4......9.2...",
        "759821643316549782284637915571296834468375291923418567897153426632784159145962378",
    ),
    (
        "7.82...6...4..8..9..1.5............2..6..1.4.3.....67..4.1.........45.87..5.7...3",
        "758294361234618759691753824489567132576321948312489675847132596163945287925876413",
    ),
    (
        "3...6..42.....5....61.3.....8...7..67...2.1.5.9....3......13...5....6.93..74.....",
        "359761842472985631861234759185347926743629185296158374928513467514876293637492518",
    ),
    (
        "..3..9........23......5..876.......53.94.....4....523...7.13.....6..8.9...12.....",
        "163789524785642319924351687612837945359426178478195236547913862236578491891264753",
    ),
    (
        "....27....13..4.....9..57...8....3..5..9..1......32...651....4...8....9.....4.6.5",
        "865127439713694582429385761984751326532968174176432958651279843348516297297843615",
    ),
];

#[test]
fn solver_solve() {
    for (i, &(sudoku, solution)) in INPUT.iter().enumerate() {
        eprintln!("{}: {}", i, sudoku);
        let solve = Sudoku::from(sudoku).solve();
        if let Solution::Complete(solve) = solve {
            assert_eq!(solve, Sudoku::from(solution));
        } else {
            panic!("No valid solution found");
        };
    }
}

#[test]
fn solver_steps() {
    for &(sudoku, _solution) in INPUT {
        let solve = Sudoku::from(sudoku).solve_steps();
        assert!(solve.end().valid);
        assert_eq!(solve.end().solver, Solver::Solved);
    }
}
