mod d01;
mod string_error;

use std::time::Instant;
use string_error::StringError;

fn main() -> Result<(), StringError> {
    let inputs = vec![read_input_file("./inputs/input_01")?];

    let t = Instant::now();
    d01::solve(&inputs[0])?;
    let dt = t.elapsed();

    println!("Elapsed time for all solutions: {:?}", dt);

    Ok(())
}

fn read_input_file(path: &str) -> Result<String, StringError> {
    std::fs::read_to_string(path).map_err(|_| format!("Could not read file {}", path).into())
}
