use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use nom::{
    bytes::complete::tag, character::complete::digit1, multi::separated_list1, sequence::tuple, IResult
};
use crate::etc::utils::Point;

type Machine = (Point, Point, Point);
type Input = Vec<Machine>;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/input13.txt").unwrap();

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
        .map(minimum_tokens)
        .sum()
}

fn part2(input: &Input) -> u64 {
    input.iter()
        .map(minimum_tokens_2)
        .sum()
}

fn minimum_tokens(eq: &Machine) -> u64 {
    let x1 = eq.0.0 as f64;
    let x2 = eq.1.0 as f64;
    let x3 = eq.2.0 as f64;
    let y1 = eq.0.1 as f64;
    let y2 = eq.1.1 as f64;
    let y3 = eq.2.1 as f64;

    let x = (y3*x2 - x3*y2) / (y1*x2 - x1*y2);
    let y = (x3 - x1*x) / x2;

    // Check if it's integer and both are between 0 and 100
    //println!("x: {}, y: {}", x, y);
    if x.fract() == 0.0 && y.fract() == 0.0 && (0.0..=100.0).contains(&x) && (0.0..=100.0).contains(&y) {
        return 3 * (x as u64) + (y as u64);
    }

    0
}

fn minimum_tokens_2(eq: &Machine) -> u64 {
    let x1 = eq.0.0 as f64;
    let x2 = eq.1.0 as f64;
    let x3 = eq.2.0 as f64 + 10000000000000.0;
    let y1 = eq.0.1 as f64;
    let y2 = eq.1.1 as f64;
    let y3 = eq.2.1 as f64 + 10000000000000.0;

    let x = (y3*x2 - x3*y2) / (y1*x2 - x1*y2);
    let y = (x3 - x1*x) / x2;

    // Check if it's integer and both are between 0 and 100
    //println!("x: {}, y: {}", x, y);
    if x.fract() == 0.0 && y.fract() == 0.0 {
        return 3 * (x as u64) + (y as u64);
    }

    0
}

// Parsing
fn parse(input: &str) -> IResult<&str, Input> {
    separated_list1(tag("\n\n"), parse_machine)(input)
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (res, (_, x1, _, y1, _)) = tuple((tag("Button A: X+"), digit1, tag(", Y+"), digit1, tag("\n")))(input)?;
    let (res, (_, x2, _, y2, _)) = tuple((tag("Button B: X+"), digit1, tag(", Y+"), digit1, tag("\n")))(res)?;
    let (res, (_, x3, _, y3)) = tuple((tag("Prize: X="), digit1, tag(", Y="), digit1))(res)?;

    Ok((res, (Point(x1.parse().unwrap(), y1.parse().unwrap()), Point(x2.parse().unwrap(), y2.parse().unwrap()), Point(x3.parse().unwrap(), y3.parse().unwrap()))))
}
