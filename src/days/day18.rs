use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use pathfinding::directed::astar::astar;
use crate::etc::utils::Point;

///////////////////////////////////////////////////////////////////////////////

const GRID_SIZE: usize = 71;
const DIRS: [Point; 4] = [Point(1, 0), Point(-1, 0), Point(0, -1), Point(0, 1)];
//                              Down         Up            Left          Right

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/input18.txt").unwrap();
    let input: Vec<Point> = input.split('\n')
        .map(parse_to_point)
        .collect();

    // Your solution here...
    let (sol1, sol2) = part1(&input);
    let sol2 = format!("{},{}", sol2.0, sol2.1);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(input: &[Point]) -> (u64, Point) {
    let mut grid = vec![vec![true; GRID_SIZE]; GRID_SIZE];

    for p in &input[..1024] {
        grid[p.0 as usize][p.1 as usize] = false;
    }

    let end = Point(GRID_SIZE as i32 - 1, GRID_SIZE as i32 - 1);

    let best_path = astar(
        &Point(0, 0),
        |state| succesors(&grid, state),
        |state| heuristic(end, state),
        |p| *p == end
    );

    let mut can_go = best_path.is_some();

    let sol1 = best_path.unwrap().1;
    let mut n_byte = 1024;
    while can_go {
        grid[input[n_byte].0 as usize][input[n_byte].1 as usize] = false;

        can_go = astar(
            &Point(0, 0),
            |state| succesors(&grid, state),
            |state| heuristic(end, state),
            |p| *p == end
        ).is_some();

        n_byte += 1;
    }

    (sol1, input[n_byte - 1])
}

fn succesors(input: &[Vec<bool>], state: &Point) -> Vec<(Point, u64)> {
    let mut succs = Vec::new();
    for dir in DIRS {
        let new_pos = *state + dir;
        if get(new_pos, input) {
            succs.push((new_pos, 1));
        }
    }
    succs
}

fn heuristic(end: Point, state: &Point) -> u64 {
    end.manhattan_distance(&state) as u64
}

fn get(p: Point, input: &[Vec<bool>]) -> bool {
    if p.0 < 0 || p.0 >= input.len() as i32 || p.1 < 0 || p.1 >= input[0].len() as i32 {
        false
    } else {
        input[p.0 as usize][p.1 as usize]
    }
}

// Parsing
fn parse_to_point(str: &str) -> Point {
    let (l, r) = str.split_once(',').unwrap();
    Point(l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap())
}
