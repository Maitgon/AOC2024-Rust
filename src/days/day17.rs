use crate::{Solution, SolutionPair};

#[derive(Debug, Clone)]
struct Computer {
    reg: [i64; 3],
    mem: Vec<i64>,
    p: usize
}

const ADV: i64 = 0;
const BXL: i64 = 1;
const BST: i64 = 2;
const JNZ: i64 = 3;
const BXC: i64 = 4;
const OUT: i64 = 5;
const BDV: i64 = 6;
const CDV: i64 = 7;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = Computer {
        reg: [37293246, 0, 0],
        //reg: [47910079998866, 0, 0],
        mem: vec![2,4,1,6,7,5,4,4,1,7,0,3,5,5,3,0],
        //mem: vec![0,3,5,4,3,0],
        p: 0
    };
    // Your solution here...
    let sol1: String = part1(input.clone()).into_iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let sol2: u64 = part2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(mut input: Computer) -> Vec<i64> {
    let mut output = Vec::new();
    //println!("{:?}", input);
    while input.p < input.mem.len() {
        match input.mem[input.p] {
            ADV => {
                input.reg[0] /= 1 << combo(input.mem[input.p+1], &input);
                input.p += 2;
            },

            BXL => {
                input.reg[1] ^= input.mem[input.p+1];
                input.p += 2;
            },

            BST => {
                input.reg[1] = combo(input.mem[input.p+1], &input) % 8;
                input.p += 2;
            },

            JNZ => {
                if input.reg[0] != 0 {
                    input.p = input.mem[input.p+1] as usize;
                } else {
                    input.p += 2;
                }
            },

            BXC => {
                input.reg[1] ^= input.reg[2];
                input.p += 2;
            },

            OUT => {
                output.push(combo(input.mem[input.p+1], &input));
                input.p += 2;
            },

            BDV => {
                input.reg[1] = input.reg[0] / (1 << combo(input.mem[input.p+1], &input));
                input.p += 2;
            }

            CDV => {
                input.reg[2] = input.reg[0] / (1 << combo(input.mem[input.p+1], &input));
                input.p += 2;
            }

            _ => unreachable!()
        }
        //println!("{:?}", input);
        //println!("{:?}", output);
    }

    //println!("{:?}", output);
    output
}

fn part2(input: &Computer) -> u64 {
    let mut actual = 0;

    for d in (0..input.mem.len()).rev() {
        for i in 1..10000 {
            let target = actual + (1 << (3*d)) * i;
            let mut new_input = input.clone();
            new_input.reg[0] = target;
            let output = part1(new_input);
            if output[d..] == input.mem[d..] {
                actual = target;
                break;
            }
        }
    }

    actual as u64
}

fn combo(op: i64, input: &Computer) -> i64 {
    if op <= 3 {
        op
    } else {
        input.reg[op as usize - 4] % 8
    }
}
