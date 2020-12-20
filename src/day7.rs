use regex::{Regex, Replacer};
use petgraph::algo::bellman_ford;
use petgraph::prelude::*;
use std::fs::read_to_string;
use std::iter::Sum;

struct Edge(String, String, f32);

pub fn day7<'a>(path: &str) {
    let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    // let input = read_to_string(path).unwrap();
    let re = Regex::new(r"([\w\s]+) bags contain ([\w\s,]+)").unwrap();
    let c_re = Regex::new(r"(\d+) ([\w\s]+) bag").unwrap();
    let mut edges = Vec::new();
    for line in input.lines() {
        let parts = re.captures(line).unwrap();
        let parent = parts.get(1).unwrap().as_str();
        let children = parts.get(2).unwrap().as_str();

        for child in c_re.captures_iter(children) {
            let child_name = child.get(2).unwrap().as_str();
            let weight = child.get(1).unwrap().as_str().parse::<i32>().unwrap();
            for i in 1..=weight {
                edges.push(Edge(parent.parse().unwrap(), child_name_with_index(child_name, i), 1.0))
            }
        }
    }

    // Solve - We want to know the cost of reaching all reachable nodes from shiny gold.
    let mut bag_graph = DiGraphMap::new();
    let shiny_gold_node = bag_graph.add_node("shiny gold");
    for edge in edges {
        println!("{}:{}", edge.0, edge.1);
        // bag_graph.add_edge(&*edge.0, &*edge.1, 1.0);
    }
    let path: Vec<f32> = bellman_ford(&bag_graph, shiny_gold_node).unwrap().0;
    println!("{:?}", path);
    let total_bags: f32 = path.into_iter().sum();
    println!("{}", total_bags);
    // for cost in path.iter().sum() {
    //     if *cost > 0.0 && !cost.is_infinite() {
    //         count += 1;
    //     }
    // }

    // println!("{:?}", count);
}

fn child_name_with_index(name: &str, index: i32) -> String {
    format!("{}{}", name, index)
}