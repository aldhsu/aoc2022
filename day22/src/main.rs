use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

use nom::branch::alt;
use nom::character::complete::char as c;
use nom::character::complete::digit1;
use nom::combinator::complete;
use nom::IResult;

use anyhow::{anyhow, Context, Error, Result};
use nom::multi::many1;

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    let part1 = part1(input)?;
    println!("part1: {part1}");

    Ok(())
}

fn part1(s: &str) -> Result<usize> {
    let (map, instructions) = s.split_once("\n\n").context("couldn't get parts")?;
    let map = map.parse::<Map>()?;
    let (_, instructions) = parse_instructions(instructions).unwrap();

    let mut cursor = Cursor {
        position: map.starting_point,
        facing: Facing::Right,
        history: vec![(map.starting_point, Facing::Right)],
    };

    for ins in &instructions {
        cursor.apply_instruction(ins, &map);
    }

    let history = cursor
        .history
        .iter()
        .cloned()
        .collect::<HashMap<Coord, Facing>>();

    for (y, line) in map.inner.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            match tile {
                Tile::Open => {
                    if let Some(facing) = history.get(&(x, y)) {
                        match facing {
                            Facing::Up => print!("^"),
                            Facing::Right => print!(">"),
                            Facing::Left => print!("<"),
                            Facing::Down => print!("v"),
                        }
                    } else {
                        print!(".");
                    };
                }
                Tile::Wall => print!("#"),
                Tile::None => print!(" "),
            }
        }
        println!()
    }

    Ok(cursor.score())
}

fn part2(s: &str) -> Result<usize> {
    let (map, instructions) = s.split_once("\n\n").context("couldn't get parts")?;
    let map = map.parse::<Map>()?;
    let (_, instructions) = parse_instructions(instructions).unwrap();

    let mut cursor = Cursor {
        position: map.starting_point,
        facing: Facing::Right,
        history: vec![(map.starting_point, Facing::Right)],
    };

    for ins in &instructions {
        cursor.apply_instruction(ins, &map);
    }

    let history = cursor
        .history
        .iter()
        .cloned()
        .collect::<HashMap<Coord, Facing>>();

    for (y, line) in map.inner.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            match tile {
                Tile::Open => {
                    if let Some(facing) = history.get(&(x, y)) {
                        match facing {
                            Facing::Up => print!("^"),
                            Facing::Right => print!(">"),
                            Facing::Left => print!("<"),
                            Facing::Down => print!("v"),
                        }
                    } else {
                        print!(".");
                    };
                }
                Tile::Wall => print!("#"),
                Tile::None => print!(" "),
            }
        }
        println!()
    }

    Ok(cursor.score())
}

fn parse_instructions(s: &str) -> IResult<&str, Vec<Instruction>> {
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

#[derive(Debug)]
enum Instruction {
    Move(usize),
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
    None,
}

impl TryInto<Tile> for char {
    type Error = Error;

    fn try_into(self) -> Result<Tile, Self::Error> {
        Ok(match self {
            '.' => Tile::Open,
            '#' => Tile::Wall,
            ' ' => Tile::None,
            _ => return Err(anyhow!("unknown tile {}", self)),
        })
    }
}

type Coord = (usize, usize);

struct Map {
    inner: Vec<Vec<Tile>>,
    x_max: usize,
    y_max: usize,
    starting_point: Coord,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x_max = s
            .lines()
            .map(|line| line.chars().count())
            .max()
            .ok_or(anyhow!("couldn't find x max"))?; // if first line is the longest
        let y_max = s.lines().count();

        let inner = s
            .lines()
            .map(|line| {
                let iter = line.chars().chain(std::iter::repeat(' '));
                iter.take(x_max)
                    .map(|c| c.try_into())
                    .collect::<Result<Vec<Tile>>>()
            })
            .collect::<Result<Vec<Vec<_>>>>()?;
        let starting_x = inner
            .first()
            .unwrap()
            .iter()
            .position(|t| t == &Tile::Open)
            .unwrap();

        Ok(Self {
            x_max,
            y_max,
            inner,
            starting_point: (starting_x, 0),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Default)]
enum Facing {
    Up,
    #[default]
    Right,
    Left,
    Down,
}

impl From<Facing> for usize {
    fn from(value: Facing) -> Self {
        match value {
            Facing::Up => 3,
            Facing::Right => 0,
            Facing::Left => 2,
            Facing::Down => 1,
        }
    }
}

#[derive(PartialEq, Eq, Default)]
struct Cursor {
    facing: Facing,
    position: Coord,
    history: Vec<(Coord, Facing)>,
}

impl std::fmt::Debug for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cursor")
            .field("facing", &self.facing)
            .field("position", &self.position)
            .finish()
    }
}

