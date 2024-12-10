use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use crate::etc::utils::Point;
use std::collections::VecDeque;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/input10.txt").unwrap();

    let input: Vec<Vec<u32>> = input.lines()
        .map(|line| line.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect())
        .collect();
    // Your solution here...
    let (sol1, sol2) = part1(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(input: &[Vec<u32>]) -> (u64, u64) {
    let mut sol1 = 0;
    let mut sol2 = 0;
    for (i, line) in input.iter().enumerate() {
        for (j, &n) in line.iter().enumerate() {
            if n == 0 {
                let (trailheads, paths) = number_paths(input, Point(i as i32, j as i32));
                sol1 += trailheads;
                sol2 += paths;
            }
        }
    }

    (sol1, sol2)
}

fn number_paths(input: &[Vec<u32>], start: Point) -> (u64, u64) {
    let mut trailheads = 0;
    let mut paths = 0;
    let mut to_visit = VecDeque::new();
    to_visit.push_back(start);
    let mut visited_number = vec![vec![0; input[0].len()]; input.len()];
    visited_number[start.0 as usize][start.1 as usize] = 1;

    while let Some(current) = to_visit.pop_front() {

        // Add path if we reach 9
        if input[current.0 as usize][current.1 as usize] == 9 {
            trailheads += 1;
            paths += visited_number[current.0 as usize][current.1 as usize];
        }

        
        for (x, y) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            let new = Point(current.0 + x, current.1 + y);
            // In bounds, not visited, and difference is less or equal than 1
            if new.0 >= 0 && new.0 < input.len() as i32 && new.1 >= 0 && new.1 < input[0].len() as i32
                && input[new.0 as usize][new.1 as usize] - input[current.0 as usize][current.1 as usize] == 1 {
                if visited_number[new.0 as usize][new.1 as usize] == 0 {
                    to_visit.push_back(new);
                }
                visited_number[new.0 as usize][new.1 as usize] += visited_number[current.0 as usize][current.1 as usize];
            }
        }
    }

    (trailheads, paths)
}
