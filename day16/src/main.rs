#![feature(hash_drain_filter)]

use std::collections::{HashMap, HashSet};

use petgraph::algo::astar;
use petgraph::prelude::*;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::{alpha1, newline};
use nom::combinator::complete;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    let part1 = part1(input);
    println!("part1: {part1}");

    let part2 = part2(input);
    println!("part2: {part2}");
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node<'a> {
    name: &'a str,
    rate: usize,
    tunnels: Vec<&'a str>,
}

impl<'a> From<&'a Node<'a>> for SimpleNode<'a> {
    fn from(value: &'a Node<'a>) -> Self {
        Self {
            name: value.name,
            rate: value.rate,
        }
    }
}

impl<'a> From<Node<'a>> for SimpleNode<'a> {
    fn from(value: Node<'a>) -> Self {
        Self {
            name: value.name,
            rate: value.rate,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
struct SimpleNode<'a> {
    name: &'a str,
    rate: usize,
}

#[test]
fn parse_node_test() {
    let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";
    let (_, node) = parse_node(input).unwrap();
    assert_eq!(
        node,
        Node {
            rate: 0,
            tunnels: vec!["DD", "II", "BB"],
            name: "AA"
        }
    );

    let input = "Valve RU has flow rate=19; tunnel leads to valve AB";
    let (_, node) = parse_node(input).unwrap();
    assert_eq!(
        node,
        Node {
            rate: 19,
            tunnels: vec!["AB"],
            name: "RU"
        }
    );
}
fn parse_node(s: &str) -> IResult<&str, Node> {
    let (s, (_, name, _, flow_rate, _, tunnels)) = tuple((
        tag("Valve "),
        alpha1,
        tag(" has flow rate="),
        digit1,
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        separated_list0(tag(", "), alpha1),
    ))(s)?;

    Ok((
        s,
        Node {
            name,
            rate: flow_rate.parse::<usize>().expect("can't parse rate"),
            tunnels,
        },
    ))
}

fn parse_map(s: &str) -> IResult<&str, Vec<Node>> {
    let (s, nodes) = complete(separated_list0(newline, parse_node))(s)?;

    Ok((s, nodes))
}

#[test]
fn part1_works() {
    let input = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;
    assert_eq!(part1(input), 1651);
}

#[test]
fn part2_works() {
    let input = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;
    assert_eq!(part2(input), 1707);
}

// path has to know when an action has opened the valve

fn process_input<'a>(
    s: &'a str,
) -> (
    HashSet<&str>,
    UnGraph<SimpleNode, usize>,
    HashMap<&str, NodeIndex>,
) {
    let (_, nodes) = parse_map(s).expect("couldn't parse map");
    let mut graph: UnGraph<SimpleNode, usize> = Default::default();
    let mut map = HashMap::new();
    let mut name_node_map: HashMap<&'a str, SimpleNode> = HashMap::new();

    for node in nodes.clone() {
        let index = graph.add_node(node.clone().into());
        map.insert(node.name, index);
        name_node_map.insert(node.name, node.into());
    }

    for node in nodes.clone() {
        let current_node = map.get(node.name).expect("couldn't get node");
        graph.extend_with_edges(node.tunnels.iter().map(|name| {
            let other = map.get(name).expect("couldn't get node");
            (*current_node, *other, 1)
        }))
    }

    let mut compressed_graph: UnGraph<SimpleNode, usize> = Default::default();
    let relevant_nodes = nodes
        .iter()
        .filter_map(|node| (node.rate > 0).then_some(node.name))
        .chain(std::iter::once("AA"))
        .collect::<HashSet<_>>();
    let mut compressed_map = HashMap::new();

    for node in relevant_nodes.clone() {
        let name_node = name_node_map.get(node).expect("couldn't get node");
        let index = compressed_graph.add_node(*name_node);
        compressed_map.insert(node, index);
    }

    for combo in relevant_nodes.clone().into_iter().combinations(2) {
        let start = map.get(combo[0]).expect("couldn't get start");
        let end = map.get(combo[1]).expect("couldn't get end");
        let Some((count, _)) = astar(&graph, *start, |n| n == *end, |_| 1, |_| 0) else { continue };

        let start = compressed_map.get(combo[0]).expect("couldn't get start");
        let end = compressed_map.get(combo[1]).expect("couldn't get end");
        compressed_graph.extend_with_edges([(*start, *end, count)]);
    }

    let name_to_node_map: HashMap<_, _> = compressed_graph
        .node_indices()
        .map(|index| {
            let node = compressed_graph[index];
            (node.name, index)
        })
        .collect();

    (relevant_nodes, compressed_graph, name_to_node_map)
}

