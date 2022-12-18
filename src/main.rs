mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod string_error;

use std::time::Instant;
use string_error::StringError;

fn main() -> Result<(), StringError> {
    let inputs = vec![
        read_input_file("./inputs/input_01")?,
        read_input_file("./inputs/input_02")?,
        read_input_file("./inputs/input_03")?,
        read_input_file("./inputs/input_04")?,
        read_input_file("./inputs/input_05")?,
        read_input_file("./inputs/input_06")?,
        read_input_file("./inputs/input_07")?,
        read_input_file("./inputs/input_08")?,
        read_input_file("./inputs/input_09")?,
        read_input_file("./inputs/input_10")?,
        read_input_file("./inputs/input_11")?,
        read_input_file("./inputs/input_12")?,
        read_input_file("./inputs/input_13")?,
        read_input_file("./inputs/input_14")?,
        read_input_file("./inputs/input_15")?,
    ];

    println!("Read all inputs from disk.\n");

    let mut s = String::new();

    let t = Instant::now();
    s += &d01::solve(&inputs[0])?;
    s += &d02::solve(&inputs[1])?;
    s += &d03::solve(&inputs[2])?;
    s += &d04::solve(&inputs[3])?;
    s += &d05::solve(&inputs[4])?;
    s += &d06::solve(&inputs[5])?;
    s += &d07::solve(&inputs[6])?;
    s += &d08::solve(&inputs[7])?;
    s += &d09::solve(&inputs[8])?;
    s += &d10::solve(&inputs[9])?;
    s += &d11::solve(&inputs[10])?;
    s += &d12::solve(&inputs[11])?;
    s += &d13::solve(&inputs[12])?;
    s += &d14::solve(&inputs[13])?;
    s += &d15::solve(&inputs[14])?;
    let dt = t.elapsed();

    println!("{}", s);

    println!("Elapsed time for all solutions: {:?}", dt);

    Ok(())
}

fn read_input_file(path: &str) -> Result<String, StringError> {
    std::fs::read_to_string(path).map_err(|_| format!("Could not read file {}", path).into())
}
