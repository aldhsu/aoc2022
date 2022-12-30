#![feature(generic_arg_infer)]

use rayon::prelude::*;

mod parser;
use parser::parse_blueprints;

mod blueprint;
use blueprint::{BluePrint, BluePrintOpt};

mod resources;
use resources::*;

mod state;
use state::{State, RobotKind};

fn main() {
    let input = include_str!("../input.txt");
    // let part1 = part1(input);
    // println!("part1: {part1}");

    let part2 = part2(input);
    println!("part2: {part2}");
}

fn part1(s: &str) -> usize {
    let (_, bps) = parse_blueprints(s).expect("couldn't parse");
    bps.par_iter().map(BluePrint::optimise::<24>).sum()
}

fn part2(s: &str) -> usize {
    let (_, bps) = parse_blueprints(s).expect("couldn't parse");
    bps.par_iter().take(3).map(BluePrint::optimise2::<32>).sum()
}

#[test]
fn part1_test() {
    let input = r#"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."#;
    assert_eq!(part1(input), 33);
}
