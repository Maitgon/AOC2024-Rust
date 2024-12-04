use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use crate::etc::utils::Point;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/input04.txt").unwrap();

    let input: Vec<&[u8]> = input.lines()
        .map(|line| line.as_bytes())
        .collect();
    // Your solution here...
    let sol1: u64 = part1(&input);
    let sol2: u64 = part2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(input: &[&[u8]]) -> u64 {
    let mut sol = 0;
    let dirs: Vec<Point> = vec![Point(-1,-1), Point(-1,0), Point(-1,1), Point(0,1), Point(1,1), Point(1,0), Point(1,-1), Point(0,-1)];
    let letters: Vec<u8> = vec![b'M', b'A', b'S'];

    for (i, x) in input.iter().enumerate() {
        for (j, y) in x.iter().enumerate() {
            if *y != b'X' {
                continue;
            }

            // Search word in every direction if it is an 'X'
            for d in &dirs {
                let mut p = Point(i as i32,j as i32);

                let mut valid = true;
                for letter in &letters{
                    p += *d;

                    if !(p.0 >= 0 && p.0 < input.len() as i32 && p.1 >= 0 && p.1 < x.len() as i32 && *letter == input[p.0 as usize][p.1 as usize]) {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    sol += 1;
                }
            }
        }
    }

    sol
}

fn part2(input: &[&[u8]]) -> u64 {
    let mut sol = 0;
    for (i, x) in input.iter().enumerate() {
        for (j, y) in x.iter().enumerate() {
            if i == 0 || j == 0 || i == input.len()-1 || j == x.len()-1 || *y != b'A' {
                continue;
            }

            if (input[i-1][j-1] == b'M' && input[i+1][j+1] == b'S' || input[i-1][j-1] == b'S' && input[i+1][j+1] == b'M') &&
               (input[i-1][j+1] == b'M' && input[i+1][j-1] == b'S' || input[i-1][j+1] == b'S' && input[i+1][j-1] == b'M') {
                sol += 1;
               }
        }
    }
    
    sol
}
