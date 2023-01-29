use nom::branch::alt;
use nom::character::complete::char as c;
use nom::character::complete::digit1;
use nom::combinator::complete;
use nom::IResult;
use nom::multi::many1;

use crate::types::*;

pub fn parse_instructions(s: &str) -> IResult<&str, Vec<Instruction>> {
    complete(many1(alt((parse_move, parse_turn))))(s)
}

fn parse_move(s: &str) -> IResult<&str, Instruction> {
    let (s, d) = digit1(s)?;
    Ok((s, Instruction::Move(d.parse::<usize>().unwrap())))
}

fn parse_turn(s: &str) -> IResult<&str, Instruction> {
    let (s, dir) = alt((c('L'), c('R')))(s)?;
    let turn = match dir {
        'L' => Instruction::Left,
        'R' => Instruction::Right,
        _ => unreachable!(),
    };
    Ok((s, turn))
}

