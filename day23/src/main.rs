use anyhow::{Error, Result};
use std::{
    array,
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    let part1 = part1(input)?;
    println!("part1: {part1}");

    let part2 = part2(input)?;
    println!("part2: {part2}");

    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    let mut map = input.parse::<Map>()?;

    for _ in 0..10 {
        map = map.next_tick().unwrap();
    }

    Ok(map.ground_covered())
}

fn part2(input: &str) -> Result<usize> {
    let mut map = input.parse::<Map>()?;

    let mut i = 0;
    loop {
        let Some(new_map) = map.next_tick() else { break };
        map = new_map;
        i += 1;
    }

    Ok(i)
}

type Coord = (isize, isize);
type Grid = HashSet<Coord>;
struct Map {
    inner: HashSet<Coord>,
    order: [Directionable; 4],
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(x, c)| matches!(c, '#').then(|| (x as isize, y as isize)))
            })
            .collect();

        Ok(Self {
            inner,
            order: [northable, southable, westable, eastable],
        })
    }
}

const OFFSETS: [Coord; 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn get_all_positions(coord: Coord, map: &Grid) -> [bool; 8] {
    array::from_fn(|i| {
        let offset = OFFSETS[i];
        let pos = (coord.0 + offset.0, coord.1 + offset.1);
        map.contains(&pos)
    })
}
type Neighbours = [bool; 8];

fn northable(coord: Coord, neighbours: &Neighbours) -> Option<Coord> {
    matches!(neighbours, [false, false, false, ..]).then(|| (coord.0, coord.1 - 1))
}

fn southable(coord: Coord, neighbours: &Neighbours) -> Option<Coord> {
    matches!(neighbours, [.., false, false, false]).then(|| (coord.0, coord.1 + 1))
}

fn westable(coord: Coord, neighbours: &Neighbours) -> Option<Coord> {
    matches!(neighbours, [false, _, _, false, _, false, ..]).then(|| (coord.0 - 1, coord.1))
}

fn eastable(coord: Coord, neighbours: &Neighbours) -> Option<Coord> {
    matches!(neighbours, [_, _, false, _, false, _, _, false]).then(|| (coord.0 + 1, coord.1))
}
type Directionable = fn(Coord, &Neighbours) -> Option<Coord>;

fn decide_move(coord: Coord, map: &Grid, move_list: [Directionable; 4]) -> Option<Coord> {
    let neighbours = get_all_positions(coord, map);

    if matches!(
        neighbours,
        [false, false, false, false, false, false, false, false]
    ) {
        return None;
    }
    move_list
        .into_iter()
        .find_map(|f| (f)(coord, &neighbours))
        .or(Some(coord))
}

impl Map {
    fn next_tick(&self) -> Option<Self> {
        let mut work = false;
        let mut new: HashMap<Coord, Vec<Coord>> = HashMap::new();

        for elf in &self.inner {
            let next_pos = if let Some(next_pos) = decide_move(*elf, &self.inner, self.order) {
                work = true;
                next_pos
            } else {
                *elf
            };
            new.entry(next_pos).or_default().push(*elf)
        }

        if !work { return None }

        let mut inner = HashSet::new();
        for (k, v) in new.into_iter() {
            if v.len() == 1 {
                inner.insert(k);
            } else {
                inner.extend(v.into_iter())
            }
        }

        let mut order = self.order;
        order.rotate_left(1);

        Some(Self { inner, order })
    }

    fn ground_covered(&self) -> usize {
        let mut min_x = isize::max_value();
        let mut max_x = 0isize;
        let mut min_y = isize::max_value();
        let mut max_y = 0isize;

        for (x, y) in &self.inner {
            if x < &min_x {
                min_x = *x;
            }
            if y < &min_y {
                min_y = *y;
            }
            if x > &max_x {
                max_x = *x;
            }
            if y > &max_y {
                max_y = *y;
            }
        }

        let mut total = 0;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if !self.inner.contains(&(x, y)) {
                    total += 1;
                }
            }
        }

        total
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut min_x = isize::max_value();
        let mut max_x = 0isize;
        let mut min_y = isize::max_value();
        let mut max_y = 0isize;

        for (x, y) in &self.inner {
            if x < &min_x {
                min_x = *x;
            }
            if y < &min_y {
                min_y = *y;
            }
            if x > &max_x {
                max_x = *x;
            }
            if y > &max_y {
                max_y = *y;
            }
        }

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.inner.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        // writeln!(f, "{:?}", self.order);
        Ok(())
    }
}

#[test]
fn example() {
    let input = r#".....
..##.
..#..
.....
..##.
....."#;
    let map = input.parse::<Map>().unwrap();
    println!("{map}");

    let next = map.next_tick().unwrap();
    println!("{next}");

    let next = next.next_tick().unwrap();
    println!("{next}");

    let next = next.next_tick().unwrap();
    println!("{next}");
}

#[test]
fn big_example() {
    let input = r#"..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
.............."#;
    assert_eq!(part1(input).unwrap(), 110)
}

#[test]
fn part2_example() {
    let input = r#"..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
.............."#;
    assert_eq!(part2(input).unwrap(), 20)
}
