use std::collections::VecDeque;

use crate::string_error::StringError;

pub fn solve(input: &str) -> Result<String, StringError> {
    let input = input.trim();

    let mut monkeys = parse_input(input)?;
    let mut monkeys2 = monkeys.clone();
    for _ in 0..20 {
        monkeys.round(true);
    }
    for _ in 0..10000 {
        monkeys2.round(false);
    }

    let mut result = format!("Day 11/01: {}\n", monkeys.monkey_business());
    result += &format!("Day 11/02: {}\n", monkeys2.monkey_business());
    Ok(result)
}

#[derive(Debug, Clone)]
struct Monkey {
    n_turns: usize,
    items: VecDeque<i64>,
    operation: Operation,
    divisor_for_test: i64,
    true_destination: usize,
    false_destination: usize,
}

trait MonkeyBusinessable {
    fn inspect(&mut self, i: usize, make_manageable: bool);
    fn round(&mut self, make_manageable: bool);
    fn monkey_business(&self) -> usize;
}

impl MonkeyBusinessable for Vec<Monkey> {
    fn inspect(&mut self, i: usize, make_manageable: bool) {
        while !self[i].items.is_empty() {
            let worry_level = self[i].items.pop_front().unwrap();
            self[i].n_turns += 1;

            let new_worry_level_intermediate = match self[i].operation {
                Operation::Add(Value::Old, Value::Old) => worry_level + worry_level,
                Operation::Add(Value::Literal(c), Value::Old) => c + worry_level,
                Operation::Add(Value::Old, Value::Literal(c)) => worry_level + c,
                Operation::Add(Value::Literal(c), Value::Literal(d)) => c + d,
                Operation::Sub(Value::Old, Value::Old) => 0,
                Operation::Sub(Value::Literal(c), Value::Old) => c - worry_level,
                Operation::Sub(Value::Old, Value::Literal(c)) => worry_level - c,
                Operation::Sub(Value::Literal(c), Value::Literal(d)) => c - d,
                Operation::Mul(Value::Old, Value::Old) => worry_level * worry_level,
                Operation::Mul(Value::Literal(c), Value::Old) => c * worry_level,
                Operation::Mul(Value::Old, Value::Literal(c)) => worry_level * c,
                Operation::Mul(Value::Literal(c), Value::Literal(d)) => c * d,
                Operation::Div(Value::Old, Value::Old) => 1,
                Operation::Div(Value::Literal(c), Value::Old) => c / worry_level,
                Operation::Div(Value::Old, Value::Literal(c)) => worry_level / c,
                Operation::Div(Value::Literal(c), Value::Literal(d)) => c / d,
            };

            let new_worry_level = if make_manageable {
                new_worry_level_intermediate / 3
            } else {
                new_worry_level_intermediate
            };

            let divisor = self[i].divisor_for_test;
            if new_worry_level % divisor == 0 {
                let j = self[i].true_destination;
                self[j].items.push_back(new_worry_level);
            } else {
                let j = self[i].false_destination;
                self[j].items.push_back(new_worry_level);
            }
        }
    }

    fn round(&mut self, make_manageable: bool) {
        for i in 0..self.len() {
            self.inspect(i, make_manageable);
        }

        if !make_manageable {
            let p = self.iter().map(|m| m.divisor_for_test).product::<i64>();
            self.iter_mut()
                .flat_map(|m| m.items.iter_mut())
                .map(|w| *w %= p)
                .count();
        }
    }

    fn monkey_business(&self) -> usize {
        let mut most_turns = [0; 2];
        for m in self.iter() {
            if m.n_turns > most_turns[0] {
                most_turns[1] = most_turns[0];
                most_turns[0] = m.n_turns;
            } else if m.n_turns > most_turns[1] {
                most_turns[1] = m.n_turns;
            }
        }
        most_turns[0] * most_turns[1]
    }
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Old,
    Literal(i64),
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Div(Value, Value),
}

