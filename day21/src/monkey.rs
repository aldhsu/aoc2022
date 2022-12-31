use anyhow::{Error, Result, anyhow};

#[derive(Eq, PartialEq, Debug)]
pub enum Op {
    Sub,
    Mul,
    Add,
    Div,
    Equal,
}

impl TryFrom<char> for Op {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Op::*;

        Ok(match value {
            '*' => Mul,
            '+' => Add,
            '-' => Sub,
            '/' => Div,
            _ => return Err(anyhow!("unknown operator")),
        })
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Compute<'a> {
    pub left: &'a str,
    pub right: &'a str,
    pub op: Op,
}

impl Op {
    pub fn apply(&self, left: i64, right: i64) -> i64 {
        match self {
            Op::Sub => left - right,
            Op::Mul => left * right,
            Op::Add => left + right,
            Op::Div => left / right,
            Op::Equal => (left == right) as i64,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Action<'a> {
    Yell(i64),
    Op(Compute<'a>),
}

#[derive(Eq, PartialEq, Debug)]
pub struct Monkey<'a> {
    pub action: Action<'a>,
    pub name: &'a str,
}
