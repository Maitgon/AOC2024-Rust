use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use rustc_hash::FxHashMap;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/input11.txt").unwrap();

    let input = input.split(' ').map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    // Your solution here...
    let (sol1, sol2) = both(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn both(input: &[u64]) -> (u64, u64) {
    let mut sol1 = 0;
    let mut new_input: FxHashMap<u64, u64> = input.iter().map(|&x| (x, 1)).collect();
    for i in 0..75 {
        let mut changed_input: FxHashMap<u64, u64> = FxHashMap::default();
        for (&x, &n) in new_input.iter() {
            if x == 0 {
                *changed_input.entry(1).or_insert(0) += n;
                continue;
            }

            // If n has even number of digits divide into left digits and right digits
            let x_digits = x.checked_ilog10().unwrap_or(0) + 1;
            if x_digits % 2 == 0 {
                let left = x / 10_u64.pow(x_digits / 2);
                let right = x % 10_u64.pow(x_digits / 2);
                *changed_input.entry(left).or_insert(0) += n;
                *changed_input.entry(right).or_insert(0) += n;
            } else {
                *changed_input.entry(x*2024).or_insert(0) += n;
            }
        }
        if i == 25 {
            sol1 = changed_input.values().sum();
        }
        new_input = changed_input;
    }

    (sol1, new_input.values().sum())
}
