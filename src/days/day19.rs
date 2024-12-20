use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp;
use rayon::prelude::*;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/input19.txt").unwrap();
    let input_aux = input.split_once("\n\n").unwrap();

    let towels: FxHashSet<&str> = input_aux.0.split(", ").collect();
    let patterns: Vec<&str> = input_aux.1.split('\n').collect();

    // Your solution here...
    let (sol1, sol2) = solve_both(&towels, &patterns);

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_both(towels: &FxHashSet<&str>, patterns: &[&str]) -> (u64, u64) {
    let sols: Vec<u64> = patterns.par_iter()
        .map(|pattern| count(pattern, towels))
        .collect();
        
    sols.iter().fold(  (0, 0), |(acc1, acc2), &count| (acc1 + (count > 0) as u64, acc2 + count))
}

fn count(pattern: &str, towels: &FxHashSet<&str>) -> u64 {
    let max_length = towels.iter()
        .map(|towel| towel.len())
        .max()
        .unwrap();
    count_mem(pattern, towels, &mut FxHashMap::default(), max_length)
}

fn count_mem<'a>(pattern: &'a str, towels: &FxHashSet<&str>, mem: &mut FxHashMap<&'a str, u64>, max_length: usize) -> u64 {
    let mut count = 0;
    if towels.contains(&pattern) {
        count += 1;
    }

    if mem.contains_key(pattern) {
        return mem[pattern];
    }

    for i in 1..cmp::min(pattern.len(), max_length+1) {
        if towels.contains(&&pattern[..i]) {
            count += count_mem(&pattern[i..], towels, mem, max_length);
        }
    }
    
    mem.insert(pattern, count);
    count
}

/* BRUTEFORCE DOESN'T GO BRRRRR
fn valid(pattern: &[char], towels:&[Vec<char>]) -> bool {
    //println!("{:?}", pattern);
    for towel in towels {
        if towel.len() > pattern.len() {
            continue;
        }

        if towel.len() == pattern.len() {
            if *towel == *pattern {
                //println!("YEP");
                return true;
            }
            continue;
        }

        // towel.len() < patter.len()

        if towel[..] == pattern[..towel.len()] && valid(&pattern[towel.len()..], towels) {
            return true;
        }
    }
    false
}
*/
