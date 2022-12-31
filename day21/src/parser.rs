use nom::branch::alt;
use nom::bytes::complete::tag;

use nom::character::complete::{alpha1, anychar, char as c, digit1, newline, one_of, space1};
use nom::combinator::all_consuming;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, tuple};
use nom::IResult;

use crate::monkey::*;

pub fn parse_input(s: &str) -> IResult<&str, Vec<Monkey>> {
    let (s, monkeys) = separated_list1(newline, parse_monkey)(s)?;
    Ok((s, monkeys))
}

#[test]
fn parse_monkey_op_test() {
    let input = "jqtt: tnwg * mbnq";
    let (s, monkey) = parse_monkey(input).unwrap();
    assert_eq!(monkey, Monkey {
        name: "jqtt",
        action: Action::Op(Compute {
            left: "tnwg",
            right: "mbnq",
            op: Op::Mul,
        }),
    })
}

#[test]
fn parse_monkey_yell_test() {
    let input = "ljqm: 14";
    let (s, monkey) = parse_monkey(input).unwrap();
    assert_eq!(monkey, Monkey {
        name: "ljqm",
        action: Action::Yell(14),
    })
}

fn parse_monkey(s: &str) -> IResult<&str, Monkey> {
    let (s, (name, _, action)) = tuple((alpha1, tag(": "), parse_action))(s)?;

    Ok((s, Monkey {
        name,
        action,
    }))
}
 
fn parse_action(s: &str) -> IResult<&str, Action> {
    let (s, action) = alt((parse_yell, parse_op))(s)?;
    Ok((s, action))
}

fn parse_yell(s: &str) -> IResult<&str, Action> {
    let (s, num) = digit1(s)?;

    let num = num.parse::<i64>().expect(&format!("couldn't parse yell {}", s));

    Ok((s, Action::Yell(num)))
}

fn parse_op(s: &str) -> IResult<&str, Action> {
    let (s, (left, _, op, _, right)) = tuple((
        alpha1,
        space1,
        anychar,
        space1,
        alpha1,
    ))(s)?;

    Ok((s,
    Action::Op(Compute {
        left,
        right,
        op: op.try_into().expect("unexpected operation"),
    })))
}
