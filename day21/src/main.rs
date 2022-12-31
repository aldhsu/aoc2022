use std::collections::HashMap;

use anyhow::{anyhow, Context, Error, Result};

mod parser;
use parser::parse_input;

mod monkey;
use monkey::*;

fn main() -> Result<()> {
    let part1 = part1()?;
    println!("part1: {part1}");
    let part2 = part2()?;
    println!("part2: {part2}");

    Ok(())
}

fn part1() -> Result<i64> {
    let input = include_str!("../input.txt");
    let (_, monkeys) = parse_input(input).context("couldn't parse monkeys")?;
    let map: HashMap<&str, Monkey> = monkeys
        .into_iter()
        .map(|monkey| (monkey.name, monkey))
        .collect();
    let mut value_map: HashMap<&str, i64> = HashMap::new();

    compute_val("root", &map, &mut value_map)
}

fn part2() -> Result<i64> {
    let input = include_str!("../input.txt");
    let (_, monkeys) = parse_input(input).context("couldn't parse monkeys")?;
    let mut map: MonkeyMap = monkeys
        .into_iter()
        .map(|monkey| (monkey.name, monkey))
        .collect();

    let Monkey { action: Action::Op(compute), .. } = map.get_mut("root").context("couldn't get root")? else {
        return Err(anyhow!("wrong structure"))
    };
    compute.op = Op::Equal;

    let mut lower_bound = 1_000_000_000_000;
    let mut upper_bound = 10_000_000_000_000;

    let val = loop {
        let guess = (lower_bound + upper_bound) / 2;

        let mut value_map: HashMap<&str, i64> = HashMap::new();
        let human = map.get_mut("humn").context("couldn't get root")?;
        human.action = Action::Yell(guess);

        let left = compute_val("tlpd", &map, &mut value_map)?;
        let right = compute_val("jjmw", &map, &mut value_map)?;

        // binary search
        match left.cmp(&right) {
            std::cmp::Ordering::Less => upper_bound = guess,
            std::cmp::Ordering::Equal => break guess,
            std::cmp::Ordering::Greater => lower_bound = guess,
        }
    };

    Ok(val)
}

type MonkeyMap<'a> = HashMap<&'a str, Monkey<'a>>;

fn compute_val<'a>(
    name: &'a str,
    map: &'a MonkeyMap,
    value_map: &mut HashMap<&'a str, i64>,
) -> Result<i64> {
    if let Some(val) = value_map.get(name) {
        return Ok(*val);
    }
    let monkey = map.get(name).context("couldn't get monkey")?;

    let val = match &monkey.action {
        Action::Yell(val) => *val,
        Action::Op(Compute { left, right, op }) => {
            let left = compute_val(left, map, value_map)?;
            let right = compute_val(right, map, value_map)?;
            op.apply(left, right)
        }
    };
    value_map.insert(name, val);
    Ok(val)
}
