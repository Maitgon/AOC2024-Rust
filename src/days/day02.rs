use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use nom::{
    IResult,
    character::complete::{digit1, one_of},
    multi::separated_list1,
    bytes::complete::tag,
};

type Input = Vec<Vec<i32>>;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/input02.txt").unwrap();

    let input = match parse(&input) {
        Ok((_, input)) => input,
        _ => panic!("Failed to parse input"),
    };

    // Your solution here...
    let sol1: u64 = part1(&input);
    let sol2: u64 = part2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

// Part 1
fn part1(input: &Input) -> u64 {
    input.iter()
        .filter(|x| (ascending(x) || descending(x)) && diff(x))
        .count() as u64
}

fn part2(input: &Input) -> u64 {
    input.iter()
        .filter(|x| check_all(x))
        .count() as u64
}

fn check_all(v: &[i32]) -> bool {
    for i in 0..v.len() {
        let vec: Vec<i32> = v.iter().enumerate()
        .filter(|&(ind, _)| ind != i) // Skip the element at `index`
        .map(|(_, &value)| value)    // Collect the remaining elements
        .collect();

        if (ascending(&vec) || descending(&vec)) && diff(&vec) {
            return true;
        }
    }

    false
}



fn ascending(report: &[i32]) -> bool {
    for i in 0..report.len()-1 {
        if report[i] > report[i+1] {
            return false;
        }
    }

    true
}

fn descending(report: &[i32]) -> bool {
    for i in 0..report.len()-1 {
        if report[i] < report[i+1] {
            return false;
        }
    }

    true
}

fn diff(report: &[i32]) -> bool {
    for i in 0..report.len()-1 {
        if abs_diff(report[i], report[i+1]) > 3 || abs_diff(report[i], report[i+1]) < 1 {
            return false;
        }
    }

    true
}

fn abs_diff(a: i32, b: i32) -> i32 {
    if a >= b {
        a - b
    } else {
        b - a
    }
}

// Parsing the Input
fn parse(input: &str) -> IResult<&str, Input> {
    separated_list1(tag("\n"), parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, line) = separated_list1(one_of(" \t"), digit1)(input)?;

    Ok((input, line.iter().map(|x| x.parse().unwrap()).collect()))
}
