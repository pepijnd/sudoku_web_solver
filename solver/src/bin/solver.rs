use solver::Sudoku;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let time = std::time::Instant::now();
    if let Some(input) = std::env::args().nth(1) {
        let file = File::open(input)?;
        let reader = BufReader::new(file);
        let mut sum = 0;
        for input in reader.lines() {
            if let Ok(input) = input {
                if input.len() == 81 {
                    let sudoku = Sudoku::from(input);
                    let solve = sudoku.solve_steps();
                    if let Some(step) = solve.iter().last() {
                        sum += step.guesses_t;
                    }
                }
            }
        }
        println!("guesses total: {}", sum);
    }
    println!("{:#?}", time.elapsed());

    Ok(())
}
