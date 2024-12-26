use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use rustc_hash::{FxHashMap, FxHashSet};

///////////////////////////////////////////////////////////////////////////////

type Graph = FxHashMap<String, Vec<String>>;

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/input23.txt").unwrap();
    let input: Vec<(&str, &str)> = input.split('\n').map(|line| line.split_once('-').unwrap()).collect();
    // Your solution here...
    let graph = build_graph(&input);
    let sol1: u64 = part1(&graph);
    let sol2: String = max_clique(&graph);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(graph: &FxHashMap<String, Vec<String>> ) -> u64 {
    let mut connected: FxHashSet<(String, String, String)> = FxHashSet::default();

    for (key, val) in graph {
        if !key.starts_with('t') {
            continue;
        }

        for v1 in val {
            for v2 in &graph[v1] {
                if v2 == key || !val.contains(v2) {
                    continue;
                }

                let mut sorted = [key.clone(), v1.clone(), v2.clone()];
                sorted.sort();

                connected.insert((sorted[0].clone(), sorted[1].clone(), sorted[2].clone()));
            }
        }
    }

    connected.len() as u64
}

fn build_graph(input: &Vec<(&str, &str)>) -> Graph {
    let mut graph: Graph = FxHashMap::default();

    for (from, to) in input {
        graph.entry(from.to_string()).or_default().push(to.to_string());
        graph.entry(to.to_string()).or_default().push(from.to_string());
    }

    graph
}

fn max_clique(graph: &Graph) -> String {
    let mut max_sol: FxHashSet<String> = FxHashSet::default();
    let mut used: FxHashSet<String> = FxHashSet::default();
    let mut candidates: FxHashSet<String> = graph.keys().cloned().collect();

    clique(&mut max_sol, &mut used, &mut candidates, graph);

    let mut max_sol = max_sol.iter().cloned().collect::<Vec<String>>();
    max_sol.sort();
    max_sol.join(",")
}

fn clique(max_sol: &mut FxHashSet<String>, used: &mut FxHashSet<String>, candidates: &mut FxHashSet<String>, graph: &Graph) {
    if used.len() > max_sol.len() {
        *max_sol = used.clone();
    }

    if used.len() + candidates.len() > max_sol.len() {
        for node in candidates.iter().cloned().collect::<Vec<String>>() {
            candidates.remove(&node);
            let mut new_used = used.clone();
            new_used.insert(node.clone());

            let adyacent_set: FxHashSet<String> = graph[&node].iter().cloned().collect();
            let new_candidates: FxHashSet<String> = candidates.intersection(&adyacent_set).cloned().collect();

            clique(max_sol, &mut new_used, &mut new_candidates.clone(), graph);
        }
    }
}

