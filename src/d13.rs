use crate::string_error::StringError;

pub fn solve(input: &str) -> Result<String, StringError> {
    let input = input.trim();

    let mut in_right_order = 0;

    let mut all_packets = Vec::new();

    for (i, (l1, l2)) in twinlines(input).enumerate() {
        let p1 = Packet::from_str(l1)?;
        let p2 = Packet::from_str(l2)?;

        if p1 < p2 {
            in_right_order += i + 1;
        }

        all_packets.push(p1);
        all_packets.push(p2);
    }

    let divider2 = Packet::from_str("[[2]]")?;
    let divider6 = Packet::from_str("[[6]]")?;

    all_packets.push(divider2.clone());
    all_packets.push(divider6.clone());
    all_packets.sort();

    let mut index2 = 0;
    let mut index6 = 0;

    for (i, p) in all_packets.iter().enumerate() {
        if *p == divider2 {
            index2 = i + 1;
        } else if *p == divider6 {
            index6 = i + 1;
        }
    }

    let mut result = format!("Day 13/01: {}\n", in_right_order);
    result += &format!("Day 13/02: {}\n", index2 * index6);
    Ok(result)
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Scalar(i64),
    List(Vec<Packet>),
}

impl Packet {
    fn cmp_func(&self, rhs: &Self) -> std::cmp::Ordering {
        match (self, rhs) {
            (Packet::Scalar(p), Packet::Scalar(q)) => p.cmp(q),
            (Packet::Scalar(_), Packet::List(_)) => {
                let k = self.sclar_to_list();
                k.cmp(rhs)
            }
            (Packet::List(_), Packet::Scalar(_)) => {
                let k = rhs.sclar_to_list();
                self.cmp(&k)
            }
            (Packet::List(p), Packet::List(q)) => {
                for (v, w) in p.iter().zip(q.iter()) {
                    let c = v.cmp(w);
                    if c.is_eq() {
                        continue;
                    } else {
                        return c;
                    }
                }

                p.len().cmp(&q.len())
            }
        }
    }

    fn sclar_to_list(&self) -> Self {
        assert!(self.is_scalar());
        let p = self.unwrap_scalar();
        let packet = Self::Scalar(p);
        Self::List(vec![packet])
    }

    fn unwrap_scalar(&self) -> i64 {
        match self {
            Self::Scalar(p) => *p,
            _ => panic!("Cannot unwrap list."),
        }
    }

    fn is_scalar(&self) -> bool {
        match self {
            Self::Scalar(_) => true,
            Self::List(_) => false,
        }
    }

    fn push(&mut self, p: Packet) {
        match self {
            Self::List(l) => l.push(p),
            _ => panic!("Cannot push into scalar."),
        }
    }

    fn from_str(s: &str) -> Result<Self, StringError> {
        let mut chars = s.chars();
        Self::from_chars(&mut chars)
    }

    fn from_chars(chars: &mut std::str::Chars) -> Result<Self, StringError> {
        let mut result = Packet::List(Vec::new());
        let mut current_val: i64 = 0;
        let mut positive = true;
        let mut reading_number = false;

        loop {
            let c = chars.next();
            if c.is_none() {
                return Ok(result);
            }
            let c = c.unwrap();
            if c.is_whitespace() && reading_number {
                return Err("d13: Invalid input (whitespace inside number).".into());
            }
            if c.is_whitespace() {
                continue;
            }
            match c {
                '-' => {
                    if reading_number {
                        return Err("d13: Invalid input. Minus sign inside of number".into());
                    }
                    positive = false;
                    reading_number = true;
                }
                '0'..='9' => {
                    current_val *= 10;
                    current_val += (c as u8 - b'0') as i64;
                    reading_number = true;
                }
                ',' => {
                    if !positive {
                        current_val = -current_val;
                    }

                    if reading_number {
                        result.push(Self::Scalar(current_val));
                    }
                    current_val = 0;
                    positive = true;
                    reading_number = false;
                }
                '[' => result.push(Self::from_chars(chars)?),
                ']' => {
                    if !positive {
                        current_val = -current_val;
                    }
                    if reading_number {
                        result.push(Self::Scalar(current_val));
                    }

                    return Ok(result);
                }
                _ => return Err("d13: Invalid input char.".into()),
            }
        }
    }
}

impl std::cmp::PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp_func(other))
    }
}

impl std::cmp::Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cmp_func(other)
    }
}

fn twinlines(s: &str) -> TwinlineIter {
    TwinlineIter {
        lines: s.lines(),
        prev: None,
    }
}

struct TwinlineIter<'a> {
    lines: std::str::Lines<'a>,
    prev: Option<&'a str>,
}

impl<'a> Iterator for TwinlineIter<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = self.lines.next()?;

            if line.trim().chars().take(1).count() == 0 {
                continue;
            }

            if self.prev.is_none() {
                self.prev = Some(line);
                continue;
            }

            let p = self.prev.unwrap();
            self.prev = None;

            return Some((p, line));
        }
    }
}
