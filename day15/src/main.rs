// use std::collections::HashSet;
// use std::fmt::Display;

use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::{consumed, recognize};
use nom::multi::{many0, separated_list0};
use nom::sequence::{preceded, tuple};
use nom::IResult;
use rayon::prelude::*;

fn main() {
    let input = include_str!("../input.txt");
    let part1 = part1(input, 2_000_000);
    println!("part1: {}", part1);

    let part2 = part2(input, 4_000_000).expect("couldn't find one");
    println!("part2: {}", part2);
}

type Coord = (isize, isize);

#[test]
fn parse_num_test() {
    let input = r#"-2"#;
    let (_, num) = parse_num(input).unwrap();
    assert_eq!(num, -2);
}

fn parse_num(s: &str) -> IResult<&str, isize> {
    let (s, num) = recognize(preceded(many0(tag("-")), digit1))(s)?;
    let num = num.parse::<isize>().expect("unable to parse");

    Ok((s, num))
}

#[test]
fn parse_coord_test() {
    let input = r#"x=-2, y=15"#;
    let (_, pair) = parse_coord(input).unwrap();
    assert_eq!(pair, (-2, 15));
}

fn parse_coord(s: &str) -> IResult<&str, Coord> {
    let (s, (_, x, _, y)) = tuple((tag("x="), parse_num, tag(", y="), parse_num))(s)?;

    Ok((s, (x, y)))
}

#[test]
fn parse_sensor_beacon_pair_test() {
    let input = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15"#;
    let (_, pair) = parse_sensor_beacon_pair(input).unwrap();
    assert_eq!(pair, ((2, 18), (-2, 15)));
}

fn parse_sensor_beacon_pair(s: &str) -> IResult<&str, (Coord, Coord)> {
    let (s, (_, sensor, _, beacon)) = tuple((
        tag("Sensor at "),
        parse_coord,
        tag(": closest beacon is at "),
        parse_coord,
    ))(s)?;

    Ok((s, (sensor, beacon)))
}

fn parse_input(s: &str) -> IResult<&str, Vec<(Coord, Coord)>> {
    separated_list0(newline, parse_sensor_beacon_pair)(s)
}

#[derive(Default, Debug)]
struct Map {
    sensors: Vec<Sensor>,
    empty: HashMap<isize, Vec<RangeInclusive<isize>>>,
}

impl Map {
    fn with_feed(feed: impl IntoIterator<Item = (Coord, Coord)>) -> Self {
        let mut map: Self = Default::default();

        for (sensor, beacon) in feed {
            let dist = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();

            map.sensors.push(Sensor {
                limit: dist,
                coord: sensor,
            });
            // go through - y to y
            for (i, y) in ((sensor.1 - dist)..=sensor.1).enumerate() {
                map.empty
                    .entry(y)
                    .or_default()
                    .push((sensor.0 - i as isize)..=(sensor.0 + i as isize))
            }
            for (i, y) in (sensor.1..(sensor.1 + dist)).rev().enumerate() {
                map.empty
                    .entry(y)
                    .or_default()
                    .push((sensor.0 - i as isize)..=(sensor.0 + i as isize))
            }
        }

        map
    }

    fn mash_ranges_together(ranges: &[RangeInclusive<isize>]) -> Vec<RangeInclusive<isize>> {
        fn helper(ranges: &[RangeInclusive<isize>]) -> Vec<RangeInclusive<isize>> {
            let mut set: Vec<RangeInclusive<isize>> = vec![];
            for range in ranges {
                let range = range.clone();

                let Some(other) = set.iter_mut().find(|other| {
                    joinable(other, &range)
                }) else { 
                    set.push(range);
                    continue 
                };

                let start = other.start().min(range.start());
                let end = other.end().max(range.end());
                let mut new = *start..=*end;

                std::mem::swap(other, &mut new);
            }

            set
        }
        let mut set = helper(ranges);

        if set.len() > 1 {
            let mut needs_work = true;

            while needs_work {
                needs_work = false;
                let new_set = helper(&set);
                if new_set.len() != set.len() {
                    needs_work = true;
                    set = new_set;
                }
            }
        }

        set
    }

