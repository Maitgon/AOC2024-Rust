use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::collections::HashSet;

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point(i32, i32);
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Guard {
    position: Point,
    direction: Direction,
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/input06.txt").unwrap();

    let input: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();
    // Your solution here...
    let sol1: usize = part1(&input);
    let sol2: usize = part2(input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(input: &[Vec<u8>]) -> usize {
    // Find ^

    let mut guard = Guard{ position: Point(0, 0), direction: Direction::Up };
    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == b'^' {
                guard.position = Point(i as i32, j as i32);
            }
        }
    }

    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(guard.position);

    loop {
        let next = match guard.direction {
            Direction::Up => Point(guard.position.0 - 1, guard.position.1),
            Direction::Down => Point(guard.position.0 + 1, guard.position.1),
            Direction::Left => Point(guard.position.0, guard.position.1 - 1),
            Direction::Right => Point(guard.position.0, guard.position.1 + 1),
        };

        if !(next.0 >= 0 && next.0 < input.len() as i32 && next.1 >= 0 && next.1 < input[0].len() as i32) {
            break;
        }

        if input[next.0 as usize][next.1 as usize] == b'#' {
            guard.direction = match guard.direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };
            continue;
        }

        visited.insert(next);
        guard.position = next;
    }

    visited.len()
}

fn part2(mut input: Vec<Vec<u8>>) -> usize {
    // Find ^

    let mut position = Point(0, 0);
    let direction = Direction::Up;
    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == b'^' {
                position = Point(i as i32, j as i32);
            }
        }
    }

    let mut sol = 0;

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let mut guard = Guard{ position, direction };

            if input[i][j] == b'#' {
                continue;
            } else {
                input[i][j] = b'#';
            }

            let mut visited: HashSet<Guard> = HashSet::new();
            visited.insert(guard);

            // Find the loop

            loop {
                let next = match guard.direction {
                    Direction::Up => Point(guard.position.0 - 1, guard.position.1),
                    Direction::Down => Point(guard.position.0 + 1, guard.position.1),
                    Direction::Left => Point(guard.position.0, guard.position.1 - 1),
                    Direction::Right => Point(guard.position.0, guard.position.1 + 1),
                };
        
                if !(next.0 >= 0 && next.0 < input.len() as i32 && next.1 >= 0 && next.1 < input[0].len() as i32) {
                    input[i][j] = b'.';
                    break;
                }

                /*if i == 6 && j == 3 {
                    println!("{:?} {:?}", guard, next);
                }*/
        
                if input[next.0 as usize][next.1 as usize] == b'#' {
                    guard.direction = match guard.direction {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    };
                    continue;
                }
                guard.position = next;
                if visited.contains(&guard) {
                    sol += 1;
                    input[i][j] = b'.';
                    break;
                }
                visited.insert(guard);
            }


        }
    }

    sol
}
