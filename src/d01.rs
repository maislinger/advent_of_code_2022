use crate::string_error::StringError;

pub fn solve(input: &str) -> Result<(), StringError> {
    let mut current_calories: u64 = 0;
    let mut max_calories = [0u64; 3];

    for line in input.lines() {
        if line.trim().chars().count() == 0 {
            insert(&mut max_calories, current_calories);
            current_calories = 0;
            continue;
        }

        let v: u64 = line
            .parse::<u64>()
            .map_err(|_| "Could not parse number in d01")?;
        current_calories += v;
    }

    println!("Day 01/01: {}", max_calories[0]);
    println!("Day 01/02: {}", max_calories.iter().sum::<u64>());

    Ok(())
}

fn insert(max_calories: &mut [u64; 3], new_value: u64) {
    if new_value > max_calories[0] {
        max_calories[2] = max_calories[1];
        max_calories[1] = max_calories[0];
        max_calories[0] = new_value;
    } else if new_value > max_calories[1] {
        max_calories[2] = max_calories[1];
        max_calories[1] = new_value;
    } else if new_value > max_calories[2] {
        max_calories[2] = new_value;
    }
}
