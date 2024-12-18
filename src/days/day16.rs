use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use pathfinding::directed::astar::astar_bag;
use crate::etc::utils::Point;
use rustc_hash::FxHashSet;

const DIRS: [Point; 4] = [Point(1, 0), Point(-1, 0), Point(0, -1), Point(0, 1)];
//                              Down         Up            Left          Right

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    pos: Point,
    dir: Point,
}

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/input16.txt").unwrap();
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // Your solution here...
    let (sol1, sol2) = both(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn both(input: &[Vec<char>]) -> (u64, u64) {
    let (start, end) = find_start_end(input);

    let (best_paths, best_cost) = astar_bag(
        &State {pos: start, dir: DIRS[3]},
        |state| succesors(input, state),
        |state| heuristic(end, state),
        |state| state.pos == end
    ).unwrap();

    let mut seats = FxHashSet::default();
    for path in best_paths {
        for state in path {
            seats.insert(state.pos);
        }
    }
    (best_cost, seats.len() as u64)
}

fn succesors(input: &[Vec<char>], state: &State) -> Vec<(State, u64)> {
    let mut succs = Vec::new();
    for dir in DIRS {
        let new_pos = state.pos + dir;
        if input[new_pos.0 as usize][new_pos.1 as usize] == '.' || input[new_pos.0 as usize][new_pos.1 as usize] == 'E' {
            let new_state = State {pos: new_pos, dir};
            let cost = if dir == state.dir { 1 } else { 1001 };
            succs.push((new_state, cost));
        }
    }
    succs
}

fn heuristic(end: Point, state: &State) -> u64 {
    let md = end.manhattan_distance(&state.pos);

    if end.0 - state.pos.0 == 0 || end.1 - state.pos.1 == 0 {
        md as u64
    } else {
        md as u64 + 1000
    }
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
