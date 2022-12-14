use anyhow::{Context, Error, Result};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::multispace1;
use nom::character::complete::{multispace0, newline};
use nom::combinator::all_consuming;
use nom::multi::separated_list0;
use nom::sequence::{delimited, tuple};
use nom::IResult;

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    let (_, mut monkeys) = parse_monkeys(input)?;
    let part1 = part1(&mut monkeys);
    println!("part1: {part1}");
    let (_, mut monkeys) = parse_monkeys(input)?;
    let part2 = part2(&mut monkeys);
    println!("part2: {part2}");
    Ok(())
}

fn part1(monkeys: &mut [Monkey]) -> usize {
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let throws = monkeys[i].take_turn();

            for (idx, item) in throws {
                monkeys[idx].add_item(item)
            }
        }

    }

    monkeys.sort_by_key(|monkey| monkey.inspection_count);
    monkeys.reverse();

    monkeys.iter().take(2).map(|monkey| monkey.inspection_count as usize).product()
}

fn part2(monkeys: &mut [Monkey]) -> usize {
    for _ in 0..5 {
        for i in 0..monkeys.len() {
            let throws = monkeys[i].take_turn2();

            for (idx, item) in throws {
                monkeys[idx].add_item(item)
            }
        }

    }

    monkeys.sort_by_key(|monkey| monkey.inspection_count);
    monkeys.reverse();

    monkeys.iter().take(2).map(|monkey| monkey.inspection_count as usize).product()
}

struct Monkey {
    id: usize,
    items: Vec<usize>,
    op: Box<dyn Fn(usize) -> Option<usize>>,
    throw: Box<dyn Fn(usize) -> usize>,
    inspection_count: usize,
}

impl Monkey {
    fn take_turn(&mut self) -> Vec<(usize, usize)> {
        let items = std::mem::take(&mut self.items);

        items
            .into_iter()
            .map(|item| {
                self.inspection_count += 1;
                let worry = (self.op)(item).expect("Shouldn't overflow") / 3;

                let next_monkey = (self.throw)(worry);
                (next_monkey, worry)
            })
            .collect()
    }

    fn take_turn2(&mut self) -> Vec<(usize, usize)> {
        let items = std::mem::take(&mut self.items);

        items
            .into_iter()
            .map(|item| {
                self.inspection_count += 1;
                let worry = (self.op)(item).expect("Shouldn't overflow");

                let next_monkey = (self.throw)(worry);
                (next_monkey, worry)
            })
            .collect()
    }

    fn add_item(&mut self, item: usize) {
        self.items.push(item)
    }
}

#[test]
fn test_parse_monkey_ids() {
    let input = "Monkey 0:";
    assert_eq!(parse_monkey_id(input), Ok(("", 0)))
}

fn parse_monkey_id(s: &str) -> IResult<&str, usize> {
    let (s, (_, id, _)) = tuple((tag("Monkey "), digit1, tag(":")))(s)?;

    let id = id.parse::<usize>().expect("couldn't get id");

    Ok((s, id))
}

#[test]
fn test_parse_item() {
    let input = " 73";
    assert_eq!(parse_item(input), Ok(("", 73)))
}

fn parse_item(s: &str) -> IResult<&str, usize> {
    let (s, (_, digit)) = tuple((multispace0, digit1))(s)?;
    let digit = digit.parse::<usize>().expect("couldn't get item digit");
    Ok((s, digit))
}

#[test]
fn test_parse_items() {
    let input = "   Starting items: 99, 63, 76, 93, 54, 73";
    assert_eq!(parse_items(input), Ok(("", vec![99, 63, 76, 93, 54, 73])))
}

fn parse_items(s: &str) -> IResult<&str, Vec<usize>> {
    let (s, (_, _, items)) = tuple((
        multispace0,
        tag("Starting items: "),
        separated_list0(tag(","), parse_item),
    ))(s)?;

    Ok((s, items))
}

#[test]
fn test_parse_operator() {
    let input = "* 11";
    let (_, result_op) = parse_operator(input).unwrap();
    assert_eq!((result_op)(12).unwrap(), 132);

    let input = "* old";
    let (_, result_op) = parse_operator(input).unwrap();
    assert_eq!((result_op)(12).unwrap(), 144)
}

