use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/input09.txt").unwrap();

    let input: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    // Your solution here...
    let sol1: i64 = part1(&input);
    let sol2: i64 = part2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(input: &[u32]) -> i64 {
    let mut blocks: Vec<i64> = Vec::new();
    
    // First block with IDs
    for (i, &n) in input.iter().enumerate() {
        for _ in 0..n {
            if i % 2 == 0 {
                blocks.push(i as i64 / 2);
            } else {
                blocks.push(-1);
            }
        }
    }

    // Change the vector
    let (mut i, mut j) = (0, blocks.len()-1);
    while i != j {
        if blocks[i] != - 1 {
            i += 1;
            continue;
        }

        if blocks[j] == - 1 {
            j -= 1;
            continue;
        }

        (blocks[i], blocks[j]) = (blocks[j], blocks[i]);
        i += 1;
        j -= 1;
    }

    // Now, checksum 
    blocks.iter()
        .filter(|&&n| n >= 0)
        .enumerate()
        .map(|(i, n)| i as i64 * n)
        .sum()

}

#[derive(Copy, Clone)]
struct IDs {
    id: i64,
    size: u32
}

fn part2(input: &[u32]) -> i64 {
    let mut blocks: Vec<IDs> = Vec::new();
    
    // First block with IDs
    for (i, &n) in input.iter().enumerate() {
        if i % 2 == 0 {
            blocks.push(IDs { id: i as i64 / 2, size: n});
        } else {
            blocks.push(IDs { id: -1, size: n});
        }
    }
    
    // Move around and find out
    for j in (0..blocks.len()).rev() {
        // If it's '.' ignore it
        if blocks[j].id == -1 {
            continue;
        }

        // If it's a number find the firt space and move it
        for i in 0..j {
            if blocks[i].id != -1 || blocks[i].size < blocks[j].size {
                continue;
            }

            //println!("Change: {} {}", blocks[i].id, blocks[j].id);

            // Change positions and add the rest
            let diff_size = blocks[i].size - blocks[j].size;
            blocks[i] = blocks[j];
            blocks[j].id = -1;

            // Add '.' in between
            if diff_size > 0 {
                blocks.insert(i+1, IDs { id: -1, size: diff_size });
            }
            break;
        }
    }

    // Create vector for checksum
    let mut checksum_vec = Vec::new();

    for block in blocks {
        for _ in 0..block.size {
            checksum_vec.push(block.id);
        }
    }

    // Now, checksum
    checksum_vec.iter()
        .enumerate()
        .filter(|(_, &n)| n >= 0)
        .map(|(i, n)| i as i64 * n)
        .sum()
}
