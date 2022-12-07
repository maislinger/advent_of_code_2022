use crate::string_error::StringError;

pub fn solve(input: &str) -> Result<String, StringError> {
    let input = input.trim();

    if !input.starts_with("$ cd /") {
        return Err("d07: Must start at root.".into());
    }

    let mut entries = vec![LsEntry::new(None, 0, "/", true)];
    let mut current_parent = 0;
    let mut ls_mode = false;

    for line in input.lines() {
        let line = line.trim();

        if line.starts_with('$') && ls_mode {
            entries[current_parent].already_lsed = true;
        }

        if line == "$ cd /" {
            current_parent = 0;
        } else if line == "$ ls" && entries[current_parent].already_lsed {
            continue;
        } else if line == "$ ls" && !entries[current_parent].already_lsed {
            ls_mode = true;
        } else if line == "$ cd .." {
            current_parent = entries[current_parent]
                .parent_index
                .ok_or("d07: cannot move one up from root")?;
        } else if line.starts_with("$ cd") {
            let mut target_iter = line.chars();
            for _ in 0..5 {
                target_iter.next();
            }
            let target = target_iter.as_str();
            current_parent = find_by_name(&entries, current_parent, target)
                .ok_or("d06: could not move to dir.")?;
        } else if ls_mode {
            let mut splitted = line.split_whitespace();
            let first = splitted
                .next()
                .ok_or("d07: listings must have two columns")?;
            let second = splitted
                .next()
                .ok_or("d07: listings must have two columns")?;

            if first == "dir" {
                entries.push(LsEntry::new(Some(current_parent), 0, second, true));
                continue;
            }

            let file_size = first.parse::<usize>().map_err(|_| "d07: invalid input.")?;
            entries.push(LsEntry::new(Some(current_parent), file_size, second, false));
        }
    }

    // Compute directory sizes.
    for i in (0..entries.len()).rev() {
        let p = entries[i].parent_index;
        if p.is_none() {
            continue;
        }

        let p = p.unwrap();
        entries[p].size += entries[i].size;
    }

    let s1: usize = entries
        .iter()
        .filter(|e| e.is_dir)
        .filter(|e| e.size <= 100000)
        .map(|e| e.size)
        .sum();

    let used_space = entries[0].size;
    let capacity = 70000000;
    if used_space > capacity {
        return Err(
            "d07: Something went wrong. You are using more space than there is capacity.".into(),
        );
    }
    let free_space = capacity - used_space;
    let needed_space = 30000000;
    if free_space >= needed_space {
        return Err(
            "d07: Something went wrong. You already have more free space than needed".into(),
        );
    }
    let min_space_to_free = needed_space - free_space;

    let s2: usize = entries
        .iter()
        .filter(|e| e.is_dir)
        .filter(|e| e.size >= min_space_to_free)
        .map(|e| e.size)
        .min()
        .ok_or("d07: Could not find a directory to delete.")?;

    let mut result = format!("Day 07/01: {}\n", s1);
    result += &format!("Day 07/02: {}\n", s2);
    Ok(result)
}

#[derive(Debug)]
struct LsEntry<'a> {
    parent_index: Option<usize>,
    size: usize,
    name: &'a str,
    is_dir: bool,
    already_lsed: bool,
}

impl<'a> LsEntry<'a> {
    fn new(parent_index: Option<usize>, size: usize, name: &'a str, is_dir: bool) -> Self {
        Self {
            parent_index,
            size,
            name,
            is_dir,
            already_lsed: false,
        }
    }
}

fn find_by_name(entries: &[LsEntry], from_index: usize, target: &str) -> Option<usize> {
    if from_index >= entries.len() {
        return None;
    }

    for (i, e) in entries.iter().enumerate().skip(from_index) {
        if e.name == target && e.parent_index == Some(from_index) {
            return Some(i);
        }
    }

    None
}