    fn empty_at_row(&self, y: isize) -> usize {
        Self::mash_ranges_together(self.empty.get(&y).expect("couldn't get row"))
            .iter()
            .map(|range| range.end() - range.start() + 1) // because inclusive range have to add 1
            .sum::<isize>() as usize
    }

    fn has_gap(&self, y: isize) -> Option<Vec<RangeInclusive<isize>>> {
        let ranges = Self::mash_ranges_together(self.empty.get(&y).expect("couldn't get row"));
        (ranges.len() > 1).then_some(ranges)
    }
}

#[test]
fn joinable_test() {
    assert!(joinable(&(1..=3), &(4..=4)));
    assert!(joinable(&(5..=6), &(4..=4)));
    assert!(joinable(&(5..=6), &(5..=5)));
}

fn joinable(range: &RangeInclusive<isize>, other: &RangeInclusive<isize>) -> bool {
    other.contains(range.start())
        || other.contains(range.end())
        || range.contains(other.start())
        || range.contains(other.end())
        || *range.start() == other.end() + 1
        || *other.start() == range.end() + 1
}

fn part1(s: &str, row: isize) -> usize {
    let (unparsed, info) = parse_input(s).expect("couldn't parse");
    let map = Map::with_feed(info);
    map.empty_at_row(row)
}

fn part2(s: &str, end_row: isize) -> Option<usize> {
    let (unparsed, info) = parse_input(s).expect("couldn't parse");
    let map = Map::with_feed(info);

    let candidate = &map.sensors.par_iter().find_map_any(|sensor |{
        let mut candidate = None;

        for coord in sensor.outer_edge() {
            if coord.0 < 0 || coord.0 > end_row {
                continue;
            }
            if coord.1 < 0 || coord.1 > end_row {
                continue;
            }

            let all_out_of_range = map.sensors.iter().all(|other| {
                !other.in_range(&coord)
            });


            if all_out_of_range {
                candidate = Some(coord);
                break 
            }
        }
        candidate
    });

    Some(candidate.unwrap().0 as usize * 4_000_000 + candidate.unwrap().1 as usize)
}

#[test]
fn part1_works() {
    let input = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;
    assert_eq!(part1(input, 10), 26)
}

#[test]
fn part2_works() {
    let input = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;
    assert_eq!(part2(input, 20), Some(56_000_011))
}

#[derive(Debug, PartialEq, Eq)]
struct Sensor {
    limit: isize,
    coord: Coord,
}

impl Sensor {
    fn in_range(&self, other: &Coord) -> bool {
        ((self.coord.0 - other.0).abs() + (self.coord.1 - other.1).abs()) <= self.limit
    }

    fn edge(&self) -> impl Iterator<Item = Coord> + '_ {
        let top_half = ((self.coord.1 - self.limit)..=self.coord.1)
            .enumerate()
            .flat_map(|(i, y)| {
                [
                    (self.coord.0 - i as isize, y),
                    (self.coord.0 + i as isize, y),
                ]
            });
        let bottom_half = (self.coord.1..(self.coord.1 + self.limit))
            .rev()
            .enumerate()
            .flat_map(|(i, y)| {
                [
                    (self.coord.0 - i as isize, y),
                    (self.coord.0 + i as isize, y),
                ]
            });
        top_half.chain(bottom_half)
    }

    fn outer_edge(&self) -> impl Iterator<Item = Coord> + '_ {
        let limit = self.limit + 1;
        let top_half = ((self.coord.1 - limit)..=self.coord.1)
            .enumerate()
            .flat_map(|(i, y)| {
                [
                    (self.coord.0 - i as isize, y),
                    (self.coord.0 + i as isize, y),
                ]
            });
        let bottom_half = (self.coord.1..(self.coord.1 + limit))
            .rev()
            .enumerate()
            .flat_map(|(i, y)| {
                [
                    (self.coord.0 - i as isize, y),
                    (self.coord.0 + i as isize, y),
                ]
            });
        top_half.chain(bottom_half)
    }
}

#[test]
fn in_range_works() {
    let sensor = Sensor {
        coord: (8, 7),
        limit: 9,
    };
    assert!(sensor.in_range(&(7, 15)));
}
