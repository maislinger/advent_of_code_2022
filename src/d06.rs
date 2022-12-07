use crate::string_error::StringError;

pub fn solve(input: &str) -> Result<String, StringError> {
    let input = input.trim();

    for c in input.chars() {
        if c.is_whitespace() {
            continue;
        }
        match c {
            'a'..='z' => (),
            _ => return Err("d06: Invalid input.".into()),
        }
    }

    let sol1 = count_with_offset(input, 4);
    let sol2 = count_with_offset(input, 14);

    let mut result = format!("Day 06/01: {}\n", sol1);
    result += &format!("Day 06/02: {}\n", sol2);
    Ok(result)
}

fn count_with_offset(input: &str, offset: usize) -> usize {
    let mut counts = [0u16; 26];

    let check = |counts: &[u16; 26]| {
        for c in counts.iter() {
            if *c > 1 {
                return false;
            }
        }
        true
    };

    for c in input.chars().take(offset) {
        let index = (c as usize) - ('a' as usize);
        counts[index] += 1;
    }

    if check(&counts) {
        return offset;
    }

    let mut pos = offset;
    for (c, d) in input.chars().zip(input.chars().skip(offset)) {
        pos += 1;

        let index_c = (c as usize) - ('a' as usize);
        let index_d = (d as usize) - ('a' as usize);

        counts[index_c] -= 1;
        counts[index_d] += 1;

        if counts[index_c] == 1 && counts[index_d] == 1 && check(&counts) {
            break;
        }
    }

    pos
}
