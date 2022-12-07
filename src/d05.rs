use crate::string_error::StringError;

pub fn solve(input: &str) -> Result<String, StringError> {
    let mut towers9000 = Towers::from_input(input)?;
    let mut towers9001 = Towers::from_input(input)?;
    towers9001.is_9000 = false;

    for line in input.lines().map(|l| l.trim()) {
        if !line.starts_with("move") {
            continue;
        }

        let (n, from, to) = parse_command(line)?;

        towers9000.move_crates(from, to, n)?;
        towers9001.move_crates(from, to, n)?;
    }

    let mut result = format!("Day 05/01: {}\n", towers9000.get_topword());
    result += &format!("Day 05/02: {}\n", towers9001.get_topword());
    Ok(result)
}

const TOWER_HEIGHT: usize = 128;
const TOWER_WIDTH: usize = 9;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum CrateCell {
    Empty,
    Filled(u8),
}

impl CrateCell {
    fn to_char(self) -> char {
        match self {
            CrateCell::Empty => ' ',
            CrateCell::Filled(n) => (n + b'A') as char,
        }
    }
}

#[derive(Debug)]
struct Towers {
    cells: [CrateCell; TOWER_HEIGHT * TOWER_WIDTH],
    heights: [usize; TOWER_WIDTH],
    is_9000: bool,
}

impl Default for Towers {
    fn default() -> Self {
        Self {
            cells: [CrateCell::Empty; TOWER_HEIGHT * TOWER_WIDTH],
            heights: [0; TOWER_WIDTH],
            is_9000: true,
        }
    }
}

impl Towers {
    fn from_input(input: &str) -> Result<Self, StringError> {
        let mut result = Self::default();

        for line in input.lines() {
            if line.trim().chars().count() == 0 {
                break;
            }
            if line.chars().any(|c| c == '1') {
                break;
            }

            for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
                match c {
                    'A'..='Z' => {
                        let cll = CrateCell::Filled((c as u8) - b'A');
                        result.insert_crate_from_below(i, cll)?;
                    }
                    ' ' => result.insert_crate_from_below(i, CrateCell::Empty)?,
                    _ => return Err("d05: invalid char in input.".into()),
                }
            }
        }

        Ok(result)
    }

    fn move_crates(
        &mut self,
        from_col: usize,
        to_col: usize,
        n_crates: usize,
    ) -> Result<(), StringError> {
        if self.heights[from_col] < n_crates {
            return Err("d05: tried to move too many crates.".into());
        }
        if self.heights[to_col] + n_crates > TOWER_HEIGHT {
            return Err("d05: TOWER_HEIGHT too small.".into());
        }

        let mut i = if self.is_9000 {
            self.heights[from_col] - 1
        } else {
            self.heights[from_col] - n_crates
        };
        let mut j = self.heights[to_col];

        for _ in 0..n_crates {
            self.cells[j * TOWER_WIDTH + to_col] = self.cells[i * TOWER_WIDTH + from_col];
            self.cells[i * TOWER_WIDTH + from_col] = CrateCell::Empty;
            j += 1;

            if self.is_9000 && i > 0 {
                i -= 1;
            } else if !self.is_9000 {
                i += 1;
            }
        }

        self.heights[from_col] -= n_crates;
        self.heights[to_col] += n_crates;

        Ok(())
    }

    fn insert_crate_from_below(&mut self, col: usize, mut c: CrateCell) -> Result<(), StringError> {
        let mut index = col;

        if c == CrateCell::Empty {
            return Ok(());
        }

        while c != CrateCell::Empty {
            if index >= TOWER_HEIGHT * TOWER_WIDTH {
                return Err("d05: TOWER_HEIGHT too small.".into());
            }

            std::mem::swap(&mut self.cells[index], &mut c);
            index += TOWER_WIDTH;
        }

        self.heights[col] += 1;

        Ok(())
    }

    fn get_topword(&self) -> String {
        let mut result = String::new();
        for (i, h) in self.heights.iter().enumerate() {
            if *h == 0 {
                result += " ";
                continue;
            }

            let c = self.cells[(h - 1) * TOWER_WIDTH + i].to_char();
            result.push(c);
        }
        result
    }
}

fn parse_command(line: &str) -> Result<(usize, usize, usize), StringError> {
    let mut split_iter = line.split_whitespace();

    let err_msg = "d05: invalid input move.";

    let a1 = split_iter.next().ok_or(err_msg)?;
    let a2 = split_iter.next().ok_or(err_msg)?;
    let a3 = split_iter.next().ok_or(err_msg)?;
    let a4 = split_iter.next().ok_or(err_msg)?;
    let a5 = split_iter.next().ok_or(err_msg)?;
    let a6 = split_iter.next().ok_or(err_msg)?;

    if a1 != "move" || a3 != "from" || a5 != "to" {
        return Err(err_msg.into());
    }

    let n = a2.parse::<usize>().map_err(|_| err_msg)?;
    let from = a4.parse::<usize>().map_err(|_| err_msg)?;
    let to = a6.parse::<usize>().map_err(|_| err_msg)?;

    if from < 1 || to < 1 {
        return Err(err_msg.into());
    }

    Ok((n, from - 1, to - 1))
}
