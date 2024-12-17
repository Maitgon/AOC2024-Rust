use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use nom::{
    bytes::complete::tag, character::complete::digit1, multi::separated_list1, sequence::tuple, IResult, multi::many1, character::complete::one_of
};
use crate::etc::utils::Point;
use std::time::Duration;
use std::{thread, vec};

#[derive(Debug, Clone, Copy)]
struct Robot {
    p: Point,
    v: Point,
}

type Input = Vec<Robot>;

const GRID_WIDTH: i32 = 101;
const GRID_HEIGHT: i32 = 103;
const LINE: i32 = 15;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/input14.txt").unwrap();

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
fn simulate_robot(robot: &Robot, time: i32) -> Point {
    Point((robot.p.0 + robot.v.0 * time).rem_euclid(GRID_WIDTH), (robot.p.1 + robot.v.1 * time).rem_euclid(GRID_HEIGHT))
}

fn part1(input: &Input) -> u64 {
    let new_robots = input.iter()
        .map(|robot| simulate_robot(robot, 100))
        .collect::<Vec<Point>>();

    let mut corner_robots: [u64; 4] = [0; 4];

    for robot in new_robots {
        if robot.0 < GRID_WIDTH / 2 && robot.1 < GRID_HEIGHT / 2 {
            corner_robots[0] += 1;
        } else if robot.0 > GRID_WIDTH / 2 && robot.1 < GRID_HEIGHT / 2 {
            corner_robots[1] += 1;
        } else if robot.0 < GRID_WIDTH / 2 && robot.1 > GRID_HEIGHT / 2 {
            corner_robots[2] += 1;
        } else if robot.0 > GRID_WIDTH / 2 && robot.1 > GRID_HEIGHT / 2 {
            corner_robots[3] += 1;
        }
    }

    corner_robots.iter().product()
}

// Part 2
fn simulate_robot_2(robot: &Robot) -> Point {
    Point((robot.p.0 + robot.v.0).rem_euclid(GRID_WIDTH), (robot.p.1 + robot.v.1).rem_euclid(GRID_HEIGHT))
}

fn part2(input: &Input) -> u64 {
    let mut time = 0;
    let mut new_robots = input.clone();

    loop {
        let mut grid = vec![vec![false; GRID_HEIGHT as usize]; GRID_WIDTH as usize];
        new_robots = new_robots.iter()
            .map(|robot| {
                let new_robot = simulate_robot_2(robot);
                grid[new_robot.0 as usize][new_robot.1 as usize] = true;
                Robot {
                    p: new_robot,
                    v: robot.v,
                }
            })
            .collect();
        time += 1;

        // Check for a line of width 20

        for x in 30..(GRID_WIDTH - 30) {
            for y in 0..GRID_HEIGHT / 2 {
                let mut count = 0;
                for k in x..(x+LINE) {
                    if grid[k as usize][y as usize] {
                        count += 1;
                    }
                }
                if count == LINE {
                    return time;
                }
            }
        }
    }
}

// Parsing
fn parse(input: &str) -> IResult<&str, Input> {
    separated_list1(tag("\n"), parse_machine)(input)
}

fn parse_int(input: &str) -> IResult<&str, i32> {
    let (res, nums) = many1(one_of("1234567890-"))(input)?;
    Ok((res, nums.iter().collect::<String>().parse().unwrap()))
}

fn parse_machine(input: &str) -> IResult<&str, Robot> {
    let (res, (_, px, _, py)) = tuple((tag("p="), parse_int, tag(","), parse_int))(input)?;
    let (res, (_, vx, _, vy)) = tuple((tag(" v="), parse_int, tag(","), parse_int))(res)?;
    Ok((res, Robot {
        p: Point(px, py),
        v: Point(vx, vy),
    }))
}
