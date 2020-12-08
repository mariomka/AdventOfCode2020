use std::collections::{BinaryHeap, HashMap};

use petgraph::graph::NodeIndex;
use petgraph::visit::{EdgeRef, VisitMap, Visitable};
use petgraph::{Directed, Graph};
use regex::{Captures, Regex};

#[derive(Debug)]
struct Bag<'a> {
    color: &'a str,
}

#[derive(Debug)]
struct Child {
    quantity: u8,
}

type BagGraph<'a> = Graph<Bag<'a>, Child, Directed>;

fn parse_bags<'a>(input: &'a Vec<&str>) -> BagGraph<'a> {
    lazy_static! {
        static ref REGEX_BAG: Regex =
            Regex::new(r"^([a-z]+ [a-z]+) bags contain (no other bags.)?").unwrap();
        static ref REGEX_CHILDREN: Regex =
            Regex::new(r"([0-9]+) ([a-z]+ [a-z]+) bags?[,.]").unwrap();
    }

    let mut graph: BagGraph = Graph::new();
    let mut nodes = HashMap::new();

    for line in input.iter() {
        let captures: Captures = REGEX_BAG.captures(line).unwrap();
        let color = captures.get(1).unwrap().as_str();
        let node = get_or_insert_node(&mut graph, &mut nodes, color);

        // Bag has children
        if captures.get(2).is_none() {
            for capture in REGEX_CHILDREN.captures_iter(line) {
                let quantity = capture.get(1).unwrap().as_str().parse::<u8>().unwrap();
                let color = capture.get(2).unwrap().as_str();
                let child_node = get_or_insert_node(&mut graph, &mut nodes, color);

                graph.add_edge(node, child_node, Child { quantity });
            }
        }
    }

    graph
}

fn get_or_insert_node<'a>(
    graph: &mut BagGraph<'a>,
    nodes: &mut HashMap<&'a str, NodeIndex>,
    color: &'a str,
) -> NodeIndex {
    if nodes.contains_key(color) {
        *nodes.get(color).unwrap()
    } else {
        let bag = Bag { color };
        let node = graph.add_node(bag);

        nodes.insert(color, node);

        node
    }
}

fn search_node_index(graph: &BagGraph, color: &str) -> NodeIndex {
    graph
        .node_indices()
        .find(|node| color == graph[*node].color)
        .unwrap()
}

pub fn part1(input: &Vec<&str>) -> u32 {
    let mut graph = parse_bags(input);
    graph.reverse();

    let mut visited = graph.visit_map();
    let mut count = 0;
    let mut visit_next = BinaryHeap::new();

    let initial_node = search_node_index(&graph, "shiny gold");
    visit_next.push(initial_node);

    while let Some(node) = visit_next.pop() {
        if visited.is_visited(&node) {
            continue;
        }

        visited.visit(node);
        count += 1;

        for edge in graph.edges(node) {
            let next = edge.target();
            visit_next.push(next);
        }
    }

    count - 1
}

pub fn part2(input: &Vec<&str>) -> u32 {
    let graph = parse_bags(input);

    let mut count = 0;
    let mut visit_next = BinaryHeap::new();

    let initial_node = search_node_index(&graph, "shiny gold");
    visit_next.push((1, initial_node));

    while let Some((multiplier, node)) = visit_next.pop() {
        for edge in graph.edges(node) {
            let next = edge.target();
            let times = multiplier * edge.weight().quantity as u32;
            visit_next.push((times, next));

            count += times;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    #[test]
    fn test_part1() {
        let input = "
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";

        assert_eq!(part1(&input_lines(input)), 4);
    }

    #[test]
    fn test_part2() {
        let input = "
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
";

        assert_eq!(part2(&input_lines(input)), 126);
    }
}
