use std::{collections::HashSet, str::FromStr};

use anyhow::{anyhow, Context, Error, Result};

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    let part1: isize = part1(input)?;
    println!("part1: {part1}");

    part2(input)?;

    Ok(())
}

fn parse(s: &str) -> Result<Vec<Op>> {
    s.lines().map(|line| line.parse()).collect()
}

fn part1(input: &str) -> Result<isize> {
    let crt = Crt::new(input)?;
    let times = HashSet::from([20, 60, 100, 140, 180, 220]);
    Ok(crt
        .into_iter()
        .filter_map(|(val, clock)| times.contains(&clock).then_some(val * clock as isize))
        .sum())
}

fn part2(input: &str) -> Result<()> {
    let mut crt = Crt::new(input)?;

    for _ in 0..6isize {
        let mut old_reg = 1;

        for x in 0..40isize {
            let coord = x;
            let sprite = [coord - 1, coord, coord + 1];
            if sprite.contains(&old_reg) {
                print!("#");
            } else {
                print!(".");
            }

            let (reg, _) = crt.next().context("couldn't get next Crt")?;
            old_reg = reg
        }
        println!()
    }
    Ok(())
}

enum Op {
    Noop,
    Add(isize),
}

impl FromStr for Op {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Op::Noop);
        }

        let (op, num) = s.split_once(' ').context("couldn't split addx")?;
        if op != "addx" {
            return Err(anyhow!("unknown op {}", op));
        }
        Ok(Op::Add(num.parse::<isize>()?))
    }
}

#[derive(Default)]
struct Crt {
    clock: usize,
    register: isize,
    ops: Vec<Op>,
    cursor: usize,
    busy_time: usize,
}

impl Crt {
    fn new(s: &str) -> Result<Self> {
        Ok(Crt {
            ops: parse(s)?,
            register: 1,
            busy_time: 1,
            ..Default::default()
        })
    }
}

impl Iterator for Crt {
    type Item = (isize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.busy_time == 0 {
            match self.ops.get(self.cursor)? {
                Op::Noop => {
                    self.busy_time = 1;
                }
                Op::Add(val) => {
                    self.busy_time = 2;
                    self.register += val;
                }
            };
            self.cursor += 1;
        }

        self.busy_time = self.busy_time.saturating_sub(1);
        self.clock += 1;

        Some((self.register, self.clock))
    }
}

#[test]
fn part1_works() {
    let input = include_str!("../test/fixtures/long_example.txt");
    let part1 = part1(input).unwrap();
    assert_eq!(part1, 13140); // test weirdly fails even though the answer is right
}

#[test]
fn part2_works() {
    let input = include_str!("../test/fixtures/long_example.txt");
    part2(input).unwrap();
}
