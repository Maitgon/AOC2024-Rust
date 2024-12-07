use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use nom::{
    IResult,
    multi::separated_list1,
    bytes::complete::tag,
    sequence::separated_pair,
    character::complete::digit1,
};

#[derive(Debug)]
struct Equation {
    target: u64,
    numbers: Vec<u64>,
}

type Input = Vec<Equation>;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/input07.txt").unwrap();

    let input = match parse(&input) {
        Ok((_, input)) => input,
        _ => panic!("Failed to parse input"),
    };

    // Your solution here...
    let sol1: u64 = part1(&input);
    let sol2: u64 = part2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(input: &Input) -> u64 {
    input.iter()
        .filter(|eq| valid(eq.target, &eq.numbers[1..], eq.numbers[0]))
        .map(|eq| eq.target)
        .sum()
}

fn valid(target: u64, numbers: &[u64], acc: u64) -> bool {
    if acc > target {
        return false;
    }


    if numbers.len() == 1 {
        return numbers[0] + acc == target || numbers[0] * acc == target;
    }

    valid(target, &numbers[1..], numbers[0] + acc) || valid(target, &numbers[1..], numbers[0] * acc)
}

fn part2(input: &Input) -> u64 {
    input.iter()
        .filter(|eq| valid2(eq.target, &eq.numbers[1..], eq.numbers[0]))
        .map(|eq| eq.target)
        .sum()
}

fn valid2(target: u64, numbers: &[u64], acc: u64) -> bool {
    if acc > target {
        return false;
    }

    if numbers.len() == 1 {
        return numbers[0] + acc == target || numbers[0] * acc == target || concatenate(acc, numbers[0]) == target;
    }

    valid2(target, &numbers[1..], numbers[0] + acc) || valid2(target, &numbers[1..], numbers[0] * acc) || valid2(target, &numbers[1..], concatenate(acc, numbers[0]))
}

fn concatenate(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

// Parsing

fn parse(input: &str) -> IResult<&str, Input> {
    separated_list1(tag("\n"), parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Equation> {
    let (input, (target, numbers)) = separated_pair(
        digit1,
        tag(": "),
        separated_list1(tag(" "), digit1),
    )(input)?;

    Ok((input, Equation {
        target: target.parse().unwrap(),
        numbers: numbers.iter().map(|n| n.parse().unwrap()).collect(),
    }))
}