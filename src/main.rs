mod d01;
mod d02;
mod d03;
mod string_error;

use std::time::Instant;
use string_error::StringError;

fn main() -> Result<(), StringError> {
    let inputs = vec![
        read_input_file("./inputs/input_01")?,
        read_input_file("./inputs/input_02")?,
        read_input_file("./inputs/input_03")?,
    ];

    println!("Read all inputs from disk.\n");

    let mut s = String::new();

    let t = Instant::now();
    s += &d01::solve(&inputs[0])?;
    s += &d02::solve(&inputs[1])?;
    s += &d03::solve(&inputs[2])?;
    let dt = t.elapsed();

    println!("{}", s);

    println!("Elapsed time for all solutions: {:?}", dt);

    Ok(())
}

fn read_input_file(path: &str) -> Result<String, StringError> {
    std::fs::read_to_string(path).map_err(|_| format!("Could not read file {}", path).into())
}
