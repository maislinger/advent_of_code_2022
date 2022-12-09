use std::collections::BTreeSet;

use crate::string_error::StringError;

pub fn solve(input: &str) -> Result<String, StringError> {
    let input = input.trim();

    const N_KNOTS: usize = 10;
    let mut knots = [Knot::default(); N_KNOTS];

    let mut visited_part1 = BTreeSet::new();
    let mut visited_part2 = BTreeSet::new();
    visited_part1.insert(knots[1]);
    visited_part2.insert(knots[9]);

    for line in input.lines() {
        let mut m = KnotMove::from_str(line)?;

        while !m.is_zero() {
            knots[0].apply_one_step(&mut m);

            for i in 1..N_KNOTS {
                let p = knots[i - 1];
                let moved = knots[i].follow(&p);
                if !moved {
                    break;
                }
            }

            visited_part1.insert(knots[1]);
            visited_part2.insert(knots[9]);
        }
    }

    let mut result = format!("Day 09/01: {}\n", visited_part1.len());
    result += &format!("Day 09/02: {}\n", visited_part2.len());
    Ok(result)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Default)]
struct Knot {
    x: i64,
    y: i64,
}

impl Knot {
    fn apply_one_step(&mut self, m: &mut KnotMove) {
        match m {
            KnotMove::X(a) => {
                let s = a.signum();
                self.x += s;
                *a -= s;
            }
            KnotMove::Y(a) => {
                let s = a.signum();
                self.y += s;
                *a -= s;
            }
        }
    }

    fn follow(&mut self, other: &Self) -> bool {
        let delta_x = self.x - other.x;
        let delta_y = self.y - other.y;
        let delta_x_abs = delta_x.abs();
        let delta_y_abs = delta_y.abs();

        if delta_x_abs <= 1 && delta_y_abs <= 1 {
            return false;
        }

        let delta_x2 = delta_x - delta_x.signum();
        let delta_y2 = delta_y - delta_y.signum();

        self.x = other.x + delta_x2;
        self.y = other.y + delta_y2;

        true
    }
}

enum KnotMove {
    X(i64),
    Y(i64),
}

impl KnotMove {
    fn from_str(input: &str) -> Result<Self, StringError> {
        let mut splitted = input.split_whitespace();
        let first = splitted.next().ok_or("d09: invalid input.")?;
        let second = splitted.next().ok_or("d09: invalid input.")?;

        let amount = second.parse().map_err(|_| "d09: amount is not numeric.")?;

        match first {
            "R" => Ok(KnotMove::X(amount)),
            "U" => Ok(KnotMove::Y(amount)),
            "L" => Ok(KnotMove::X(-amount)),
            "D" => Ok(KnotMove::Y(-amount)),
            _ => Err("d09: invalid direction.".into()),
        }
    }

    fn is_zero(&self) -> bool {
        let c = match self {
            KnotMove::X(a) => a,
            KnotMove::Y(a) => a,
        };

        *c == 0
    }
}
