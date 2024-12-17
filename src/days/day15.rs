use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use crate::etc::utils::Point;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/input15.txt").unwrap();
    let (grid_str, dirs_str) = input.split_once("\n\n").unwrap();
    let grid: Vec<Vec<char>> = grid_str.split('\n').map(|line| line.chars().collect()).collect();
    
    let mut dirs: Vec<Point> = Vec::new();
    for c in dirs_str.chars() {
        let p = match c {
            '^' => Point(-1, 0),
            '>' => Point(0, 1),
            'v' => Point(1, 0),
            '<' => Point(0, -1),
            _ => continue,
        };

        dirs.push(p);
    }

    let grid2: Vec<Vec<char>> = grid.iter().map( |line| {
        line.iter().flat_map(|&c| match c {
            '@' => ['@', '.'],
            '#' => ['#', '#'],
            'O' => ['[', ']'],
            '.' => ['.', '.'],
            _ => unreachable!(),
        }).collect()
    }).collect();
    
    // Your solution here...
    let sol1: u64 = part1(grid, &dirs);
    let sol2: u64 = part2(grid2, &dirs);

    (Solution::from(sol1), Solution::from(sol2))
}

// Part 1

fn part1(mut grid: Vec<Vec<char>>, dirs: &Vec<Point>) -> u64 {
    // Find @
    let mut robot: Point = Point(-1, -1);
    'outer: for (i, line) in grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == '@' {
                robot = Point(i as i32, j as i32);
                break 'outer;
            }
        }
    }

    for &dir in dirs {
        let mut new_pos = robot + dir;

        while grid[new_pos.0 as usize][new_pos.1 as usize] == 'O' {
            new_pos += dir;
        }

        // Don't move if robot find a wall
        if grid[new_pos.0 as usize][new_pos.1 as usize] == '#' {
            continue;
        }

        // Move everything if robot finds '.'
        robot += dir;
        grid[new_pos.0 as usize][new_pos.1 as usize] = 'O';
        grid[robot.0 as usize][robot.1 as usize] = '.';
    }

    // Count boxes
    let mut gps_coordinates = 0;
    for (i, line) in grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'O' {
                gps_coordinates += 100*i + j
            }
        }
    }

    gps_coordinates as u64
}

// part 2

fn part2(mut grid: Vec<Vec<char>>, dirs: &Vec<Point>) -> u64 {
    let mut robot: Point = Point(-1, -1);
    'outer: for (i, line) in grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == '@' {
                robot = Point(i as i32, j as i32);
                break 'outer;
            }
        }
    }

    for &dir in dirs {
        let mut new_pos = robot + dir;

        if grid[new_pos.0 as usize][new_pos.1 as usize] == '.' {
            grid[robot.0 as usize][robot.1 as usize] = '.';
            robot = new_pos;
            grid[robot.0 as usize][robot.1 as usize] = '@';
            continue;
        }

        if grid[new_pos.0 as usize][new_pos.1 as usize] == '#' {
            continue;
        }

        // Now robot is colliding with boxes
        
        // Horizontal
        if dir == Point(0, 1) || dir == Point(0, -1) {
            while grid[new_pos.0 as usize][new_pos.1 as usize] == '[' || grid[new_pos.0 as usize][new_pos.1 as usize] == ']' {
                new_pos += dir + dir;
            }

            if grid[new_pos.0 as usize][new_pos.1 as usize] == '#' {
                continue;
            }

            // Else '.'
            while new_pos != robot {
                let prev_pos = new_pos - dir;
                grid[new_pos.0 as usize][new_pos.1 as usize] = grid[prev_pos.0 as usize][prev_pos.1 as usize];
                new_pos = prev_pos;
            }
            grid[robot.0 as usize][robot.1 as usize] = '.';
            robot += dir;
        }

        // Vertical moves
        if dir == Point(1, 0) || dir == Point(-1, 0) {
            let mut other_pos = match grid[new_pos.0 as usize][new_pos.1 as usize] {
                '[' => new_pos + Point(0, 1),
                ']' => new_pos + Point(0, -1),
                _ => unreachable!(),
            };

            if grid[new_pos.0 as usize][new_pos.1 as usize] == ']' {
                (new_pos, other_pos) = (other_pos, new_pos);
            }

            if can_move(&grid, new_pos, other_pos, dir) {
                move_boxes(&mut grid, new_pos, other_pos, dir);
                grid[robot.0 as usize][robot.1 as usize] = '.';
                robot += dir;
                grid[robot.0 as usize][robot.1 as usize] = '@';
            }
        }        
    }
    
    // Count boxes
    let mut gps_coordinates = 0;
    for (i, line) in grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == '[' {
                gps_coordinates += 100*i + j
            }
        }
    }

    gps_coordinates as u64
}

fn can_move(grid: &Vec<Vec<char>>, left: Point, right: Point, dir: Point) -> bool {
    // Check if left part can move
    let left_pos_move = left + dir;
    let can_move_left: bool;
    if grid[left_pos_move.0 as usize][left_pos_move.1 as usize] == '#' {
        return false;
    } else if grid[left_pos_move.0 as usize][left_pos_move.1 as usize] == '.' {
        can_move_left = true;
    } else if grid[left_pos_move.0 as usize][left_pos_move.1 as usize] == '[' {
        return can_move(grid, left_pos_move, left_pos_move + Point(0, 1), dir);
    } else {
        can_move_left = can_move(grid, left_pos_move + Point(0, -1), left_pos_move, dir)
    }

    // Check if right part can move
    let right_pos_move = right + dir;
    let can_move_right: bool;
    if grid[right_pos_move.0 as usize][right_pos_move.1 as usize] == '#' {
        return false;
    } else if grid[right_pos_move.0 as usize][right_pos_move.1 as usize] == '.' {
        can_move_right = true;
    } else { // Other case already covered in left part
        can_move_right = can_move(grid, right_pos_move, right_pos_move + Point(0, 1), dir);
    }

    can_move_left && can_move_right
}

fn move_boxes(grid: &mut Vec<Vec<char>>, left: Point, right: Point, dir: Point) {
    // Check if you need to move boxes first
    let left_pos_move = left + dir;
    if grid[left_pos_move.0 as usize][left_pos_move.1 as usize] == '[' {
        move_boxes(grid, left_pos_move, left_pos_move + Point(0,1), dir);
    } else if grid[left_pos_move.0 as usize][left_pos_move.1 as usize] == ']' {
        move_boxes(grid, left_pos_move + Point(0, -1), left_pos_move, dir);
    }

    let right_pos_move = right + dir;
    if grid[right_pos_move.0 as usize][right_pos_move.1 as usize] == '[' {
        move_boxes(grid, right_pos_move, right_pos_move + Point(0, 1), dir);
    }

    // After moving the rest of the boxes move these
    grid[left_pos_move.0 as usize][left_pos_move.1 as usize] = '[';
    grid[right_pos_move.0 as usize][right_pos_move.1 as usize] = ']';

    grid[left.0 as usize][left.1 as usize] = '.';
    grid[right.0 as usize][right.1 as usize] = '.';
}
