use std::collections::BTreeSet;

use crate::string_error::StringError;

pub fn solve(input: &str) -> Result<String, StringError> {
    let input = input.trim();

    let mut cavemap = BTreeSet::new();
    let mut max_y = None;

    for (a, b) in iter_points(input) {
        for c in pointline(a, b) {
            cavemap.insert(c);
            if max_y.is_none() || max_y.unwrap() < c.y {
                max_y = Some(c.y);
            }
        }
    }
    let max_y = max_y.unwrap();

    let n_rocks = cavemap.len();

    let init_point = Point::new(500, 0);
    let mut sandpath = vec![init_point];

    let mut solution_one = None;
    let solution_two;

    loop {
        let p = *sandpath.last().unwrap();

        if solution_one.is_some() && p == init_point && cavemap.contains(&p) {
            solution_two = cavemap.len() - n_rocks;
            break;
        }

        if cavemap.contains(&p) {
            sandpath.pop();
            continue;
        }

        if solution_one.is_none() && p.y >= max_y {
            solution_one = Some(cavemap.len() - n_rocks);
        }

        // Bottom of second part.
        if p.y == max_y + 1 {
            cavemap.insert(p);
            sandpath.pop();
            continue;
        }

        let mut pc = p;
        pc.y += 1;

        if !cavemap.contains(&pc) {
            sandpath.push(pc);
            continue;
        }

        pc.x -= 1;
        if !cavemap.contains(&pc) {
            sandpath.push(pc);
            continue;
        }

        pc.x += 2;
        if !cavemap.contains(&pc) {
            sandpath.push(pc);
            continue;
        }

        cavemap.insert(p);
        if p != init_point {
            sandpath.pop();
        }
    }

    let mut result = format!("Day 14/01: {}\n", solution_one.unwrap());
    result += &format!("Day 14/02: {}\n", solution_two);
    Ok(result)
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

    fn from_str(s: &str) -> Self {
        let mut k = s.split(',');
        let first = k
            .next()
            .expect("d12: failed to get first coordinate for point.")
            .trim();
        let second = k
            .next()
            .expect("d12: failed to get second coordinate for point.")
            .trim();

        let x = first.parse().expect("d12: failed to parse x for Point");
        let y = second.parse().expect("d12: failed to parse y for Point");
        Self::new(x, y)
    }
}

struct StrPointIter<'a> {
    lines: std::str::Lines<'a>,
    current_line_iter: std::str::Split<'a, &'a str>,
    previous: Option<Point>,
}

impl<'a> Iterator for StrPointIter<'a> {
    type Item = (Point, Point);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut s = self.current_line_iter.next();
            if s.is_none() {
                self.previous = None;
                let next_line = self.lines.next()?;
                self.current_line_iter = next_line.split(" -> ");
                s = self.current_line_iter.next();
            }
            let s = s?;

            if self.previous.is_none() {
                self.previous = Some(Point::from_str(s));
                continue;
            }

            let a = self.previous.unwrap();
            let b = Point::from_str(s);
            self.previous = Some(b);

            return Some((a, b));
        }
    }
}

fn iter_points(input: &str) -> StrPointIter {
    let mut lines = input.lines();
    let first_line = lines.next();

    if first_line.is_none() {
        let tmp = "";
        let mut current_line_iter = tmp.split(" -> ");
        current_line_iter.next();
        return StrPointIter {
            lines,
            current_line_iter,
            previous: None,
        };
    }

    let first_line = first_line.unwrap();
    let current_line_iter = first_line.split(" -> ");
    StrPointIter {
        lines,
        current_line_iter,
        previous: None,
    }
}

struct LineIter {
    current: Point,
    end: Point,
    first: bool,
    stopped: bool,
}

impl Iterator for LineIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stopped {
            return None;
        }

        if self.first {
            self.first = false;
            return Some(self.current);
        }

        let dx = (self.current.x - self.end.x).signum();
        let dy = (self.current.y - self.end.y).signum();

        self.current.x -= dx;
        self.current.y -= dy;

        if self.current == self.end {
            self.stopped = true;
        }

        Some(self.current)
    }
}

fn pointline(a: Point, b: Point) -> LineIter {
    LineIter {
        current: a,
        end: b,
        first: true,
        stopped: false,
    }
}
