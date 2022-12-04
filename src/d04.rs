use crate::string_error::StringError;

pub fn solve(input: &str) -> Result<String, StringError> {
    let mut count_fully_contained: u64 = 0;
    let mut count_partially_contained: u64 = 0;

    for line in input.lines() {
        if line.trim().chars().count() == 0 {
            continue;
        }
        let sections = parse_section_ranges(line)?;

        if sections[0].fully_contained_within(&sections[1])
            || sections[1].fully_contained_within(&sections[0])
        {
            count_fully_contained += 1;
            count_partially_contained += 1;
        } else if sections[0].overlaps_with(&sections[1]) {
            count_partially_contained += 1;
        }
    }

    let mut result = format!("Day 04/01: {}\n", count_fully_contained);
    result += &format!("Day 04/02: {}\n", count_partially_contained);
    Ok(result)
}

#[derive(Debug)]
struct Section {
    lower: i64,
    upper: i64,
}

impl Section {
    fn new(lower: i64, upper: i64) -> Self {
        Section { lower, upper }
    }

    fn fully_contained_within(&self, other: &Self) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }

    fn overlaps_with(&self, other: &Self) -> bool {
        let lower_between = other.lower <= self.lower && self.lower <= other.upper;
        let upper_between = other.lower <= self.upper && self.upper <= other.upper;
        lower_between || upper_between
    }
}

fn parse_section_ranges(s: &str) -> Result<[Section; 2], StringError> {
    let mut numbers = [0; 4];
    let mut current = 0;
    let mut index = 0;
    let mut read = false;

    for c in s.chars() {
        match c {
            '0'..='9' => {
                let v = (c as i64) - '0' as i64;
                current *= 10;
                current += v;
                read = true;
            }
            '-' | ',' => {
                if index >= 4 {
                    return Err("d04: too many input numbers".into());
                }
                numbers[index] = current;
                current = 0;
                index += 1;
                read = false;
            }
            _ => {
                if !c.is_whitespace() {
                    return Err("d04: invalid input char".into());
                }
            }
        }
    }

    if index != 3 || !read {
        return Err("d04: too few input numbers".into());
    } else {
        numbers[index] = current;
    }

    Ok([
        Section::new(numbers[0], numbers[1]),
        Section::new(numbers[2], numbers[3]),
    ])
}
