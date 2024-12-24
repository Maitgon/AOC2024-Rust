use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use rustc_hash::{FxHashMap, FxHashSet};
use rayon::prelude::*;
use std::sync::Mutex;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/input22.txt").unwrap();
    let input: Vec<u64> = input.lines().map(|line| line.parse::<u64>().unwrap()).collect();
    // Your solution here...
    let (sol1, sol2) = both(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn both(input: &Vec<u64>) -> (u64, u64) {
    let sol1 = Mutex::new(0);
    let secret_numbers: Mutex<Vec<Vec<u64>>> = Mutex::new(Vec::new());
    input.par_iter().for_each( |n|{
        let mut n = *n;
        let mut secrets: Vec<u64> = vec![n % 10];

        for i in 0..2000 {
            n = next_secret_number(n);
            if i == 1999 {
                *sol1.lock().unwrap() += n;
            }
            secrets.push(n % 10);
        } 
        //println!("Secrets: {:?}", secrets);
        let mut s = secret_numbers.lock().unwrap();
        s.push(secrets);
    });

    let mut seqs: FxHashMap<(u64, u64, u64, u64), u64> = FxHashMap::default();

    let secret_numbers = secret_numbers.lock().unwrap();

    //println!("Secret numbers: {:?}", secret_numbers);

    for vals in secret_numbers.iter() { // cant paralelice this, idk why
        let mut seqs_used: FxHashSet<(u64, u64, u64, u64)> = FxHashSet::default();
        for i in 0..vals.len() - 4 {
            if let [a, b, c, d, e] = vals[i..i+5] {
                let seq = (b-a, c-b, d-c, e-d);
                if !seqs_used.insert(seq) {
                    continue;
                }
                
                *seqs.entry(seq).or_insert(0) += e;
            }
        }
    }

    let sol1_result = *sol1.lock().unwrap(); // ok compiler, WTF???

    (sol1_result, *seqs.values().max().unwrap())
}

/*
fn both(input: &Vec<u64>) -> (u64, u64) {
    let mut sol1 = 0;
    let mut secret_numbers: Vec<Vec<u64>> = Vec::new();
    for n in input {
        let mut n = *n;
        let mut secrets: Vec<u64> = vec![n % 10];

        for i in 0..2000 {
            n = next_secret_number(n);
            if i == 1999 {
                sol1 += n;
            }
            secrets.push(n % 10);
        }

        secret_numbers.push(secrets);
    }

    let mut seqs: FxHashMap<(u64, u64, u64, u64), u64> = FxHashMap::default();

    for vals in secret_numbers {
        let mut seqs_used: FxHashSet<(u64, u64, u64, u64)> = FxHashSet::default();
        for i in 0..vals.len() - 4 {
            if let [a, b, c, d, e] = vals[i..i+5] {
                let seq = (b-a, c-b, d-c, e-d);
                if !seqs_used.insert(seq) {
                    continue;
                }
                *seqs.entry(seq).or_insert(0) += e;
            }
        }
    }

    (sol1, *seqs.values().max().unwrap())
}
*/
fn next_secret_number(mut n: u64) -> u64 {
    n = (n ^ (n << 6)) & 0xFFFFFF;
    n = (n ^ (n >> 5)) & 0xFFFFFF;
    (n ^ (n << 11)) & 0xFFFFFF
}