use crate::string_error::StringError;

pub fn solve(input: &str) -> Result<String, StringError> {
    let mut sum_priority = 0;
    let mut sum_badge_priority = 0;
    let mut occuring_items = [true; 52];

    for (i, line) in input.lines().enumerate() {
        let line_counts = compute_counts(line)?;
        sum_priority += compute_priority_from_counts(&line_counts);
        items_update(&mut occuring_items, &line_counts);

        if i % 3 == 2 {
            sum_badge_priority += compute_priority_from_items(&occuring_items);
            items_reset(&mut occuring_items);
        }
    }

    let mut result = format!("Day 03/01: {}\n", sum_priority);
    result += &format!("Day 03/02: {}\n", sum_badge_priority);
    Ok(result)
}

fn compute_counts(rucksack: &str) -> Result<[[u64; 2]; 52], StringError> {
    // [a (left), a (right)] [b (left), b (right)] ... [Z (left), Z (right)]
    let mut counts = [[0u64; 2]; 52];

    let total_item_cout = rucksack.chars().count();
    if total_item_cout % 2 != 0 {
        return Err("d03: at least on rucksack does not contain an even number of items.".into());
    }

    for (i, c) in rucksack.chars().enumerate() {
        let side_index = 2 * i / total_item_cout;
        let item_index = match c {
            'a'..='z' => Ok((c as usize) - ('a' as usize)),
            'A'..='Z' => Ok((c as usize) - ('A' as usize) + 26),
            _ => Err("d03: illegal item."),
        }?;

        counts[item_index][side_index] += 1;
    }

    Ok(counts)
}

fn compute_priority_from_counts(counts: &[[u64; 2]; 52]) -> u64 {
    let mut r = 0;

    for (i, c) in counts.iter().enumerate() {
        if c[0] > 0 && c[1] > 0 {
            r += (i + 1) as u64;
        }
    }

    r
}

fn items_update(items: &mut [bool; 52], counts: &[[u64; 2]; 52]) {
    for (b, count) in items.iter_mut().zip(counts.iter()) {
        if count[0] == 0 && count[1] == 0 {
            *b = false;
        }
    }
}

fn items_reset(items: &mut [bool; 52]) {
    *items = [true; 52];
}

fn compute_priority_from_items(items: &[bool; 52]) -> u64 {
    let mut r = 0;

    for (i, b) in items.iter().enumerate() {
        if *b {
            r += (i + 1) as u64;
        }
    }

    r
}
