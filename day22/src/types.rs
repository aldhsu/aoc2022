use anyhow::{anyhow, Error, Result};
use std::{collections::HashMap, fmt::Display, str::FromStr};

pub struct Cube {
    pub inner: Vec<Vec<Tile>>,
    pub starting_point: Coord,
    face_map: HashMap<Coord, Face>,
    sides: HashMap<Face, Side>,
}

impl Cube {
    fn move_facing(&self, pos: Coord, facing: Facing) -> Option<(Coord, Facing)> {
        let offset = match facing {
            Facing::Up => (0, -1),
            Facing::Right => (1, 0),
            Facing::Left => (-1, 0),
            Facing::Down => (0, 1),
        };
        if let Some(pos) = self.basic_movement(pos, offset) {
            Some((pos?, facing))
        } else {
            self.advanced_movement(pos, facing)
        }
    }

    /// return first option is basic movement can't be done
    /// second option is if hit a wall
    fn basic_movement(&self, pos: Coord, offset: Offset) -> Option<Option<Coord>> {
        let pos = (
            pos.0.checked_add_signed(offset.0)?,
            pos.1.checked_add_signed(offset.1)?,
        );

        match self.inner.get(pos.1)?.get(pos.0)? {
            Tile::Open => Some(Some(pos)),
            Tile::Wall => Some(None),
            Tile::None => None,
        }
    }

    /// return None when hits wall
    fn advanced_movement(&self, pos: Coord, facing: Facing) -> Option<(Coord, Facing)> {
        let face = self
            .face_map
            .get(&pos)
            .expect("couldn't get face when doing advanced movement");
        let side = self.sides.get(face).expect("couldn't get side");

        let (next_coord, next_face, next_facing) = side.travel_border(facing, &self.sides, pos);
        let tile = self
            .inner
            .get(next_coord.1)
            .expect("get advanceds y")
            .get(next_coord.0)
            .expect("get advanced x");

        match tile {
            Tile::Open => Some((next_coord, next_facing)),
            Tile::Wall => None,
            Tile::None => panic!("advanced should never index into no tiles {next_coord:?}"),
        }
    }

