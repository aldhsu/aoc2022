use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

use anyhow::{anyhow, Context, Error, Result};

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
    let input = r#"#E######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"#;
    assert_eq!(part1(input).unwrap(), 18)
}

fn part1(input: &str) -> Result<usize> {
    let map = input.parse::<Map>()?;
    let goal = map.goal();
    let mut memo = Memo { maps: vec![map] };
    let mut seen: HashSet<DfsState> = HashSet::new();

    let mut work = BinaryHeap::from([Reverse(DfsState {
        position: (1, 0),
        map: 0,
    })]);
    let mut answer = None;

    while let Some(Reverse(state)) = work.pop() {
        if !seen.insert(state) {
            continue;
        }

        if state.position == goal {
            answer = Some(state);
            break;
        }
        work.extend(state.next_moves(&mut memo).map(Reverse));
    }

    Ok(answer.context("couldn't find an answer")?.map - 1)
}

#[test]
fn part2_works() {
    let input = r#"#E######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"#;
    assert_eq!(part2(input).unwrap(), 54)
}

fn part2(input: &str) -> Result<usize> {
    let map = input.parse::<Map>()?;
    let start = (1, 0);
    let end = map.goal();
    let mut memo = Memo { maps: vec![map] };

    let start_to_end = dfs(
        DfsState {
            position: start,
            map: 0,
        },
        end,
        &mut memo,
    )?;
    let end_to_start = dfs(start_to_end, start, &mut memo)?;
    let start_to_end = dfs(end_to_start, end, &mut memo)?;

    Ok(start_to_end.map - 1)
}

fn dfs(start: DfsState, goal: Coord, mut memo: &mut Memo) -> Result<DfsState> {
    let mut seen: HashSet<DfsState> = HashSet::new();

    let mut work = BinaryHeap::from([Reverse(DfsState {
        position: start.position,
        map: start.map,
    })]);
    let mut answer = None;

    while let Some(Reverse(state)) = work.pop() {
        if !seen.insert(state) {
            continue;
        }

        if state.position == goal {
            answer = Some(state);
            break;
        }
        work.extend(state.next_moves(memo).map(Reverse));
    }

    answer.context("couldn't find an answer")
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct DfsState {
    position: Coord,
    map: usize,
}

impl PartialOrd for DfsState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // match self.position.partial_cmp(&other.position) {
        //     Some(core::cmp::Ordering::Equal) => {}
        //     ord => return ord,
        // }
        self.map.partial_cmp(&other.map)
    }
}

impl Ord for DfsState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

type Offset = (isize, isize);

impl DfsState {
    const OFFSETS: [Offset; 5] = [(-1, 0), (0, -1), (1, 0), (0, 1), (0, 0)];

    fn next_moves<'a>(&'a self, memo: &'a mut Memo) -> impl Iterator<Item = Self> + '_ {
        let next_map = memo.get_or_create(self.map + 1);

        Self::OFFSETS.iter().cloned().filter_map(|offset| {
            let x = self.position.0.checked_add_signed(offset.0)?;
            let y = self.position.1.checked_add_signed(offset.1)?;

            if x > next_map.max_x {
                return None;
            }
            if y > next_map.max_y {
                return None;
            }

            if next_map.inner.contains_key(&(x, y)) {
                return None;
            }
            Some(DfsState {
                position: (x, y),
                map: self.map + 1,
            })
        })
    }
}

#[derive(Debug)]
struct Memo {
    maps: Vec<Map>,
}

impl Memo {
    fn get_or_create(&mut self, num: usize) -> &Map {
        if self.maps.get(num).is_none() {
            let next_map = self.maps.last().unwrap().next();
            self.maps.push(next_map);
        }

        self.maps.get(num).unwrap()
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for String {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => "^",
            Direction::Down => "v",
            Direction::Left => "<",
            Direction::Right => ">",
        }
        .into()
    }
}

impl Direction {
    fn next_pos(&self, current: Coord) -> Coord {
        match self {
            Direction::Up => (current.0, current.1 - 1),
            Direction::Down => (current.0, current.1 + 1),
            Direction::Left => (current.0 - 1, current.1),
            Direction::Right => (current.0 + 1, current.1),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Blizzard(Direction),
    Wall,
}

impl Tile {
    fn next_tile(&self, current: Coord, map: &Map) -> Coord {
        match self {
            Tile::Blizzard(direction) => {
                let (mut new_x, mut new_y) = direction.next_pos(current);

                match direction {
                    Direction::Up if new_y == 0 => new_y = map.max_y - 2,
                    Direction::Down if new_y == map.max_y - 1 => new_y = 1,
                    Direction::Left if new_x == 0 => new_x = map.max_x - 2,
                    Direction::Right if new_x == map.max_x - 1 => new_x = 1,
                    _ => {}
                };

                (new_x, new_y)
            }
            _ => current,
        }
    }
}

type Coord = (usize, usize);

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '^' => Tile::Blizzard(Direction::Up),
            'v' => Tile::Blizzard(Direction::Down),
            '<' => Tile::Blizzard(Direction::Left),
            '>' => Tile::Blizzard(Direction::Right),
            '#' => Tile::Wall,
            _ => return Err(anyhow!("unknown")),
        })
    }
}

type Grid = HashMap<(usize, usize), Vec<Tile>>;

#[derive(Debug)]
struct Map {
    inner: Grid,
    max_x: usize,
    max_y: usize,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inner = Grid::new();
        let max_x = s.lines().next().unwrap().chars().count();
        let max_y = s.lines().count();

        for (coord, tile) in s.lines().enumerate().flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| Some(((x, y), c.try_into().ok()?)))
        }) {
            inner.entry(coord).or_default().push(tile)
        }

        Ok(Self {
            inner,
            max_y,
            max_x,
        })
    }
}

impl Map {
    fn next(&self) -> Self {
        let mut new_grid = Grid::with_capacity(self.inner.len());

        for (coord, tiles) in &self.inner {
            for tile in tiles {
                new_grid
                    .entry(tile.next_tile(*coord, self))
                    .or_default()
                    .push(*tile)
            }
        }

        Self {
            inner: new_grid,
            max_x: self.max_x,
            max_y: self.max_y,
        }
    }

    fn goal(&self) -> Coord {
        (self.max_x - 2, self.max_y)
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                let cell = if let Some(items) = self.inner.get(&(x, y)) {
                    if items.len() == 1 {
                        match items.first().unwrap() {
                            Tile::Blizzard(direction) => (*direction).into(),
                            Tile::Wall => "#".into(),
                        }
                    } else {
                        items.len().to_string()
                    }
                } else {
                    ".".into()
                };

                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