fn parse_operator(s: &str) -> IResult<&str, Box<dyn Fn(usize) -> Option<usize>>> {
    let (s, (operator, _, digit)) = tuple((
        alt((char('*'), char('+'))),
        multispace0,
        alt((digit1, tag("old"))),
    ))(s)?;

    let op = match operator {
        '*' => usize::checked_mul,
        '+' => usize::checked_add,
        _ => unreachable!("Unknown operator"),
    };

    let func: Box<dyn Fn(usize) -> Option<usize>> = if let Ok(digit) = digit.parse::<usize>() {
        Box::new(move |other: usize| (op)(digit, other))
    } else {
        Box::new(move |other: usize| (op)(other, other))
    };

    Ok((s, func))
}

#[test]
fn test_parse_operation() {
    let input = " Operation: new = old * 11";
    let (_, result_op) = parse_operation(input).unwrap();
    assert_eq!((result_op)(12).unwrap(), 132)
}

fn parse_operation(s: &str) -> IResult<&str, Box<dyn Fn(usize) -> Option<usize>>> {
    let (s, (_, _, op)) = tuple((multispace0, tag("Operation: new = old "), parse_operator))(s)?;

    Ok((s, Box::new(op)))
}

#[test]
fn test_parse_condition() {
    let input = "  Test: divisible by 2";
    let (_, result_op) = parse_condition(input).unwrap();
    assert!((result_op)(12))
}

fn parse_condition(s: &str) -> IResult<&str, Box<dyn Fn(usize) -> bool>> {
    let (s, (_, _, digit)) = tuple((multispace0, tag("Test: divisible by "), digit1))(s)?;

    let digit = digit.parse::<usize>().expect("couldn't get throw divisor");
    let func = move |other| (other % digit) == 0;

    Ok((s, Box::new(func)))
}

#[test]
fn test_parse_on() {
    let input = " If true: throw to monkey 7";
    let (_, result_op) = parse_on(input).unwrap();
    assert_eq!((result_op), 7)
}

fn parse_on(s: &str) -> IResult<&str, usize> {
    let (s, (_, _, digit)) = tuple((multispace0, tag("If true: throw to monkey "), digit1))(s)?;

    let digit = digit.parse::<usize>().expect("couldn't get throw divisor");

    Ok((s, digit))
}

#[test]
fn test_parse_off() {
    let input = "    If false: throw to monkey 1";
    let (_, result_op) = parse_off(input).unwrap();
    assert_eq!((result_op), 1)
}

fn parse_off(s: &str) -> IResult<&str, usize> {
    let (s, (_, _, digit)) = tuple((multispace0, tag("If false: throw to monkey "), digit1))(s)?;

    let digit = digit.parse::<usize>().expect("couldn't get throw divisor");

    Ok((s, digit))
}

#[test]
fn test_parse_throw() {
    let input = r#" Test: divisible by 2
    If true: throw to monkey 7
    If false: throw to monkey 1
"#;
    let (_, result_op) = parse_throw(input).unwrap();
    assert_eq!((result_op)(12), 7);
    assert_eq!((result_op)(13), 1);
}

fn parse_throw(s: &str) -> IResult<&str, Box<dyn Fn(usize) -> usize>> {
    let (s, (_, cond, is_on, is_off)) =
        tuple((multispace0, parse_condition, parse_on, parse_off))(s)?;

    let func = move |other| if (cond)(other) { is_on } else { is_off };

    Ok((s, Box::new(func)))
}

#[test]
fn test_parse_monkey() {
    let input = r#"Monkey 0:
  Starting items: 99, 63, 76, 93, 54, 73
  Operation: new = old * 11
  Test: divisible by 2
    If true: throw to monkey 7
    If false: throw to monkey 1"#;
    assert!(parse_monkey(input).is_ok())
}

fn parse_monkey(s: &str) -> IResult<&str, Monkey> {
    let (s, (id, items, op, throw)) =
        tuple((parse_monkey_id, parse_items, parse_operation, parse_throw))(s)?;

    Ok((
        s,
        Monkey {
            id,
            items,
            op,
            throw,
            inspection_count: 0,
        },
    ))
}

#[test]
fn test_parse_monkeys() {
    let input = include_str!("../input.txt");
    let (s, result) = parse_monkeys(input).unwrap();
    dbg!(s);
    assert_eq!(result.len(), 8)
}

fn parse_monkeys(s: &str) -> IResult<&str, Vec<Monkey>> {
    let (s, monkeys) = separated_list0(multispace1, parse_monkey)(s)?;

    Ok((s, monkeys))
}
