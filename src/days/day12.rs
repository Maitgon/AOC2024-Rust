use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

use crate::etc::utils::Point;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/input12.txt").unwrap();

    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // Your solution here...
    let (sol1, sol2) = solve_both(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

// Part 1

fn solve_both(input: &[Vec<char>]) -> (u64, u64) {
    let areas = get_areas(input);

    areas.iter().map(|area| mult_tuple(perimeter_corners(area, input), area.len() as u64))
        .fold((0, 0), |acc, (perimeter, corners)| (acc.0 + perimeter, acc.1 + corners))
}

fn mult_tuple(t: (u64, u64), n: u64) -> (u64, u64) {
    (t.0 * n, t.1 * n)
}

fn perimeter_corners(area: &[Point], input: &[Vec<char>]) -> (u64, u64) {
    let mut corners = 0;
    let mut perimeter = 0;
    let aux_dirs = [Point(0, 1), Point(1, 0), Point(0, -1), Point(-1, 0), Point(0, 1)];

    for &p in area {

        for i in 0..4 {
            let dir1 = aux_dirs[i] + p;
            let dir2 = aux_dirs[i + 1] + p;
            let diagonal = aux_dirs[i] + aux_dirs[i + 1] + p;
            // Check if its an outside corner
            if get(p, input) != get(dir1, input) && get(p, input) != get(dir2, input)
                // Check if it's an inside corner
                || get(p, input) == get(dir1, input) && get(p, input) == get(dir2, input) && get(p, input) != get(diagonal, input) {
                    corners += 1;
            }
            // Check fences
            if get(dir1, input) != get(p, input) {
                perimeter += 1;
            }
        }

    }

    //println!("Perimeter: {}", perimeter);
    (perimeter, corners)
}

fn get(p: Point, input: &[Vec<char>]) -> char {
    if p.0 < 0 || p.0 >= input.len() as i32 || p.1 < 0 || p.1 >= input[0].len() as i32 {
        '@'
    } else {
        input[p.0 as usize][p.1 as usize]
    }
}

fn get_areas(input: &[Vec<char>]) -> Vec<Vec<Point>> {
    let mut areas = vec![];
    let mut visited = vec![vec![false; input[0].len()]; input.len()];

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if visited[i][j] {
                continue;
            }

            let mut area = vec![];
            let mut stack = vec![(i, j)];

            while let Some((x, y)) = stack.pop() {
                if visited[x][y] || input[x][y] != input[i][j] {
                    continue;
                }

                visited[x][y] = true;
                area.push(Point(x as i32, y as i32));

                for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;

                    if nx >= 0 && nx < input.len() as i32 && ny >= 0 && ny < input[0].len() as i32 {
                        stack.push((nx as usize, ny as usize));
                    }
                }
            }

            areas.push(area);
        }
    }

    areas
}
