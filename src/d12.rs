use std::collections::{BTreeSet, VecDeque};

use crate::string_error::StringError;

pub fn solve(input: &str) -> Result<String, StringError> {
    let input = input.trim();

    let (startindex, endindex, height_map) = HeightMap::from_input(input)?;

    let steps = height_map.count_steps(startindex, endindex);
    // let steps2 = height_map
    //     .data
    //     .iter()
    //     .enumerate()
    //     .filter(|(_, h)| **h == 0)
    //     .map(|(i, _)| height_map.count_steps(i, endindex))
    //     .filter(|s| s.is_some())
    //     .map(|s| s.unwrap())
    //     .min();

    let steps2 = height_map.find_shortest_route_to_zero(endindex);

    let mut result = String::new();
    if let Some(s) = steps {
        result += &format!("Day 12/01: {}\n", s);
    } else {
        result += "Day 12/01: Did not find route.\n"
    }
    if let Some(s) = steps2 {
        result += &format!("Day 12/02: {}\n", s);
    } else {
        result += "Day 12/02: Did not find route.\n"
    }

    Ok(result)
}

#[derive(Debug)]
struct HeightMap {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl HeightMap {
    fn count_steps(&self, startindex: usize, endindex: usize) -> Option<usize> {
        let sorted_insert =
            |index: usize,
             distance_from_start: usize,
             estimate_to_end: usize,
             todo: &mut VecDeque<(usize, usize, usize)>| {
                todo.push_front((index, distance_from_start, estimate_to_end));

                for i in 0..(todo.len() - 1) {
                    if todo[i].1 + todo[i].2 > todo[i + 1].1 + todo[i + 1].2 {
                        todo.swap(i, i + 1);
                    } else {
                        break;
                    }
                }
            };

        let manhatten = |a: usize, b: usize, width: usize| -> usize {
            let ia = (a / width) as i64;
            let ja = (a % width) as i64;
            let ib = (b / width) as i64;
            let jb = (b % width) as i64;
            ((ia - ib).abs() + (ja - jb).abs()) as usize
        };

        let mut distances_from_start = vec![None; self.width * self.height];
        distances_from_start[startindex] = Some(0);

        let mut todo = VecDeque::new();

        todo.push_back((startindex, 0, manhatten(startindex, endindex, self.width)));

        while !todo.is_empty() {
            let (index, distance_from_start, _) = todo.pop_front().unwrap();

            if index == endindex {
                return Some(distance_from_start);
            }

            if let Some(d) = distances_from_start[index] {
                if d < distance_from_start {
                    continue;
                }
            }

            let i = index / self.width;
            let j = index % self.width;

            let new_distance = distance_from_start + 1;

            for (ni, nj) in neighbors(i, j, self.height, self.width) {
                let neighbor_index = ni * self.width + nj;

                let mut do_push = false;

                if self.data[neighbor_index] > self.data[index] + 1 {
                    continue;
                }

                if let Some(d) = distances_from_start[neighbor_index] {
                    if d > new_distance {
                        do_push = true;
                    }
                } else {
                    do_push = true;
                }

                if do_push {
                    distances_from_start[neighbor_index] = Some(new_distance);
                    sorted_insert(
                        neighbor_index,
                        new_distance,
                        manhatten(index, endindex, self.width),
                        &mut todo,
                    );
                }
            }
        }

        None
    }

    fn find_shortest_route_to_zero(&self, endindex: usize) -> Option<usize> {
        let mut todo = VecDeque::new();
        let mut visited = BTreeSet::new();

        todo.push_back((endindex, 0));

        while !todo.is_empty() {
            let (index, distance_from_end) = todo.pop_front().unwrap();

            if self.data[index] == 0 {
                return Some(distance_from_end);
            }

            let i = index / self.width;
            let j = index % self.width;

            let new_distance = distance_from_end + 1;

            for (ni, nj) in neighbors(i, j, self.height, self.width) {
                let neighbor_index = ni * self.width + nj;
                if visited.contains(&neighbor_index) {
                    continue;
                }

                if self.data[index] > self.data[neighbor_index] + 1 {
                    continue;
                }

                todo.push_back((neighbor_index, new_distance));
                visited.insert(neighbor_index);
            }
        }

        None
    }

    fn from_input(input: &str) -> Result<(usize, usize, Self), StringError> {
        let mut width = None;
        let mut height = 0;

        let mut start = None;
        let mut end = None;

        let mut data = Vec::new();
        let mut foundchars = false;

        for (i, line) in input.lines().map(|l| l.trim()).enumerate() {
            height = i + 1;
            let mut local_width = 0;
            for (j, c) in line.chars().enumerate() {
                foundchars = true;
                local_width = j + 1;
                match c {
                    'a'..='z' => data.push((c as u8) - b'a'),
                    'S' => {
                        if start.is_some() {
                            return Err("d12: multiple starts.".into());
                        }
                        start = Some(i * width.unwrap_or(0) + j);
                        data.push(0);
                    }
                    'E' => {
                        if end.is_some() {
                            return Err("d12: multiple ends.".into());
                        }
                        end = Some(i * width.unwrap_or(0) + j);
                        data.push(b'z' - b'a');
                    }
                    _ => return Err("d12: invalid inputs.".into()),
                }
            }
            if width.is_none() {
                width = Some(local_width);
            }

            if width.unwrap() != local_width {
                return Err("d12: input must be a square.".into());
            }
        }

        if !foundchars {
            return Err("d12: received no input.".into());
        }

        if start.is_none() || end.is_none() {
            return Err("d12: Did not find S and E.".into());
        }

        Ok((
            start.unwrap(),
            end.unwrap(),
            Self {
                width: width.unwrap(),
                height,
                data,
            },
        ))
    }
}

fn neighbors(i: usize, j: usize, height: usize, width: usize) -> NeighborIter {
    NeighborIter {
        i,
        j,
        width,
        height,
        dir: 0,
    }
}

struct NeighborIter {
    i: usize,
    j: usize,
    width: usize,
    height: usize,
    dir: u8,
}

impl Iterator for NeighborIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.dir {
            0 => {
                if self.j + 1 < self.width {
                    self.dir += 1;
                    Some((self.i, self.j + 1))
                } else {
                    self.dir += 1;
                    self.next()
                }
            }
            1 => {
                if self.i > 0 {
                    self.dir += 1;
                    Some((self.i - 1, self.j))
                } else {
                    self.dir += 1;
                    self.next()
                }
            }
            2 => {
                if self.j > 0 {
                    self.dir += 1;
                    Some((self.i, self.j - 1))
                } else {
                    self.dir += 1;
                    self.next()
                }
            }
            3 => {
                if self.i + 1 < self.height {
                    self.dir += 1;
                    Some((self.i + 1, self.j))
                } else {
                    self.dir += 1;
                    self.next()
                }
            }
            _ => None,
        }
    }
}
