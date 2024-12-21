use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use crate::etc::utils::Point;
use pathfinding::directed::astar::astar_bag;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/input21.txt").unwrap();
    let input: Vec<String> = input.lines().map(|line| line.to_string()).collect();
    // Your solution here...
    let sol1: u64 = part1(&input);
    let sol2: u64 = part2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(input: &[String]) -> u64 {
    input.par_iter().map(|line| path_len(line, 2) * line[0..3].parse::<u64>().unwrap()).sum()
}

fn part2(input: &[String]) -> u64 {
    input.par_iter().map(|line| path_len(line, 25) * line[0..3].parse::<u64>().unwrap()).sum()
}

fn path_len(input: &String, depth: u64) -> u64 {
    // Get the pads and their paths
    let numerical_pad: Vec<Vec<char>> = vec![ vec!['7', '8', '9'],
                                              vec!['4', '5', '6'],
                                              vec!['1', '2', '3'],
                                              vec!['#', '0', 'A']];

    let numerical_paths = get_all_paths(&numerical_pad);

    let directional_pad: Vec<Vec<char>> = vec![ vec!['#', '^', 'A'],
                                                vec!['<', 'v', '>']];

    let directional_paths = get_all_paths(&directional_pad);

    let mut robot1: Vec<String> = vec!["".to_string()];
    for (c1, c2) in format!("A{}",input).chars().zip(input.chars()) {

        let mut new_paths = Vec::new();
        for path1 in &robot1 {
            for path2 in &numerical_paths[&(c1, c2)] {
                new_paths.push(path1.clone() + path2);
            }
        }

        robot1 = new_paths; // In the first step, all paths are the shortest one
    }

    let mut directional_paths_lengths: FxHashMap<(char, char, u64), u64> = FxHashMap::default();
    for (&(s, e), val) in &directional_paths {
        directional_paths_lengths.insert((s, e, 1), val[0].len() as u64);
    }

    let mut optimal = u64::MAX;
    for paths in robot1 {
        let mut length = 0;
        for (s, e) in format!("A{}",paths).chars().zip(paths.chars()) {
            length += compute_min_length(s, e, depth, &mut directional_paths_lengths, &directional_paths)
        }
        optimal = std::cmp::min(length, optimal)
    }

    optimal
}

fn compute_min_length(s: char, e: char, depth: u64, mem: &mut FxHashMap<(char, char, u64), u64>, directional_paths: &FxHashMap<(char, char), Vec<String>>) -> u64 {
    if let Some(v) = mem.get(&(s, e, depth)) {
        return *v
    }

    let mut optimal = u64::MAX;
    for paths in &directional_paths[&(s, e)] {
        let mut total_length = 0;
        for (x, y) in format!("A{}",paths).chars().zip(paths.chars()) {
            let val = compute_min_length(x, y, depth-1, mem, directional_paths);
            mem.insert((x, y, depth - 1), val);
            total_length += val;
        }
        optimal = std::cmp::min(optimal, total_length);
    }

    optimal
}

fn get_all_paths(pad: &[Vec<char>]) -> FxHashMap<(char, char), Vec<String>> {
    let mut paths_all: FxHashMap<(char, char), Vec<String>> = FxHashMap::default();
    for line in pad.iter() {
        for &c in line {
            for line2 in pad.iter() {
                for &c2 in line2 {
                    if c == '#' || c2 == '#' {
                        continue;
                    }

                    if c == c2 {
                        paths_all.insert((c, c2), vec!["A".to_string()]);
                        continue;
                    }

                    let paths = shortest_paths(pad, c, c2);

                    paths_all.insert((c, c2), paths);
                        
                }
            }
        }
    }

    paths_all
}

fn shortest_paths(input: &[Vec<char>], start: char, end: char) -> Vec<String> {
    // Find start and end positions
    let (start, end) = find_start_end(input, start, end);

    // Find shortest paths
    let result = astar_bag(
        &start,
        |p| neighbors(input, p),
        |p| p.manhattan_distance(&end) as u64,
        |p| *p == end
    ).unwrap();

    let mut paths = Vec::new();
    for path in result.0 {
        // result as directions ^ > v <
        let mut path_chars = String::new();
        for i in 0..path.len()-1 {
            match path[i+1] - path[i] {
                Point(1, 0) => path_chars.push('v'),
                Point(-1, 0) => path_chars.push('^'),
                Point(0, 1) => path_chars.push('>'),
                Point(0, -1) => path_chars.push('<'),
                _ => unreachable!()
            }
        }
        path_chars.push('A');
        paths.push(path_chars);
    }
    
    paths
}

fn neighbors(input: &[Vec<char>], p: &Point) -> Vec<(Point, u64)> {
    let mut neighbors = Vec::new();
    for dir in &[Point(1, 0), Point(-1, 0), Point(0, -1), Point(0, 1)] {
        let new_p = *p + *dir;
        if get(new_p, input) != '#' {
            neighbors.push((new_p, 1));
        }
    }
    neighbors
}



fn find_start_end(input: &[Vec<char>], s: char, e: char) -> (Point, Point) {
    let mut start = Point(0, 0);
    let mut end = Point(0, 0);

    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == s {
                start = Point(i as i32, j as i32);
            } else if *c == e {
                end = Point(i as i32, j as i32);
            }
        }
    }

    (start, end)
}

fn get(p: Point, grid: &[Vec<char>]) -> char {
    if p.0 < 0 || p.1 < 0 || p.0 >= grid.len() as i32 || p.1 >= grid[0].len() as i32 {
        return '#';
    }
    grid[p.0 as usize][p.1 as usize]
}