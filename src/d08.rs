use std::ops::{Index, IndexMut};

use crate::string_error::StringError;

pub fn solve(input: &str) -> Result<String, StringError> {
    let input = input.trim();
    let (width, height) = check_input(input)?;

    let mut grid = Grid::new(width, height);
    grid.fill_from_input_unchecked(input);
    grid.check_visibility();

    let mut result = format!(
        "Day 08/01: {}\n",
        grid.trees.iter().filter(|t| t.border_visible).count()
    );
    result += &format!(
        "Day 08/02: {}\n",
        grid.trees.iter().map(|t| t.view_score).max().unwrap()
    );
    Ok(result)
}

#[derive(Copy, Clone)]
struct Tree {
    height: u8,
    view_score: u32,
    border_visible: bool,
}

impl Tree {
    fn update_properties(
        &mut self,
        distance_from_border: usize,
        distances_from_heights: &[Option<usize>; 10],
    ) {
        if distance_from_border == 1 {
            self.view_score = 0;
            self.border_visible = true;
            return;
        }

        let mut blocked_view = false;
        let mut max_view = distance_from_border - 1;
        for h_index in (self.height as usize)..10 {
            if let Some(d) = distances_from_heights[h_index] {
                blocked_view = true;
                if max_view > d {
                    max_view = d;
                }
            }
        }

        if !blocked_view {
            self.border_visible = true;
        }

        self.view_score *= max_view as u32;
    }
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            height: 0,
            view_score: 1,
            border_visible: false,
        }
    }
}

struct Grid {
    trees: Vec<Tree>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            trees: vec![Tree::default(); width * height],
            width,
            height,
        }
    }

    fn fill_from_input_unchecked(&mut self, input: &str) {
        let mut t_iter = self.trees.iter_mut();
        let mut t = t_iter.next();

        for c in input.chars() {
            if let '0'..='9' = c {
                (t.unwrap()).height = c as u8 - b'0';
                t = t_iter.next();
            }
        }
    }

    fn check_visibility(&mut self) {
        for i in 0..self.height {
            self.check_visibility_row(i);
        }

        for j in 0..self.width {
            self.check_visibility_col(j);
        }
    }

    fn check_visibile_step(
        &mut self,
        i: usize,
        j: usize,
        distance: &mut usize,
        distances_from_heights: &mut [Option<usize>; 10],
    ) {
        self[(i, j)].update_properties(*distance, distances_from_heights);

        let h_index = self[(i, j)].height as usize;
        distances_from_heights[h_index] = Some(0);

        *distance += 1;
        distances_from_heights
            .iter_mut()
            .map(|d| *d = d.map(|v| v + 1))
            .count();
    }

    fn check_visibility_row(&mut self, i: usize) {
        let mut distances_from_heights = [None; 10];
        let mut distance = 1;

        for j in 0..self.width {
            self.check_visibile_step(i, j, &mut distance, &mut distances_from_heights);
        }

        distances_from_heights = [None; 10];
        distance = 1;
        for j in (0..self.width).rev() {
            self.check_visibile_step(i, j, &mut distance, &mut distances_from_heights);
        }
    }

    fn check_visibility_col(&mut self, j: usize) {
        let mut distances_from_heights = [None; 10];
        let mut distance = 1;

        for i in 0..self.height {
            self.check_visibile_step(i, j, &mut distance, &mut distances_from_heights);
        }

        distances_from_heights = [None; 10];
        distance = 1;
        for i in (0..self.height).rev() {
            self.check_visibile_step(i, j, &mut distance, &mut distances_from_heights);
        }
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Tree;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(index.0 < self.height);
        assert!(index.1 < self.width);
        &self.trees[index.0 * self.width + index.1]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert!(index.0 < self.height);
        assert!(index.1 < self.width);
        &mut self.trees[index.0 * self.width + index.1]
    }
}

fn check_input(input: &str) -> Result<(usize, usize), StringError> {
    let mut width = None;
    let mut current_width = 0;
    let mut height = 1;

    for c in input.chars() {
        match c {
            '0'..='9' => current_width += 1,
            '\n' => {
                height += 1;

                if width.is_none() {
                    width = Some(current_width);
                }

                if width.unwrap() != current_width {
                    return Err("d08: not all rows have the same width.".into());
                }

                current_width = 0;
            }
            '\r' => (),
            _ => return Err("d08: invalid input char.".into()),
        }
    }

    Ok((current_width, height))
}
