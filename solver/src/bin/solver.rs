use solver::Sudoku;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let time = std::time::Instant::now();
    if let Some(input) = std::env::args().nth(1) {
        let file = File::open(input)?;
        let reader = BufReader::new(file);
        let mut sum = 0;
        reader.lines().for_each(|input| {
            if let Ok(input) = input {
                if input.len() == 81 {
                    let sudoku = Sudoku::from(input);
                    let _solve = sudoku.solve(None);
                    let _ = sum += 1;
                    // if let Some(step) = solve.iter().last() {
                    //     sum += step.guesses_t;
                    // }
                }
            }
        });
        println!("guesses total: {}", sum);
    }
    println!("{:#?}", time.elapsed());

    Ok(())
}
