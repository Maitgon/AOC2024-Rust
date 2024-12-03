use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::char,
    sequence::{delimited, separated_pair},
    combinator::map_res,
    IResult,
};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/input03.txt").unwrap();

    let input1 = parse(&input);
    let input2 = parse2(&input);
    
    // Your solution here...
    let sol1: i32 = input1.iter().map(|(a,b)| a*b).sum();
    let sol2: i32 = input2.iter().map(|(a,b)| a*b).sum();

    (Solution::from(sol1), Solution::from(sol2))
}

// Parsing

fn parse_number(input: &str) -> IResult<&str, i32> {
    map_res(take_while_m_n(1, 3, |c: char| c.is_ascii_digit()), |s: &str| s.parse::<i32>())(input)
}

fn parse_mult(input: &str) -> IResult<&str, (i32, i32)> {
    delimited(
        tag("mul("),
        separated_pair(parse_number, char(','), parse_number),
        char(')'),
    )(input)
}

fn parse(input: &str) -> Vec<(i32, i32)> {
    let mut results = Vec::new();
    let mut remaining = input;

    while !remaining.is_empty() {
        match parse_mult(remaining) {
            Ok((next_input, result)) => {
                results.push(result);
                remaining = next_input;
            }
            Err(_) => {
                remaining = &remaining[1..];
            }
        }
    }

    results
}

fn parse2(input: &str) -> Vec<(i32, i32)> {
    let mut results = Vec::new();
    let mut remaining = input;

    let mut can = true;

    while !remaining.is_empty() {
        if let Ok((next_input, _)) = tag::<&str, &str, nom::error::Error<&str>>("do()")(remaining) {
            can = true;
            remaining = next_input;
        }
        if let Ok((next_input, _)) = tag::<&str, &str, nom::error::Error<&str>>("don't()")(remaining) {
            can = false;
            remaining = next_input;
        }
        match parse_mult(remaining) {
            Ok((next_input, result)) => {
                if can {
                    results.push(result);
                }
                remaining = next_input;
            }
            Err(_) => {
                remaining = &remaining[1..];
            }
        }
    }

    results
}
