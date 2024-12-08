use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use crate::etc::utils::Point;
use rustc_hash::{FxHashMap, FxHashSet};
use itertools::Itertools;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/input08.txt").unwrap();

    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // Your solution here...
    let (sol1, sol2) = both(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn both(input: &[Vec<char>]) -> (usize, usize) {
    let mut antenas: FxHashMap<char, Vec<Point>> = FxHashMap::default();

    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c != '.' {
                antenas.entry(*c).or_default().push(Point(i as i32, j as i32))
            }
        }
    }

    let mut antinodes: FxHashSet<Point> = FxHashSet::default();
    let mut antinodes2: FxHashSet<Point> = FxHashSet::default();

    for (_, p) in antenas {
        for (x, y) in p.iter().tuple_combinations() {
            // Add to part 1 antinodes
            let delta = *x - *y;
            antinodes.insert(*y - delta);
            antinodes.insert(*x + delta);

            // Add to part 2 antinodes
            let mut y = *y;
            while y.0 >= 0 && y.0 < input.len() as i32 && y.1 >= 0 && y.1 < input[0].len() as i32 {
                antinodes2.insert(y);
                y -= delta;
            }

            let mut x = *x;
            while x.0 >= 0 && x.0 < input.len() as i32 && x.1 >= 0 && x.1 < input[0].len() as i32 {
                antinodes2.insert(x);
                x += delta;
            }
            
        }
    }

    let sol1 = antinodes.iter()
        .filter(|Point(x, y)| *x >= 0 && *x < input.len() as i32 && *y >= 0 && *y < input[0].len() as i32)
        .count();
    
    (sol1, antinodes2.len())
}
