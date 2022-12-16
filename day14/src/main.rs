#![feature(array_windows)]

use std::collections::HashSet;
use std::fmt::Display;

use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::IResult;

fn main() {
    let input = include_str!("../input.txt");
    let part1 = part1(input);
    println!("part1: {part1}");

    let part2 = part2(input);
    println!("part2: {part2}");
}

fn part1(s: &str) -> usize {
    let (_, mut map) = parse_map(s).unwrap();
    while map.drop_sand() {}

    map.sand.len()
}

fn part2(s: &str) -> usize {
    let (_, mut map) = parse_map(s).unwrap();
    map.add_floor();
    while map.drop_sand() {}

    map.sand.len()
}

type Coord = (isize, isize);

#[derive(Debug, PartialEq, Eq, Default)]
struct Map {
    formations: HashSet<Coord>,
    sand: HashSet<Coord>,
    y_max: isize,
}

impl Map {
    const DROP_POINT: Coord = (500, 0);
    fn drop_sand(&mut self) -> bool {
        let mut current_position = Self::DROP_POINT;

        // move until stops
        loop {
            if current_position.1 > self.y_max {
                break false;
            }
            let y = current_position.1 + 1;
            let next_position = [
                (current_position.0, y),
                (current_position.0 - 1, y),
                (current_position.0 + 1, y),
            ]
            .into_iter()
            .find(|pos| !self.is_filled(pos));

            match next_position {
                Some(pos) => current_position = pos,
                None => {
                    self.sand.insert(current_position);

                    if current_position == Self::DROP_POINT {
                        break false;
                    } else {
                        break true;
                    }
                }
            }
        }
    }

    fn is_filled(&self, next_position: &Coord) -> bool {
        self.sand.contains(&next_position) || self.formations.contains(&next_position)
    }

    fn add_floor(&mut self) {
        let floor_y = self.y_max + 2;

        for x in -1000..=1000 {
            self.formations.insert((x, floor_y));
        }

        self.y_max = floor_y + 1;
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_max = self.formations.iter().max_by_key(|item| item.0).unwrap().0;
        let y_max = self.formations.iter().max_by_key(|item| item.1).unwrap().1;

        for y in 0..y_max {
            for x in 0..x_max {
                if self.sand.contains(&(x, y)) {
                    write!(f, "@")?;
                }
                if self.formations.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[test]
fn parse_map_works() {
    assert_eq!(
        parse_map("498,4 -> 498,6").unwrap(),
        (
            "",
            Map {
                formations: HashSet::from([(498, 4), (498, 5), (498, 6)]),
                y_max: 6,
                ..Default::default()
            }
        )
    )
}

fn parse_map(s: &str) -> IResult<&str, Map> {
    let (s, formations) = separated_list0(newline, parse_formation)(s)?;
    let mut inner = HashSet::new();
    let mut y_max = 0;
    for formation in formations {
        formation.array_windows().for_each(|[start, end]| {
            let x_diff = start.0 - end.0;
            if start.1 > y_max {
                y_max = start.1
            }
            if end.1 > y_max {
                y_max = end.1
            }

            if x_diff == 0 {
                let mut range = [start.1, end.1];
                range.sort();
                for y in range[0]..=range[1] {
                    inner.insert((start.0, y));
                }
            } else {
                let mut range = [start.0, end.0];
                range.sort();
                for x in range[0]..=range[1] {
                    inner.insert((x, start.1));
                }
            }
        });
    }

    Ok((
        s,
        Map {
            formations: inner,
            y_max,
            sand: Default::default(),
        },
    ))
}

type Formation = Vec<Coord>;

#[test]
fn parse_formation_works() {
    assert_eq!(
        parse_formation("498,4 -> 498,6 -> 496,6").unwrap(),
        ("", vec![(498, 4), (498, 6), (496, 6)])
    )
}

fn parse_formation(s: &str) -> IResult<&str, Formation> {
    let (s, ranges) = separated_list0(tag(" -> "), parse_coord)(s)?;
    Ok((s, ranges))
}

#[test]
fn parse_coord_works() {
    assert_eq!(parse_coord("498,4").unwrap(), ("", (498, 4)))
}

fn parse_coord(s: &str) -> IResult<&str, Coord> {
    let (s, (x, y)) = separated_pair(digit1, tag(","), digit1)(s)?;
    let x = x.parse::<isize>().expect("couldn't parse x");
    let y = y.parse::<isize>().expect("couldn't parse y");
    Ok((s, (x, y)))
}

#[test]
fn part1_works() {
    let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;
    assert_eq!(part1(input), 24);
}

#[test]
fn part2_works() {
    let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;
    assert_eq!(part2(input), 93);
}