fn part1(s: &str) -> usize {
    let (mut relevant_nodes, compressed_graph, name_to_node_map) = process_input(s);
    relevant_nodes.remove("AA");

    let state = DfsState {
        remaining_turns: 30,
        current_node_name: "AA",
        remaining_dest: relevant_nodes,
    };
    best_path_value(state, &compressed_graph, &name_to_node_map)
}

fn part2(s: &str) -> usize {
    let (mut relevant_nodes, compressed_graph, name_to_node_map) = process_input(s);
    relevant_nodes.remove("AA");

    let state = DfsState {
        remaining_turns: 26,
        current_node_name: "AA",
        remaining_dest: relevant_nodes,
    };
    best_path_value_2(state, &compressed_graph, &name_to_node_map)
}

struct DfsState<'a> {
    remaining_turns: usize,
    current_node_name: &'a str,
    remaining_dest: HashSet<&'a str>,
}

fn best_path_value<'a>(
    state: DfsState,
    graph: &'a UnGraph<SimpleNode<'a>, usize>,
    name_map: &'a HashMap<&'a str, NodeIndex>,
) -> usize {
    let current_index = *name_map
        .get(state.current_node_name)
        .expect("can't get the node");
    let current_node = graph[current_index];
    let node_value = current_node.rate * state.remaining_turns;

    let mut max_inner_value = 0;

    for inner_node_name in &state.remaining_dest {
        let index = name_map[inner_node_name];
        let edge = graph
            .find_edge(current_index, index)
            .unwrap_or_else(|| panic!("{inner_node_name}"));
        let travel_cost = graph.edge_weight(edge).expect("couldn't get edge weight");

        if (*travel_cost + 1) <= state.remaining_turns {
            let mut remaining_dest = state.remaining_dest.clone();
            remaining_dest.remove(inner_node_name);

            let next_state = DfsState {
                remaining_turns: state.remaining_turns - travel_cost - 1,
                current_node_name: inner_node_name,
                remaining_dest,
            };

            max_inner_value = max_inner_value.max(best_path_value(next_state, graph, name_map));
        }
    }

    max_inner_value + node_value
}

fn best_path_value_2<'a>(
    state: DfsState,
    graph: &'a UnGraph<SimpleNode<'a>, usize>,
    name_map: &'a HashMap<&'a str, NodeIndex>,
) -> usize {
    let current_index = *name_map
        .get(state.current_node_name)
        .expect("can't get the node");
    let current_node = graph[current_index];
    let node_value = current_node.rate * state.remaining_turns;

    let mut max_inner_value = 0;

    for inner_node_name in &state.remaining_dest {
        let index = name_map[inner_node_name];
        let edge = graph
            .find_edge(current_index, index)
            .expect("couldnt' get edge");
        let travel_cost = graph.edge_weight(edge).expect("couldn't get edge weight");

        if (*travel_cost + 1) <= state.remaining_turns {
            let mut remaining_dest = state.remaining_dest.clone();
            remaining_dest.remove(inner_node_name);

            let next_state = DfsState {
                remaining_turns: state.remaining_turns - travel_cost - 1,
                current_node_name: inner_node_name,
                remaining_dest,
            };

            max_inner_value = max_inner_value.max(best_path_value_2(next_state, graph, name_map));
        }
    }

    let mut elephant = 0;
    if max_inner_value == 0 {
        let state = DfsState {
            remaining_turns: 26,
            current_node_name: "AA",
            remaining_dest: state.remaining_dest,
        };
        elephant = best_path_value(state, graph, name_map);
    }

    max_inner_value + node_value + elephant
}
