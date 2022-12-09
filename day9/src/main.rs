#![feature(array_windows)]
use anyhow::{anyhow, Context, Error, Result};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

enum Dir {
    Left,
    Down,
    Right,
    Up,
}

impl FromStr for Dir {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "L" => Dir::Left,
            "R" => Dir::Right,
            "U" => Dir::Up,
            "D" => Dir::Down,
            _ => return Err(anyhow!("couldn't parse direction")),
        })
    }
}

struct Move {
    dir: Dir,
    steps: usize,
}
impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, steps) = s.split_once(' ').context("couldn't get move")?;
        Ok(Self {
            dir: dir.parse()?,
            steps: steps.parse()?,
        })
    }
}

fn parse(s: &str) -> Result<Vec<Move>> {
    s.lines().map(|line| line.parse()).collect()
}

type Coord = (isize, isize);

struct Map<const N: usize> {
    knots: [Coord; N],
    seen: HashSet<Coord>,
}

impl<const N: usize> Default for Map<N> {
    fn default() -> Self {
        Self {
            knots: [(0, 0); N],
            seen: Default::default(),
        }
    }
}

impl<const N: usize> Map<N> {
    fn apply_moves(&mut self, moves: &[Move]) -> Result<()> {
        for m in moves {
            self.apply_move(m)?;
        }
        Ok(())
    }

    fn apply_move(&mut self, Move { dir, steps }: &Move) -> Result<()> {
        let index = core::array::from_fn::<_, N, _>(|i| i);
        for _ in 0..*steps {
            let mut head = self.knots.get_mut(0).context("couldn't get head")?;

            match dir {
                Dir::Left => head.0 -= 1,
                Dir::Down => head.1 += 1,
                Dir::Right => head.0 += 1,
                Dir::Up => head.1 -= 1,
            }

            for [head, tail] in index.array_windows() {
                let head = &self.knots[*head];
                let old_tail = &self.knots[*tail];
                let new_tail = calculate_tail_pos(old_tail, head);
                self.knots[*tail] = new_tail;
            }

            let tail = self.knots.last().context("couldn't get tail")?;
            self.seen.insert(*tail);
        }

        // Debug printing
        //
        // let map = self
        //     .knots
        //     .iter()
        //     .cloned()
        //     .enumerate()
        //     .map(|(num, coord)| (coord, num))
        //     .collect::<HashMap<Coord, usize>>();

        // for y in -25..25 {
        //     for x in -25..25 {
        //         if let Some(key) = map.get(&(x, y)) {
        //             print!("{key}");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }
        // println!();

        Ok(())
    }
}

fn calculate_tail_pos((x1, y1): &Coord, (x2, y2): &Coord) -> Coord {
    let difference = (x1 - x2).abs() + (y1 - y2).abs();
    if difference > 1 {
        if x1 == x2 {
            (*x1, (*y1 + *y2) / 2)
        } else if y1 == y2 {
            ((*x1 + *x2) / 2, *y1)
        } else if difference >= 3 {
            let x = if (x2 - x1).is_positive() {
                *x1 + 1
            } else {
                *x1 - 1
            };
            let y = if (y2 - y1).is_positive() {
                *y1 + 1
            } else {
                *y1 - 1
            };
            (x, y)
        } else {
            (*x1, *y1)
        }
    } else {
        (*x1, *y1)
    }
}

fn part1(input: &str) -> Result<usize> {
    let moves = parse(input)?;
    let mut map: Map<2> = Default::default();
    map.apply_moves(&moves)?;
    Ok(map.seen.len())
}

fn part2(input: &str) -> Result<usize> {
    let moves = parse(input)?;
    let mut map: Map<10> = Default::default();
    map.apply_moves(&moves)?;
    Ok(map.seen.len())
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    let part1 = part1(input)?;
    println!("part1: {part1}");

    let part2 = part2(input)?;
    println!("part2: {part2}");

    Ok(())
}

#[test]
fn part1_works() {
    let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;
    assert_eq!(part1(input).unwrap(), 13);
}

#[test]
fn part2_works_example1() {
    let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;
    assert_eq!(part2(input).unwrap(), 1);
}

#[test]
fn part2_works() {
    let input = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;
    assert_eq!(part2(input).unwrap(), 36);
}
