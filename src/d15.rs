use std::str::FromStr;

use crate::string_error::StringError;

pub fn solve(input: &str) -> Result<String, StringError> {
    let input = input.trim();

    let mut beacons = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.chars().take(1).count() == 0 {
            continue;
        }

        let beacon = Beacon::from_str(line)?;
        beacons.push(beacon);
    }

    let row_part1 = 2_000_000;
    // Find at least on impossible lattice point.
    let mut x_init_part1 = None;

    for b in beacons.iter() {
        let dy = (b.position.y - row_part1).abs();
        let d = b.position.manhatten_distance(&b.closest_beacon);
        if d >= dy {
            x_init_part1 = Some(b.position.x);
            break;
        }
    }

    let mut solution1: i64 = 0;
    if let Some(x) = x_init_part1 {
        let p = Point::new(x, row_part1);
        let p_right = step_right_if_overlap_slice(p, &beacons);
        let p_left = step_left_if_overlap_slice(p, &beacons);

        solution1 = p_right.x - p_left.x - 2;

        for b in beacons.iter() {
            let pb = b.closest_beacon;
            if pb.y != row_part1 {
                continue;
            }
        }
    }

    let mut solution2 = None;
    let xy_max = 4_000_000;
    for xy in 0..=xy_max {
        let p = Point::new(xy, xy);
        let p_right = step_right_if_overlap_slice(p, &beacons);
        let p_up = step_up_if_overlap_slice(p, &beacons);

        if solution2.is_none() && p_right.x <= xy_max {
            solution2 = Some(p_right);
            break;
        }
        if solution2.is_none() && p_up.y <= xy_max {
            solution2 = Some(p_up);
            break;
        }
    }
    let solution2 = solution2.ok_or("d12: did not find solution for part 2")?;

    let mut result = format!("Day 15/01: {}\n", solution1);
    result += &format!("Day 15/02: {}\n", solution2.x * 4_000_000 + solution2.y);
    Ok(result)
}

fn step_left_if_overlap(p: &mut Point, beacon: &Beacon) -> bool {
    let d = beacon.position.manhatten_distance(&beacon.closest_beacon);
    let d2 = p.manhatten_distance(&beacon.position);

    if d2 > d {
        return false;
    }

    let y = p.y;
    let diff_y = (p.y - beacon.position.y).abs();
    let x = beacon.position.x - d - 1 + diff_y;
    *p = Point::new(x, y);

    true
}

fn step_left_if_overlap_slice(mut p: Point, beacons: &[Beacon]) -> Point {
    let mut change = true;
    while change {
        change = false;
        for b in beacons {
            change = change || step_left_if_overlap(&mut p, b);
        }
    }

    p
}

fn step_right_if_overlap(p: &mut Point, beacon: &Beacon) -> bool {
    let d = beacon.position.manhatten_distance(&beacon.closest_beacon);
    let d2 = p.manhatten_distance(&beacon.position);

    if d2 > d {
        return false;
    }

    let y = p.y;
    let diff_y = (p.y - beacon.position.y).abs();
    let x = beacon.position.x + d + 1 - diff_y;
    *p = Point::new(x, y);

    true
}

fn step_right_if_overlap_slice(mut p: Point, beacons: &[Beacon]) -> Point {
    let mut change = true;
    while change {
        change = false;
        for b in beacons {
            change = change || step_right_if_overlap(&mut p, b);
        }
    }

    p
}

fn step_up_if_overlap(p: &mut Point, beacon: &Beacon) -> bool {
    let d = beacon.position.manhatten_distance(&beacon.closest_beacon);
    let d2 = p.manhatten_distance(&beacon.position);

    if d2 > d {
        return false;
    }

    let x = p.x;
    let diff_x = (p.x - beacon.position.x).abs();
    let y = beacon.position.y + d + 1 - diff_x;
    *p = Point::new(x, y);

    true
}

