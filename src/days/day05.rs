use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use nom::{
    IResult,
    multi::separated_list1,
    bytes::complete::tag,
    sequence::separated_pair,
    character::complete::digit1,
};
use std::collections::HashMap;

struct Input {
    orders: Vec<(i32, i32)>,
    updates: Vec<Vec<i32>>,
}

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/input05.txt").unwrap();

    let input = match parse(&input) {
        Ok((_, input)) => input,
        _ => panic!("Failed to parse input"),
    };
    // Your solution here...
    let (sol1, sol2) = part1(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

// Part 1
fn part1(input: &Input) -> (i32, i32) {
    // First, convert the order into an IntMap
    let mut order: HashMap<i32, Vec<i32>> = HashMap::new();

    // Insert the orders into the map as if the key were there, then add it to the vector, if not, create the vector and add it
    for (k, v) in &input.orders {
        order.entry(*k).or_default().push(*v);
    }

    let mut sol1 = 0;
    let mut sol2 = 0;
    for u in &input.updates {
        let valid = check_valid(&order, u);
        if valid {
            sol1 += u[u.len() / 2];
        } else {
            sol2 += get_median(&order, u, u.len() / 2);
        }
    }

    (sol1, sol2)
}

fn check_valid(order: &HashMap<i32, Vec<i32>>, u: &[i32]) -> bool {
    for (i, v) in u.iter().enumerate() {
        let after = order.get(v);
        if after.is_none() {
            continue;
        }
        // If one is behind, then, return false
        for elem in &u[..i] {
            if after.unwrap().contains(elem) {
                return false;
            }
        }
    }

    true
}

fn get_median(order: &HashMap<i32, Vec<i32>>, u: &[i32], k: usize) -> i32 {
    let mut rest = u.to_vec();

    let mid = rest.remove(0);

    if rest.is_empty() {
        return mid;
    }

    let greater: Vec<&i32> = rest.iter().filter(|x| match order.get(&mid) {
        Some(v) => v.contains(x),
        None => false,
    }).collect();
    let less: Vec<&i32> = rest.iter().filter(|x| !greater.contains(x)).collect();

    if k == less.len() {
        return mid;
    }

    if k < less.len() {
        return get_median(order, &less.iter().map(|x| **x).collect::<Vec<i32>>(), k);
    }

    get_median(order, &greater.iter().map(|x| **x).collect::<Vec<i32>>(), k - less.len() - 1)
}


// Parse Input

// input -> orders '\n\n' updates
fn parse(input: &str) -> IResult<&str, Input> {
    let (s, (l, r)) = separated_pair(parse_orders, tag("\n\n"), parse_updates)(input)?;

    Ok((s, Input { orders: l, updates: r }))
}

// orders -> order ( '\n' order )*
fn parse_orders(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    separated_list1(tag("\n"), parse_order)(input)
}

// order -> i32 '|' i32
fn parse_order(input: &str) -> IResult<&str, (i32, i32)> {
    let (s, (l, d)) = separated_pair(digit1, tag("|"), digit1)(input)?;

    Ok((s, (l.parse().unwrap(), d.parse().unwrap())))
}

// updates -> update ( '\n' update )*
fn parse_updates(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(tag("\n"), parse_update)(input)
}

// update -> i32 ( ',' i32 )*
fn parse_update(input: &str) -> IResult<&str, Vec<i32>> {
    let (s, l) = separated_list1(tag(","), digit1)(input)?;

    Ok((s, l.iter().map(|x| x.parse().unwrap()).collect()))
}