    fn next_pos(&self, pos: Coord, facing: Facing) -> Option<(Coord, Facing)> {
        self.move_facing(pos, facing)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Face {
    Top,
    Down,
    Left,
    Right,
    Back,
    Front,
}

impl Face {
    fn next_face(&self, facing: Facing) -> (Face, Facing) {
        // - T R
        // - F -
        // L D -
        // B
        match (self, facing) {
            (Face::Top, Facing::Up) => (Face::Back, Facing::Right),
            (Face::Top, Facing::Left) => (Face::Left, Facing::Right), // reversed
            (Face::Down, Facing::Right) => (Face::Right, Facing::Left), //reversed
            (Face::Down, Facing::Down) => (Face::Back, Facing::Left),
            (Face::Left, Facing::Up) => (Face::Front, Facing::Right),
            (Face::Left, Facing::Left) => (Face::Top, Facing::Right), //reversed
            (Face::Right, Facing::Up) => (Face::Back, Facing::Up), //reversed
            (Face::Right, Facing::Right) => (Face::Down, Facing::Left), // reversed?
            (Face::Right, Facing::Down) => (Face::Front, Facing::Left), // rotate
            (Face::Back, Facing::Right) => (Face::Down, Facing::Up), //rotate
            (Face::Back, Facing::Left) => (Face::Top, Facing::Down), //rotate
            (Face::Back, Facing::Down) => (Face::Right, Facing::Down),
            (Face::Front, Facing::Right) => (Face::Right, Facing::Up), //rotate
            (Face::Front, Facing::Left) => (Face::Left, Facing::Down), // rotate
            _ => unreachable!("should be handled already")
        }
    }

    fn next_coord(&self, coord: Coord, facing: Facing, face: Face) -> Coord {
        todo!()
    }
}

#[derive(Debug)]
struct Side {
    face: Face,
    inner: Cell,
}

type MapInner = Vec<Vec<Tile>>;

impl Side {
    fn travel_border(
        &self,
        facing: Facing,
        sides: &HashMap<Face, Side>,
        coord: Coord,
    ) -> (Coord, Face, Facing) {
        let (next_face, next_facing) = self.face.next_face(facing);

        let border = self.border(facing);

        let rot = Cursor::TURNS
            .iter()
            .position(|turn| *turn == facing)
            .expect("couldn't get first facing");
        let next_rot = Cursor::TURNS
            .iter()
            .position(|turn| *turn == next_facing)
            .expect("couldn't get next facing");
        let rotation = (next_rot as isize - rot as isize).rem_euclid(Cursor::TURNS.len() as isize); //dubious
                                                                                                    //
        let other_side = sides.get(&next_face).expect("unable to get other side");
        let mut other_border = other_side.border(next_facing.opposite());

        if rotation == 2 {
            other_border.reverse();
        }

        let next_coord = border
            .iter()
            .zip(other_border)
            .find_map(|(left, right)| (*left == coord).then_some(right))
            .expect(&format!("couldn't find next coord {border:?}, {coord:?}"));

        (next_coord, next_face, next_facing)
    }

    fn border(&self, facing: Facing) -> Vec<Coord> {
        match facing {
            Facing::Up => {
                let y = self.inner.0 .1;
                let x_start = self.inner.0 .0;
                let x_end = self.inner.1 .0;
                (x_start..x_end).map(move |x| (x, y)).collect()
            }
            Facing::Right => {
                let x = self.inner.1 .0 - 1;
                let y_start = self.inner.0 .1;
                let y_end = self.inner.1 .1;
                (y_start..y_end).map(move |y| (x, y)).collect()
            }
            Facing::Left => {
                let x = self.inner.0 .0;
                let y_start = self.inner.0 .1;
                let y_end = self.inner.1 .1;
                (y_start..y_end).map(move |y| (x, y)).collect()
            }
            Facing::Down => {
                let y = self.inner.1 .1 - 1;
                let x_start = self.inner.0 .0;
                let x_end = self.inner.1 .0;
                (x_start..x_end).map(move |x| (x, y)).collect()
            }
        }
    }
}

type Cell = (Coord, Coord);

pub struct CubeCursor {
    pub facing: Facing, // relative to current face
    pub coord: Coord,
    pub history: Vec<(Coord, Facing)>,
}

impl CubeCursor {
    pub fn score(&self) -> usize {
        (self.coord.1 + 1) * 1000 + (self.coord.0 + 1) * 4usize + usize::from(self.facing)
    }

    pub fn apply_instruction(&mut self, ins: &Instruction, map: &Cube) {
        match ins {
            Instruction::Move(val) => self.apply_move(*val, map),
            Instruction::Left => self.turn_left(),
            Instruction::Right => self.turn_right(),
        }

        self.history.push((self.coord, self.facing));
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

    fn apply_move(&mut self, number: usize, map: &Cube) {
        for _ in 0..number {
            let Some((new_pos, facing)) = map.move_facing(self.coord, self.facing) else { break };
            self.coord = new_pos;
            self.facing = facing;
            self.history.push((self.coord, self.facing));
        }
    }
}

impl FromStr for Cube {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().filter(|&c| c == '.' || c == '#').count();
        let side_len = ((chars / 6) as f64).sqrt() as usize;
        let map = s.parse::<Map>()?;
        // chunk it into grids (Coord, Coord)
        let grid_cells: Vec<(Coord, Coord)> = (0..map.y_max)
            .step_by(side_len)
            .flat_map(|y_chunk| {
                (0..map.x_max).step_by(side_len).map(move |x_chunk| {
                    ((x_chunk, y_chunk), (x_chunk + side_len, y_chunk + side_len))
                })
            })
            .collect();
        let starting_point = map.starting_point;

        fn grid_index_to_face(index: usize) -> Option<Face> {
            // - 1 2
            // - 4 -
            // 6 7 -
            // 9
            //
            // - T R
            // - F -
            // L D -
            // B
            Some(match index {
                1 => Face::Top,
                2 => Face::Right,
                4 => Face::Front,
                6 => Face::Left,
                7 => Face::Down,
                9 => Face::Back,
                _ => return None,
            })
        }

        let face_map: HashMap<Coord, Face> = grid_cells
            .iter()
            .enumerate()
            .filter_map(|(count, cell)| {
                let face = grid_index_to_face(count)?;
                let ((x_start, y_start), (x_end, y_end)) = *cell;
                Some(
                    (y_start..y_end) // dubious maybe needs to be actual end
                        .flat_map(move |y| (x_start..x_end).map(move |x| ((x, y), face))),
                )
            })
            .flatten()
            .collect();

        let sides = grid_cells
            .into_iter()
            .enumerate()
            .filter_map(|(count, cell)| {
                // - 1 2
                // - 4 -
                // 6 7 -
                // 9
                //
                // - T R
                // - F -
                // L D -
                // B
                let face = grid_index_to_face(count)?;
                Some((face, Side { face, inner: cell }))
            })
            .collect();

        Ok(Self {
            inner: map.inner,
            starting_point,
            face_map,
            sides,
        })
    }
}

#[test]
fn test_parsing_into_cube() {
    let input = r#" ..
 .
..
."#;
    let cube: Cube = input.parse().unwrap();
}

#[derive(Debug)]
pub enum Instruction {
    Move(usize),
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Tile {
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

pub type Coord = (usize, usize);

pub struct Map {
    pub inner: Vec<Vec<Tile>>,
    pub x_max: usize,
    pub y_max: usize,
    pub starting_point: Coord,
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
pub enum Facing {
    Up,
    #[default]
    Right,
    Left,
    Down,
}

impl Facing {
    fn opposite(&self) -> Self {
        match self {
            Facing::Up => Facing::Down,
            Facing::Right => Facing::Left,
            Facing::Left => Facing::Right,
            Facing::Down => Facing::Up,
        }
    }
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
pub struct Cursor {
    pub facing: Facing,
    pub position: Coord,
    pub history: Vec<(Coord, Facing)>,
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
    pub fn score(&self) -> usize {
        (self.position.1 + 1) * 1000 + (self.position.0 + 1) * 4usize + usize::from(self.facing)
    }

    pub fn apply_instruction(&mut self, ins: &Instruction, map: &impl Mappable) {
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

// impl Display for Map {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for y in 0..self.y_max {
//             for x in 0..self.x_max {
//                 let cell = if let Some(items) = self.inner.get(&(x, y)) {
//                     if items.len() == 1 {
//                         match items.first().unwrap() {
//                             Tile::Blizzard(direction) => (*direction).into(),
//                             Tile::Wall => "#".into(),
//                         }
//                     } else {
//                         items.len().to_string()
//                     }
//                 } else {
//                     ".".into()
//                 };
//
//                 write!(f, "{cell}")?;
//             }
//             writeln!(f)?;
//         }
//         Ok(())
//     }
// }

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
fn weird_part1_works() {
    let input = include_str!("../input.txt");
    let (map, _) = input.split_once("\n\n").unwrap();
    let map = map.parse::<Map>().unwrap();

    // assert_eq!(map.next_pos((117, 49), Facing::Down), Some((117, 0)));
    assert_eq!(map.next_pos((125, 0), Facing::Up), Some((125, 49)));
}

#[test]
fn rem_euclid_works_they_way_i_think() {
    assert_eq!((1isize - 2).rem_euclid(4), 3)
}

#[test]
fn cube_move_facing_works() {
    // - T R
    // - F -
    // L D -
    // B
    let input = r#" ..
 .
..
."#;
    let cube: Cube = input.parse().unwrap();

    let mut cursor = CubeCursor {
        facing: Facing::Right,
        coord: (1,0),
        history: vec![],
    };

    cursor.apply_instruction(&Instruction::Move(1), &cube);
    assert_eq!(cursor.facing, Facing::Right);
    assert_eq!(cursor.coord, (2, 0));

    cursor.apply_instruction(&Instruction::Move(1), &cube);
    assert_eq!(cursor.facing, Facing::Left);
    assert_eq!(cursor.coord, (1, 2));
}