trait Mappable {
    fn next_pos(&self, pos: Coord, facing: Facing) -> Option<Coord>;
}

impl Cursor {
    fn score(&self) -> usize {
        (self.position.1 + 1) * 1000 + (self.position.0 + 1) * 4usize + usize::from(self.facing)
    }

    fn apply_instruction(&mut self, ins: &Instruction, map: &impl Mappable) {
        match ins {
            Instruction::Move(val) => self.apply_move(*val, map),
            Instruction::Left => self.turn_left(),
            Instruction::Right => self.turn_right(),
        }

        self.history.push((self.position, self.facing));
    }

    const TURNS: [Facing; 4] = [Facing::Up, Facing::Right, Facing::Down, Facing::Left];

    fn turn_left(&mut self) {
        let pos = Self::TURNS.iter().position(|f| f == &self.facing).unwrap();
        self.facing = match pos.checked_add_signed(-1) {
            Some(val) => Self::TURNS[val],
            None => Facing::Left,
        }
    }

    fn turn_right(&mut self) {
        let pos = Self::TURNS.iter().position(|f| f == &self.facing).unwrap();
        self.facing = Self::TURNS[(pos + 1) % Self::TURNS.len()]
    }

    fn apply_move(&mut self, number: usize, map: &impl Mappable) {
        for _ in 0..number {
            let Some(new_pos) = map.next_pos(self.position, self.facing) else { break };
            self.position = new_pos;
            self.history.push((self.position, self.facing));
        }
    }
}

type Offset = (isize, isize);

impl Map {
    fn wrap_coord(&self, pos: Coord, offset: Offset) -> Coord {
        let mut x = match pos.0.checked_add_signed(offset.0) {
            Some(v) => v,
            None => self.x_max - 1,
        };

        if x == self.x_max {
            x = 0;
        }

        let mut y = match pos.1.checked_add_signed(offset.1) {
            Some(v) => v,
            None => self.y_max - 1,
        };

        if y == self.y_max {
            y = 0;
        }

        (x, y)
    }

    fn wrap_position(&self, pos: Coord, facing: Facing) -> Option<Coord> {
        match self.inner.get(pos.1)?.get(pos.0)? {
            Tile::Open => Some(pos),
            Tile::Wall => None,
            Tile::None => self.next_valid_pos(pos, facing),
        }
    }

    fn next_valid_pos(&self, pos: Coord, facing: Facing) -> Option<Coord> {
        fn match_tile((val, tile): (usize, &Tile)) -> Option<Option<usize>> {
            match tile {
                Tile::None => None,
                Tile::Wall => Some(None),
                Tile::Open => Some(Some(val)),
            }
        }
        // assumes no overlapping sections
        match facing {
            Facing::Up => {
                let y = self
                    .inner
                    .iter()
                    .enumerate()
                    .rev()
                    .find_map(|(y, line)| match_tile((y, line.get(pos.0)?)))??;
                Some((pos.0, y))
            }
            Facing::Right => {
                let x = self
                    .inner
                    .get(pos.1)?
                    .iter()
                    .enumerate()
                    .find_map(match_tile)??;
                Some((x, pos.1))
            }
            Facing::Left => {
                let x = self
                    .inner
                    .get(pos.1)?
                    .iter()
                    .enumerate()
                    .rev()
                    .find_map(match_tile)??;
                Some((x, pos.1))
            }
            Facing::Down => {
                let y = self
                    .inner
                    .iter()
                    .enumerate()
                    .find_map(|(y, line)| match_tile((y, line.get(pos.0)?)))??;
                Some((pos.0, y))
            }
        }
    }

}

