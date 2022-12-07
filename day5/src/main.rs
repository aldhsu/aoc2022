use anyhow::{Context, Error, Result};
use std::{collections::HashMap, str::FromStr};

struct Step {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Step {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        parts.next().context("can't get move")?;
        let count = parts.next().context("can't get count number")?.parse()?;
        parts.next().context("can't get from")?;
        let from = parts.next().context("can't get from number")?.parse()?;
        parts.next().context("can't get to")?;
        let to = parts.next().context("can't get to number")?.parse()?;

        Ok(Self { count, from, to })
    }
}

struct Map {
    inner: HashMap<usize, Vec<char>>,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines().rev();

        let mut inner: HashMap<usize, Vec<char>> = HashMap::new();
        iter.next().context("couldn't get columns")?;
        for line in iter {
            line.chars()
                .collect::<Vec<_>>()
                .chunks(4)
                .map(|s| s[1])
                .enumerate()
                .for_each(|(num, c)| {
                    if c.is_alphabetic() {
                        inner.entry(num + 1).or_default().push(c);
                    }
                });
        }

        Ok(Map { inner })
    }
}

struct World {
    map: Map,
    steps: Vec<Step>,
}
impl World {
    fn process_instructions(&mut self) -> Result<()> {
        for step in &self.steps {
            for _ in 0..step.count {
                let from_col = self
                    .map
                    .inner
                    .get_mut(&step.from)
                    .context("Couldn't get from column")?;
                let taken = from_col.pop().context("didn't get anything")?;

                let to_col: &mut Vec<char> = self
                    .map
                    .inner
                    .get_mut(&step.to)
                    .context("Couldn't get to column")?;
                to_col.push(taken)
            }
        }
        Ok(())
    }

    fn process_instructions2(&mut self) -> Result<()> {
        for step in &self.steps {
            let idx = self.map.inner[&step.from].len() - step.count;
            let from_col = self
                .map
                .inner
                .get_mut(&step.from)
                .context("Couldn't get from column")?;
            let mut taken = from_col.drain(idx..).collect::<Vec<_>>();
            let to_col: &mut Vec<char> = self
                .map
                .inner
                .get_mut(&step.to)
                .context("Couldn't get to column")?;
            to_col.append(&mut taken)
        }
        Ok(())
    }

    fn tops(&self) -> Result<Vec<char>> {
        (1..=9)
            .map(|num| {
                self.map
                    .inner
                    .get(&num)
                    .context("couldn't get column")
                    .map(|v| *v.iter().last().unwrap_or(&' '))
            })
            .collect()
    }
}

impl FromStr for World {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (map, steps) = s
            .split_once("\n\n")
            .context("couldn't find map and steps")?;
        let map = map.parse()?;
        let steps = steps
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<_>>>()?;

        Ok(World { map, steps })
    }
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    let mut world: World = input.parse()?;
    world.process_instructions()?;
    let part1 = world.tops()?.into_iter().collect::<String>();
    println!("part1: {:?}", part1);

    let mut world: World = input.parse()?;
    world.process_instructions2()?;
    let part2 = world.tops()?.into_iter().collect::<String>();
    println!("part2: {:?}", part2);

    Ok(())
}
