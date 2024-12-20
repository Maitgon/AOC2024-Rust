use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use rustc_hash::{FxHashMap, FxHashSet};
use crate::etc::utils::Point;

///////////////////////////////////////////////////////////////////////////////

struct Node {
    p: Point,
    d: u64
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/input20.txt").unwrap();
    let input: Vec<Vec<char>> = input.split('\n').map(|line| line.chars().collect()).collect();
    // Your solution here...
    let (sol1, sol2) = solve_both(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_both(grid: &[Vec<char>]) -> (u64, u64) {
    // First, dfs all nodes and get how much it cost from each node to get to the end
    let (start, end) = find_start_end(grid);

    let distances = dfs(grid, &start, &end);

    //println!("{:?}", distances);

    // Now, we start checking the cheats from each point sing dfs again
    // If the difference is greater than 100, then we add the cheat

    let mut sol1 = 0;
    let mut sol2 = 0;
    let mut to_visit: Vec<Point> = Vec::new();
    to_visit.push(start);
    let mut visited: FxHashSet<Point> = FxHashSet::default();

    let mut posible_cheats = Vec::new();
    for i in 0..=20 {
        for j in -20+i..=20-i {
            posible_cheats.push(Point(i, j));
            if i != 0 {
                posible_cheats.push(Point(-i, j));
            }
        }
    }

    while let Some(p) = to_visit.pop() {
        for &cheat in &posible_cheats {
            if get(p + cheat, grid) == '#' {
                continue;
            }

            let new_cheat = p + cheat;
            if distances[p.0 as usize][p.1 as usize] - distances[new_cheat.0 as usize][new_cheat.1 as usize] >= 100 + p.manhattan_distance(&new_cheat) as i64 {
                sol2 += 1;
                if p.manhattan_distance(&new_cheat) == 2 {
                    sol1 += 1;
                }
            }
        }

        visited.insert(p);

        for &dir in &[Point(1, 0), Point(-1, 0), Point(0, -1), Point(0, 1)] {
            let new_p = dir+p;
            if !visited.contains(&new_p) && grid[new_p.0 as usize][new_p.1 as usize] == '.' && distances[new_p.0 as usize][new_p.1 as usize] >= 100 {
                to_visit.push(new_p);
            }
        }
    }

    (sol1, sol2)
}

fn dfs(grid: &[Vec<char>], start: &Point, end: &Point) -> Vec<Vec<i64>> {
    let mut to_visit = vec![Node { p: *end, d: 0 }];
    let mut visited = vec![vec![-1; grid[0].len()]; grid.len()];

    while let Some(node) = to_visit.pop() {
        if visited[node.p.0 as usize][node.p.1 as usize] != -1 && visited[node.p.0 as usize][node.p.1 as usize] < node.d as i64 {
            continue;
        }

        visited[node.p.0 as usize][node.p.1 as usize] = node.d as i64;

        for dir in &[Point(1, 0), Point(-1, 0), Point(0, -1), Point(0, 1)] {
            let new_pos = node.p + *dir;
            if grid[new_pos.0 as usize][new_pos.1 as usize] == '.' || new_pos == *start {
                to_visit.push(Node { p: new_pos, d: node.d + 1 });
            }
        }
    }

    visited
}

fn find_start_end(input: &[Vec<char>]) -> (Point, Point) {
    let mut start = Point(0, 0);
    let mut end = Point(0, 0);

    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'S' {
                start = Point(i as i32, j as i32);
            } else if *c == 'E' {
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
