use crate::string_error::StringError;

pub fn solve(input: &str) -> Result<String, StringError> {
    let mut points_part_one = 0;
    let mut points_part_two = 0;

    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.chars().count() == 0 {
            continue;
        }

        let mut s = trimmed.split(' ');
        let first = s.next().ok_or("d02: could not find other player move")?;
        let second = s.next().ok_or("d02: could not find self player move")?;

        let other_move = str_to_move(first)?;
        let self_move_part_one = str_to_move(second)?;
        let self_move_part_two = intended_result_to_move(&other_move, second)?;

        points_part_one += self_move_part_one.points(&other_move);
        points_part_two += self_move_part_two.points(&other_move);
    }

    let mut result = format!("Day 02/01: {}\n", points_part_one);
    result += &format!("Day 02/02: {}\n", points_part_two);
    Ok(result)
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn result_points(&self, other: &Move) -> u64 {
        if *self == *other {
            3
        } else if *self == other.loses_against() {
            6
        } else {
            0
        }
    }

    fn base_points(&self) -> u64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn points(&self, other: &Move) -> u64 {
        self.base_points() + self.result_points(other)
    }

    fn wins_against(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn loses_against(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }
}

fn str_to_move(s: &str) -> Result<Move, StringError> {
    if s.chars().count() != 1 {
        return Err("d02: move not equal to one char".into());
    }

    let c = s.chars().next().unwrap();

    match c {
        'A' | 'X' => Ok(Move::Rock),
        'B' | 'Y' => Ok(Move::Paper),
        'C' | 'Z' => Ok(Move::Scissors),
        _ => Err("d02: invalid input char".into()),
    }
}

fn intended_result_to_move(other: &Move, r: &str) -> Result<Move, StringError> {
    if r.chars().count() != 1 {
        return Err("d02: intended result not equal to one char".into());
    }

    let c = r.chars().next().unwrap();

    match c {
        'X' => Ok(other.wins_against()),
        'Y' => Ok(*other),
        'Z' => Ok(other.loses_against()),
        _ => Err("d02: invalid input char for intended result".into()),
    }
}