fn step_up_if_overlap_slice(mut p: Point, beacons: &[Beacon]) -> Point {
    let mut change = true;
    while change {
        change = false;
        for b in beacons {
            change = change || step_up_if_overlap(&mut p, b);
        }
    }

    p
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn manhatten_distance(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug)]
struct Beacon {
    position: Point,
    closest_beacon: Point,
}

impl Beacon {
    // fn get_impossible_row(&self, row: i64) -> BTreeSet<i64> {
    //     let mut result = BTreeSet::new();

    //     let closest_beacond_distance = self.position.manhatten_distance(&self.closest_beacon);

    //     let mut x = self.position.x;
    //     let mut distance = (row - self.position.y).abs();

    //     if distance > closest_beacond_distance {
    //         return result;
    //     }

    //     while distance <= closest_beacond_distance {
    //         if row != self.closest_beacon.y || x != self.closest_beacon.x {
    //             result.insert(x);
    //         }

    //         x += 1;
    //         distance += 1;
    //     }

    //     x = self.position.x;
    //     distance = (row - self.position.y).abs();
    //     while distance <= closest_beacond_distance {
    //         if row != self.closest_beacon.y || x != self.closest_beacon.x {
    //             result.insert(x);
    //         }

    //         x -= 1;
    //         distance += 1;
    //     }

    //     result
    // }

    fn from_str(s: &str) -> Result<Self, StringError> {
        let mut parser = Parser::from_str(s);
        parser.trim_whitespace_front();
        parser.trim_str_front("Sensor at x=")?;

        let x0 = parser.parse_number::<i64>()?;

        parser.trim_str_front(",")?;
        parser.trim_whitespace_front();
        parser.trim_str_front("y=")?;

        let y0 = parser.parse_number::<i64>()?;

        parser.trim_str_front(": closest beacon is at x=")?;

        let x1 = parser.parse_number::<i64>()?;

        parser.trim_str_front(",")?;
        parser.trim_whitespace_front();

        parser.trim_str_front("y=")?;

        let y1 = parser.parse_number::<i64>()?;

        Ok(Self {
            position: Point::new(x0, y0),
            closest_beacon: Point::new(x1, y1),
        })
    }
}

struct Parser<'a> {
    chars: std::str::Chars<'a>,
}

impl<'a> Parser<'a> {
    fn from_str(s: &'a str) -> Self {
        Self { chars: s.chars() }
    }

    fn pop_chars_front(&mut self, n: usize) {
        for _ in 0..n {
            self.chars.next();
        }
    }

    fn trim_str_front(&mut self, s: &str) -> Result<(), StringError> {
        let mut tmp = self.chars.clone();
        let mut counter = 0;

        for c in s.chars() {
            let d = tmp.next();
            if d.is_none() {
                return Err("Tried to trim more chars than there are left in the parser.".into());
            }
            let d = d.unwrap();
            if c != d {
                return Err(
                    "Tried to peele something of from the parser that was not there.".into(),
                );
            }

            counter += 1;
        }

        self.pop_chars_front(counter);

        Ok(())
    }

    fn trim_whitespace_front(&mut self) {
        let tmp = self.chars.clone();

        let mut counter = 0;
        for c in tmp {
            if c.is_whitespace() {
                counter += 1;
            } else {
                break;
            }
        }

        self.pop_chars_front(counter);
    }

    fn parse_number<T>(&mut self) -> Result<T, StringError>
    where
        T: FromStr,
    {
        let tmp = self.chars.clone();

        let mut n_bytes = 0;
        let mut count = 0;
        for c in tmp {
            if c.is_ascii_digit() || c == '-' {
                n_bytes += c.len_utf8();
                count += 1;
            } else {
                break;
            }
        }

        let s = &self.chars.as_str()[..n_bytes];
        self.pop_chars_front(count);
        s.parse::<T>().map_err(|_| "d15: parse error.".into())
    }
}