impl Mappable for Map {
    fn next_pos(&self, pos: Coord, facing: Facing) -> Option<Coord> {
        let new_pos = self.wrap_coord(
            pos,
            match facing {
                Facing::Up => (0, -1),
                Facing::Right => (1, 0),
                Facing::Left => (-1, 0),
                Facing::Down => (0, 1),
            },
        );
        self.wrap_position(new_pos, facing)
    }
}

#[test]
fn next_pos_works() {
    let map = Map {
        x_max: 2,
        y_max: 2,
        inner: vec![vec![Tile::None, Tile::Open], vec![Tile::Open, Tile::None]],
        starting_point: (1, 0),
    };

    // none skipping works
    assert_eq!(map.next_pos((1, 0), Facing::Right), Some((1, 0)));
    assert_eq!(map.next_pos((1, 0), Facing::Up), Some((1, 0)));
    assert_eq!(map.next_pos((1, 0), Facing::Left), Some((1, 0)));
    assert_eq!(map.next_pos((1, 0), Facing::Down), Some((1, 0)));

    assert_eq!(map.next_pos((0, 1), Facing::Right), Some((0, 1)));
    assert_eq!(map.next_pos((0, 1), Facing::Up), Some((0, 1)));
    assert_eq!(map.next_pos((0, 1), Facing::Left), Some((0, 1)));
    assert_eq!(map.next_pos((0, 1), Facing::Down), Some((0, 1)));

    //regular works
    let map = Map {
        x_max: 2,
        y_max: 2,
        inner: vec![vec![Tile::Open, Tile::Open], vec![Tile::Open, Tile::None]],
        starting_point: (1, 0),
    };
    assert_eq!(map.next_pos((1, 0), Facing::Right), Some((0, 0)));
    assert_eq!(map.next_pos((1, 0), Facing::Left), Some((0, 0)));

    // walls work
    let map = Map {
        x_max: 2,
        y_max: 4,
        inner: vec![
            vec![Tile::None, Tile::Open],
            vec![Tile::Wall, Tile::Open],
            vec![Tile::Open, Tile::Open],
            vec![Tile::None, Tile::Open],
        ],
        starting_point: (1, 0),
    };
    assert_eq!(map.next_pos((0, 2), Facing::Down), None);
}

#[test]
fn failing() {
    let map = Map {
        x_max: 2,
        y_max: 3,
        inner: vec![
            vec![Tile::Open, Tile::Open],
            vec![Tile::Open, Tile::Open],
            vec![Tile::None, Tile::Open],
        ],
        starting_point: (1, 0),
    };
    assert_eq!(map.next_pos((0, 2), Facing::Down), Some((0, 0)));
}

#[test]
fn part1_works() {
    let input = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"#;
    assert_eq!(part1(input).unwrap(), 6032);
}

#[test]
fn weird_part1_works() {
    let input = include_str!("../input.txt");
    let (map, _) = input.split_once("\n\n").unwrap();
    let map = map.parse::<Map>().unwrap();

    // assert_eq!(map.next_pos((117, 49), Facing::Down), Some((117, 0)));
    assert_eq!(map.next_pos((125, 0), Facing::Up), Some((125, 49)));
}

struct Cube {
    inner: [Vec<Vec<Tile>>; 6],
    starting_point: Coord,
}

impl FromStr for Cube {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let starting_point = (0, 0);
        let inner = Default::default();
        let chars = s.chars().filter(|&c| c == '.' || c == '#').count();
        let side = ((chars / 6) as f64).sqrt() as usize;
        let map = s.parse::<Map>()?;

        Ok(Self {
            inner,
            starting_point
        })
    }
}
