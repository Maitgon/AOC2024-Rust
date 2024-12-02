use crate::{Solution, SolutionPair};
use std::{fs::read_to_string, cmp::Ordering};

///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Input {
    list1: Vec<i32>,
    list2: Vec<i32>
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/input01.txt").unwrap();

    let input = parse(input);
    // Your solution here...
    let sol1: i32 = part1(input.clone());
    let sol2: i32 = part2(input);

    (Solution::from(sol1), Solution::from(sol2))
}

// Part 1

fn part1(mut input: Input) -> i32 {
    input.list1.sort();
    input.list2.sort();

    input.list1.iter()
        .zip(input.list2.iter())
        .map(|(a, b)| abs_diff(*a,*b))
        .sum()
}

// Part 2

fn part2(mut input: Input) -> i32 {
    input.list1.sort();
    input.list2.sort();

    let mut total = 0;
    let mut idx = 0;

    for n in input.list1 {
        let mut similarity = 0;
        while idx < input.list2.len() {
            match n.cmp(&input.list2[idx]) {
                Ordering::Greater => idx += 1,
                Ordering::Equal => {
                    similarity += 1;
                    idx += 1;
                }
                Ordering::Less => break,
            }
        }
        total += n*similarity;
    }

    total
}

// Parsing

fn parse(input: String) -> Input {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in input.split('\n') {
        if let Some((l, r)) = line.split_once("   ") {
            list1.push(l.parse().unwrap());
            list2.push(r.parse().unwrap());
        }
    }

    Input {list1, list2}
}

fn abs_diff(a: i32, b: i32) -> i32 {
    if a >= b {
        a - b
    } else {
        b - a
    }
}