fn parse_input(input: &str) -> Result<Vec<Monkey>, StringError> {
    let check_monkey_number = |line: &str, target: usize| -> Result<(), StringError> {
        if !line.starts_with("Monkey ") {
            return Err("d11: Invalid input. Line should start with \"Monkey\"".into());
        }

        if !line.ends_with(':') {
            return Err("d11: Invalid input. Monkey number line should end with \":\"".into());
        }

        let trimmed = line.trim_start_matches("Monkey ").trim_end_matches(':');

        let np = trimmed.parse::<usize>();

        match np {
            Ok(n) => {
                if n == target {
                    Ok(())
                } else {
                    Err("d11: Invalid input. Wrong monkey number".into())
                }
            }
            Err(_) => Err("d11: Invalid Input. Monkey number not a number".into()),
        }
    };

    let parse_items = |line: &str| -> Result<VecDeque<i64>, StringError> {
        let mut result = VecDeque::new();

        if !line.starts_with("Starting items: ") {
            return Err("d11: Invalid input. Line should start with \"Starting items: \"".into());
        }

        let trimmed = line.trim_start_matches("Starting items: ");
        for ns in trimmed.split(',').map(|s| s.trim()) {
            let n = ns
                .parse::<i64>()
                .map_err(|_| "d11: Could not parse item number.")?;

            result.push_back(n);
        }

        Ok(result)
    };

    let string_to_value = |s: &str| -> Result<Value, StringError> {
        if s.trim() == "old" {
            return Ok(Value::Old);
        }

        let v = s
            .trim()
            .parse::<i64>()
            .map_err(|_| "d11: could not parse literal.")?;

        Ok(Value::Literal(v))
    };

    let parse_operation = |line: &str| -> Result<Operation, StringError> {
        if !line.starts_with("Operation: new = ") {
            return Err("d11: Invalid input. Line should start with \"Operation: new = \"".into());
        }

        let trimmed = line.trim_start_matches("Operation: new = ");
        let mut splitted = trimmed.split_whitespace();

        let first = splitted
            .next()
            .ok_or("d11: Operation not enough arguments.")?;
        let second = splitted
            .next()
            .ok_or("d11: Operation not enough arguments.")?;
        let third = splitted
            .next()
            .ok_or("d11: Operation not enough arguments.")?;

        let left = string_to_value(first)?;
        let right = string_to_value(third)?;

        match second.trim() {
            "+" => Ok(Operation::Add(left, right)),
            "-" => Ok(Operation::Sub(left, right)),
            "*" => Ok(Operation::Mul(left, right)),
            "/" => Ok(Operation::Div(left, right)),
            _ => Err("d11: Unknown Operation.".into()),
        }
    };

    let parse_test = |line: &str| -> Result<i64, StringError> {
        if !line.starts_with("Test: divisible by ") {
            return Err(
                "d11: Invalid input. Line should start with \"Test: divisible by \"".into(),
            );
        }

        let trimmed = line.trim_start_matches("Test: divisible by ");
        let v = trimmed
            .trim()
            .parse::<i64>()
            .map_err(|_| "d11: could not parse test number.")?;

        Ok(v)
    };

    let parse_destination = |line: &str, do_true: bool| -> Result<usize, StringError> {
        if do_true && !line.starts_with("If true: throw to monkey ") {
            return Err(
                "d11: Invalid input. Line should start with \"If true: throw to monkey \"".into(),
            );
        }

        if !do_true && !line.starts_with("If false: throw to monkey ") {
            return Err(
                "d11: Invalid input. Line should start with \"If false: throw to monkey \"".into(),
            );
        }

        let trimmed = if do_true {
            line.trim_start_matches("If true: throw to monkey ")
        } else {
            line.trim_start_matches("If false: throw to monkey ")
        };

        let v = trimmed
            .trim()
            .parse::<usize>()
            .map_err(|_| "d11: could not parse destination number.")?;

        Ok(v)
    };

    let mut expected_next_monkey_number = 0;

    let mut line_iter = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.chars().count() != 0);

    let mut result = Vec::new();

    loop {
        let line = line_iter.next();
        if line.is_none() {
            return Ok(result);
        }
        let line = line.unwrap();

        check_monkey_number(line, expected_next_monkey_number)?;

        let line = line_iter.next().ok_or(format!(
            "d11: not enough input lines for Monkey {}",
            expected_next_monkey_number
        ))?;
        let items = parse_items(line)?;

        let line = line_iter.next().ok_or(format!(
            "d11: not enough input lines for Monkey {}",
            expected_next_monkey_number
        ))?;
        let operation = parse_operation(line)?;

        let line = line_iter.next().ok_or(format!(
            "d11: not enough input lines for Monkey {}",
            expected_next_monkey_number
        ))?;
        let divisor_for_test = parse_test(line)?;

        let line = line_iter.next().ok_or(format!(
            "d11: not enough input lines for Monkey {}",
            expected_next_monkey_number
        ))?;
        let true_destination = parse_destination(line, true)?;

        let line = line_iter.next().ok_or(format!(
            "d11: not enough input lines for Monkey {}",
            expected_next_monkey_number
        ))?;
        let false_destination = parse_destination(line, false)?;

        expected_next_monkey_number += 1;

        result.push(Monkey {
            n_turns: 0,
            items,
            operation,
            divisor_for_test,
            true_destination,
            false_destination,
        })
    }
}
